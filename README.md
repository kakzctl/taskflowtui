# TaskFlowTUI

A practical, keyboard-driven task manager built with Rust and Ratatui.

**Repository:** https://github.com/kakzctl/taskflowtui

## Install

### From crates.io

After the package is published:

```sh
cargo install taskflowtui
```

### Debian and Ubuntu

Download the `.deb` package from [GitHub Releases](https://github.com/kakzctl/taskflowtui/releases), then install it with your system package manager.

### Fedora, RHEL and openSUSE

Download the `.rpm` package from [GitHub Releases](https://github.com/kakzctl/taskflowtui/releases), then install it with your system package manager.

### Arch Linux

The repository includes a PKGBUILD:

```sh
cd packaging/arch
makepkg -si
```

Update `pkgver` in the PKGBUILD and regenerate `.SRCINFO` when preparing a new version for an AUR submission.

### Prebuilt binaries

Download the archive for your operating system and architecture from GitHub Releases, extract it, and place the `taskflowtui` binary somewhere on your `PATH`.

## Publishing to crates.io

The repository includes a manually triggered GitHub Actions workflow named **Publish to crates.io**. Before using it:

1. Create a crates.io API token.
2. Add it to the GitHub repository as an Actions secret named `CARGO_REGISTRY_TOKEN`.
3. Run the workflow from the Actions tab and explicitly confirm publication.

The workflow runs `cargo publish --dry-run` before publishing. A published crate version cannot be reused, so update the package version in `Cargo.toml` before each release.

## Build from source

Requirements: stable Rust and Cargo.

```sh
git clone https://github.com/kakzctl/taskflowtui.git
cd taskflowtui
cargo run
```

## Product vision

TaskFlow is a local-first terminal application for capturing, organizing and tracking everyday tasks. It is designed to work offline and store data locally in SQLite.

## License

TaskFlowTUI is dual-licensed under either the MIT License (`LICENSE-MIT`) or Apache License 2.0 (`LICENSE-APACHE`). You may choose either license.
