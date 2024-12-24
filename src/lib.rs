use std::fmt;
use std::fs;
use std::path::{Path};
use semver::{Version, Prerelease};
use toml_edit::{DocumentMut, Item, Value};
use std::error::Error;

#[derive(Debug)]
pub enum VersionUpgradeError {
    InvalidVersionFormat,
    InvalidIncrement,
    FileReadError(std::io::Error),
    FileWriteError(std::io::Error),
    InvalidPrereleaseTag,
}

impl fmt::Display for VersionUpgradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionUpgradeError::InvalidVersionFormat => write!(f, "Invalid version format"),
            VersionUpgradeError::InvalidIncrement => write!(f, "Invalid increment type"),
            VersionUpgradeError::FileReadError(err) => write!(f, "Error reading file: {}", err),
            VersionUpgradeError::FileWriteError(err) => write!(f, "Error writing file: {}", err),
            VersionUpgradeError::InvalidPrereleaseTag => write!(f, "Invalid prerelease tag format"),
        }
    }
}

impl Error for VersionUpgradeError {}

pub fn update_version(cargo_toml_path: &Path, increment: &str, tag: Option<&str>) -> Result<(), VersionUpgradeError> {
    let content = fs::read_to_string(cargo_toml_path).map_err(VersionUpgradeError::FileReadError)?;

    let mut doc: DocumentMut = content.parse().map_err(|_| VersionUpgradeError::InvalidVersionFormat)?;

    let version_str = doc["package"]["version"].as_str().ok_or(VersionUpgradeError::InvalidVersionFormat)?;
    let version = Version::parse(version_str).map_err(|_| VersionUpgradeError::InvalidVersionFormat)?;

    let incremented_version = match increment {
        "patch" => increment_version(version, IncrementType::Patch, tag),
        "minor" => increment_version(version, IncrementType::Minor, tag),
        "major" => increment_version(version, IncrementType::Major, tag),
        _ => return Err(VersionUpgradeError::InvalidIncrement),
    }?;

    doc["package"]["version"] = Item::from(Value::from(incremented_version.to_string()));

    fs::write(cargo_toml_path, doc.to_string()).map_err(VersionUpgradeError::FileWriteError)?;
    Ok(())
}

#[derive(Debug)]
enum IncrementType {
    Patch,
    Minor,
    Major,
}

fn increment_version(
    mut version: Version,
    increment_type: IncrementType,
    tag: Option<&str>,
) -> Result<Version, VersionUpgradeError> {
    match increment_type {
        IncrementType::Patch => version.patch += 1,
        IncrementType::Minor => {
            version.minor += 1;
            version.patch = 0; // Reset patch on minor version increment
        }
        IncrementType::Major => {
            version.major += 1;
            version.minor = 0; // Reset minor and patch on major version increment
            version.patch = 0;
        }
    }

    if let Some(tag) = tag {
        let prerelease = Prerelease::new(tag).map_err(|_| VersionUpgradeError::InvalidPrereleaseTag)?;
        version.pre = prerelease;
    }

    Ok(version)
}
