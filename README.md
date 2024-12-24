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