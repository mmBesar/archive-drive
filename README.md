<div align="center">

# ◆ archive-drive

**Fast, offline, searchable HTML catalogs for cold storage drives.**

Point it at a folder or drive. Get back a single self-contained `.html`
file you can browse like a real file explorer — breadcrumbs, sortable
columns, search — forever, offline, with no server and no database.

[![License: GPLv3](https://img.shields.io/github/license/mmBesar/archive-drive)](LICENSE)
[![Release](https://github.com/mmBesar/archive-drive/actions/workflows/release.yml/badge.svg)](https://github.com/mmBesar/archive-drive/actions/workflows/release.yml)
[![Latest release](https://img.shields.io/github/v/release/mmBesar/archive-drive)](https://github.com/mmBesar/archive-drive/releases)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Platforms](https://img.shields.io/badge/platforms-amd64%20%7C%20arm64%20%7C%20riscv64-informational)](#install)
[![Theme: Catppuccin Macchiato](https://img.shields.io/badge/theme-Catppuccin%20Macchiato-c6a0f6)](https://github.com/catppuccin/catppuccin)

</div>

---

## Why

Cold storage drives don't stay plugged in. `archive-drive` scans one once,
writes a single portable HTML file, and from then on you always know
what's on that drive — searchable, browsable, no need to ever mount it
again just to check.

- **Fast, parallel scanning** via [`jwalk`](https://crates.io/crates/jwalk)
  — hundreds of thousands of files in seconds on real hardware, not hours.
- **One self-contained HTML file per drive.** Everything's embedded — the
  catalog works forever, offline, on any machine, with zero dependencies.
- **Real explorer-style browsing** — breadcrumb navigation, a sortable
  table (name / type / items / size), and a toggleable sidebar tree.
- **Search** across both files and folders, jumping straight to wherever
  a match lives.
- **Catppuccin Macchiato theme**, mauve accent.
- **Portable output** — shows just the drive's folder name (e.g. `/data`),
  never the full absolute path, since that won't mean anything once the
  file moves to another machine.
- **Static binaries for amd64, arm64, and riscv64** — no distro-specific
  packaging, same binary works on NixOS, Debian, Fedora, Arch, or anywhere
  else on Linux.

## Install

Grab a static binary from the [Releases](https://github.com/mmBesar/archive-drive/releases)
page for your architecture, or build from source:

```sh
git clone https://github.com/mmBesar/archive-drive.git
cd archive-drive
cargo build --release
# binary at target/release/archive-drive
```

<details>
<summary>Building with Nix</summary>

A `flake.nix` devShell is included for a reproducible toolchain:

```sh
nix develop
cargo build --release
```

</details>

## Usage

The defaults are the whole point — point it at something and go:

```sh
archive-drive /mnt/data
```

That's it. It writes **`data.html`** into your current directory —
the name comes straight from the folder you scanned — and it's ready to
open in any browser.

```sh
archive-drive              # no args: prints help
```

### Options

| Flag | Default | Description |
|---|---|---|
| `<path>` | — | Directory or drive to scan (required) |
| `-o, --output <path>` | — | Write the catalog to this exact path (overrides the two below) |
| `--output-dir <dir>` | `.` | Directory to write the HTML catalog into |
| `--output-name <name>` | `<folder-name>.html` | Filename for the catalog |
| `--show-tree` | off | Also print the recursive folder tree in the terminal |
| `--tree-depth <n>` | `2` | Depth for `--show-tree` |

### Examples

```sh
# Default: writes ./data.html
archive-drive /mnt/data

# Quick one-shot output path
archive-drive /mnt/data -o ~/catalogs/data.html

# Or split it: choose where it lands
archive-drive /mnt/data --output-dir ~/catalogs

# ...and/or choose the filename too
archive-drive /mnt/data --output-dir ~/catalogs --output-name backup-drive-2026.html

# Peek at the folder structure in the terminal while you're at it
archive-drive /mnt/data --show-tree --tree-depth 3
```

Terminal output always includes a quick scan summary regardless of
flags — file count, total size, a top-10 file-type breakdown, and how
long the scan took.

## Design notes

- **No file hashing / integrity checking in v1, on purpose.** The point
  of this tool is a fast catalog, not a bit-rot verification pass — full
  hashing means reading every byte off the drive, which is inherently
  slow and disk-bound no matter the implementation. That may come later
  as a clearly separate, opt-in `verify` mode — never part of the default
  scan.
- **Metadata-only scanning is the default and only mode** — a 5TB drive
  with a few hundred thousand files catalogs in low single-digit seconds
  on reasonable hardware.

## License

GPLv3 — see [LICENSE](LICENSE). Free to use, study, modify, and
redistribute; modified or redistributed versions must remain open under
the same license.
