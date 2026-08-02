# archive-drive

Fast, offline, searchable HTML catalogs for cold storage drives.

Point `archive-drive` at a folder or drive and it produces a single
self-contained `.html` file — no server, no database, just double-click and
browse. Built for cataloging drives you don't keep connected all the time:
scan once, disconnect the drive, and still know exactly what's on it.

![Catppuccin Macchiato](https://img.shields.io/badge/theme-Catppuccin%20Macchiato-c6a0f6)
![License: GPL v3](https://img.shields.io/badge/license-GPLv3-blue.svg)

## Features

- **Fast, parallel scanning** via [`jwalk`](https://crates.io/crates/jwalk) —
  hundreds of thousands of files in seconds on real hardware, not hours.
- **One self-contained HTML file per drive.** All data is embedded — the
  catalog works forever, offline, on any machine, with no dependencies.
- **Real explorer-style browsing** — breadcrumb navigation, a sortable table
  (name / type / items / size), and a toggleable sidebar tree — not just a
  flat file listing.
- **Search** across both files and folders, jumping straight to wherever a
  match lives.
- **Catppuccin Macchiato theme**, mauve accent.
- **Portable output** — the catalog shows just the drive's folder name
  (e.g. `/data`), not the full absolute path, since that won't mean
  anything once the file moves to another machine.
- Static binaries for **amd64, arm64, and riscv64** — no distro-specific
  packaging, works the same on NixOS, Debian, Fedora, Arch, or anywhere
  else on Linux.

## Install

Grab a static binary from the
[Releases](https://github.com/mmBesar/archive-drive/releases) page for your
architecture, or build from source:

```sh
git clone https://github.com/mmBesar/archive-drive.git
cd archive-drive
cargo build --release
# binary at target/release/archive-drive
```

### Building with Nix

A `flake.nix` devShell is included for a reproducible toolchain:

```sh
nix develop
cargo build --release
```

## Usage

```sh
archive-drive /path/to/drive --html catalog.html
```

Open `catalog.html` in any browser. That's it — the file is fully
self-contained.

### Options

| Flag | Description |
|---|---|
| `--html <path>` | Write the searchable HTML catalog to this file |
| `--show-tree` | Also print the recursive folder tree in the terminal |
| `--tree-depth <n>` | Depth for `--show-tree` (default: 2) |

Terminal output without `--html` just prints a scan summary — file count,
total size, a top-10 file-type breakdown, and elapsed time.

## Design notes

- **No file hashing / integrity checking in v1, on purpose.** The point of
  this tool is a fast catalog, not a bit-rot verification pass — full
  hashing means reading every byte off the drive, which is inherently slow
  and disk-bound regardless of implementation. That may come later as a
  clearly separate, opt-in `verify` mode — never part of the default scan.
- **Metadata-only scanning is the default and only mode** — a 5TB drive with
  a few hundred thousand files catalogs in low single-digit seconds on
  reasonable hardware.

## License

GPLv3 — see [LICENSE](LICENSE). Free to use, study, modify, and
redistribute; modified or redistributed versions must remain open under the
same license.
