# TaskFlowTUI

A practical, keyboard-driven task manager built with Rust and Ratatui.

**Repository:** https://github.com/kakzctl/taskflowtui

## Install

### From crates.io

After the package is published:

```sh
cargo install taskflowtui
```

### Linux packages

Download a matching `.deb` or `.rpm` package from GitHub Releases. Arch Linux users can build with `packaging/arch/PKGBUILD`.

### Prebuilt binaries

Download the archive for your operating system and architecture from GitHub Releases, extract it, and place the `taskflowtui` binary somewhere on your `PATH`.

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
