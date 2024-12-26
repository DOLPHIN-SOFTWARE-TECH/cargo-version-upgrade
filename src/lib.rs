use semver::{Prerelease, Version};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;
use toml_edit::{DocumentMut, Item, Value};

#[derive(Debug)]
pub enum VersionUpgradeError {
    InvalidVersionFormat,
    InvalidIncrement,
    FileReadError(std::io::Error),
    FileWriteError(std::io::Error),
    InvalidPrereleaseTag,
    GitCommitError,
    GitTagError,
}

impl fmt::Display for VersionUpgradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionUpgradeError::InvalidVersionFormat => write!(f, "Invalid version format"),
            VersionUpgradeError::InvalidIncrement => write!(f, "Invalid increment type"),
            VersionUpgradeError::FileReadError(err) => write!(f, "Error reading file: {}", err),
            VersionUpgradeError::FileWriteError(err) => write!(f, "Error writing file: {}", err),
            VersionUpgradeError::InvalidPrereleaseTag => write!(f, "Invalid prerelease tag format"),
            VersionUpgradeError::GitCommitError => write!(f, "Failed to create Git commit"),
            VersionUpgradeError::GitTagError => write!(f, "Failed to create Git tag"),
        }
    }
}

impl Error for VersionUpgradeError {}

pub fn update_version_and_commit(
    cargo_toml_path: &Path,
    increment: &str,
    tag: Option<&str>,
) -> Result<(), VersionUpgradeError> {
    // Read and update Cargo.toml
    let content =
        fs::read_to_string(cargo_toml_path).map_err(VersionUpgradeError::FileReadError)?;
    let mut doc: DocumentMut = content
        .parse()
        .map_err(|_| VersionUpgradeError::InvalidVersionFormat)?;

    let version_str = doc["package"]["version"]
        .as_str()
        .ok_or(VersionUpgradeError::InvalidVersionFormat)?;
    let version =
        Version::parse(version_str).map_err(|_| VersionUpgradeError::InvalidVersionFormat)?;
    let suffix = &version.pre.clone();

    let incremented_version = match increment {
        "patch" => increment_version(version, IncrementType::Patch, tag, suffix),
        "minor" => increment_version(version, IncrementType::Minor, tag, suffix),
        "major" => increment_version(version, IncrementType::Major, tag, suffix),
        "pre" => increment_version(version, IncrementType::Pre, tag, suffix),
        _ => return Err(VersionUpgradeError::InvalidIncrement),
    }?;

    doc["package"]["version"] = Item::from(Value::from(incremented_version.to_string()));
    fs::write(cargo_toml_path, doc.to_string()).map_err(VersionUpgradeError::FileWriteError)?;

    // Commit the changes and create a Git tag
    commit_and_tag(&incremented_version)?;
    Ok(())
}

#[derive(Debug)]
enum IncrementType {
    Patch,
    Minor,
    Major,
    Pre,
}

fn increment_version(
    mut version: Version,
    increment_type: IncrementType,
    tag: Option<&str>,
    suffix: &Prerelease,
) -> Result<Version, VersionUpgradeError> {
    /*
     * patch (0.0.4 -> 0.0.5)
     * patch --tags <name> (0.0.4 -> 0.0.5-tagname.0 , 0.0.5-tagname.0 -> 0.0.6-tagname.0)
     * minor (0.0.4 -> 0.1.4)
     * minor --tags <name> (0.0.4 -> 0.1.0-tagname.0 , 0.0.4-tagname.0 -> 0.1.0-tagname.0)
     * major (0.0.4 -> 0.0.5)
     * major --tags <name> (0.0.4 -> 1.0.0-tagname.0 , 0.0.4-tagname.0 -> 1.0.0-tagname.0)
     * pre (0.0.4 -> err msg, 0.0.4-tagname.0 -> 0.0.4-tagname.1, 0.4.4-tagname.4 -> 0.4.4-tagname.5)
     */
    fn set_prerelease(tag: &Option<&str>, version: &mut Version) {
        if let Some(tag) = tag {
            let pr = format!("{}.0", tag);
            version.pre = Prerelease::new(&pr).unwrap();
        } else {
            version.pre = Prerelease::EMPTY;
        }
    }

    match increment_type {
        IncrementType::Patch => {
            version.patch += 1;
            set_prerelease(&tag, &mut version);
        }
        IncrementType::Minor => {
            version.minor += 1;
            version.patch = 0;
            set_prerelease(&tag, &mut version);
        }
        IncrementType::Major => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            set_prerelease(&tag, &mut version);
        }
        IncrementType::Pre => {
            let mut prerelease = suffix.as_str().to_string();
            let parts: Vec<_> = prerelease.split('.').collect();
            let mut parts2 = parts.clone();
            if let Some(last) = parts.last() {
                parts2.pop();
                let tag_ref = parts2.join(".");
                prerelease = match last.parse::<u32>() {
                    Ok(num) => format!("{}.{}", tag_ref, num + 1),
                    Err(_) => return Err(VersionUpgradeError::InvalidPrereleaseTag),
                };
            }
            version.pre = Prerelease::new(&prerelease).unwrap();
        }
    }
    Ok(version)
}

fn commit_and_tag(version: &Version) -> Result<(), VersionUpgradeError> {
    let version_str = version.to_string();

    // Commit the changes
    let commit_message = format!("v{}", version_str);
    let commit_output = Command::new("git")
        .arg("commit")
        .arg("-am")
        .arg(&commit_message)
        .output()
        .map_err(|_| VersionUpgradeError::GitCommitError)?;

    if !commit_output.status.success() {
        return Err(VersionUpgradeError::GitCommitError);
    }

    // Create the git tag
    let tag_output = Command::new("git")
        .arg("tag")
        .arg(&version_str)
        .output()
        .map_err(|_| VersionUpgradeError::GitTagError)?;

    if !tag_output.status.success() {
        return Err(VersionUpgradeError::GitTagError);
    }

    println!("Git commit and tag '{}' created successfully", version_str);
    Ok(())
}
