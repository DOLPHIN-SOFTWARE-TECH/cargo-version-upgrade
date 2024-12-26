# cargo-version-upgrade

`cargo-version-upgrade` is a Rust library designed for managing semantic versioning in Rust projects. It provides an easy-to-use CLI to update versions in your `Cargo.toml` file based on semantic versioning rules.

---

## Features
- **Patch Updates**: Increment the patch version (e.g., `0.0.1` → `0.0.2`).
- **Minor Updates**: Increment the minor version and reset the patch version (e.g., `0.0.5` → `0.1.0`).
- **Major Updates**: Increment the major version and reset the minor and patch versions (e.g., `1.0.4` → `2.0.0`).
- **Pre-release Tags**: Add tags like `beta`, `alpha`, etc., to any version update.

---

## Installation

To install the library, use the following command:

```bash
cargo install cargo-version-upgrade
```

## Usage

The CLI provides several commands to manage versions. Below is an overview of the available commands and their effects:

### Basic Commands

1. **Patch**:  
   Increment the patch version.  
   ```bash
   cargo-version-upgrade patch
   ```  
   Example: `0.0.4 → 0.0.5`

2. **Minor**:  
   Increment the minor version and reset the patch version.  
   ```bash
   cargo-version-upgrade minor
   ```  
   Example: `0.0.4 → 0.1.0`

3. **Major**:  
   Increment the major version and reset the minor and patch versions.  
   ```bash
   cargo-version-upgrade major
   ```  
   Example: `0.0.4 → 1.0.0`

### Commands with Tags

1. **Patch with Tags**:  
   Increment the patch version and append a pre-release tag.  
   ```bash
   cargo-version-upgrade patch --tags <tagname>
   ```  
   Examples:
    - `0.0.4 → 0.0.5-tagname.0`
    - `0.0.5-tagname.0 → 0.0.6-tagname.0`

2. **Minor with Tags**:  
   Increment the minor version, reset the patch version, and append a pre-release tag.  
   ```bash
   cargo-version-upgrade minor --tags <tagname>
   ```  
   Examples:
    - `0.0.4 → 0.1.0-tagname.0`
    - `0.0.4-tagname.0 → 0.1.0-tagname.0`

3. **Major with Tags**:  
   Increment the major version, reset the minor and patch versions, and append a pre-release tag.  
   ```bash
   cargo-version-upgrade major --tags <tagname>
   ```  
   Examples:
    - `0.0.4 → 1.0.0-tagname.0`
    - `0.0.4-tagname.0 → 1.0.0-tagname.0`

### Pre-release Command

1. **Pre-release**:  
   Update the pre-release version for a specific tag.  
   ```bash
   cargo-version-upgrade pre
   ```  
   Examples:
    - `0.0.4 → Error: No tag present`
    - `0.0.4-tagname.0 → 0.0.4-tagname.1`
    - `0.4.4-tagname.4 → 0.4.4-tagname.5`