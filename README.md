# Hdu

A lightweight cross-platform disk usage visualizer for Windows, Linux, and macOS. Inspired by WizTree, but CLI-based.

```
hdu              # interactive TUI (default, scans current directory)
hdu scan .       # one-shot sorted scan
hdu tree /home   # directory tree
hdu watch /var   # live-updating TUI
```

## Features

- **Interactive TUI** with real-time sorting, filtering, and navigation
- **Sort by** size, name, or item count (ascending/descending)
- **Search** files/directories by name (`/` to start typing)
- **Visual bars** showing each entry's percentage of parent
- **Navigate** into directories with `Enter`, back with `Backspace`
- **Color themes** dark/light (`T` to toggle) — 10 themes total
- **Auto-refresh** at configurable interval
- **Config file** at `~/.config/hdu/config.toml`
- **Cross-platform** Linux, Windows, macOS

## Supported platforms

| OS     | Status |
|--------|--------|
| Linux  | Full support |
| macOS  | Full support |
| Windows | Full support |

## Quick start

### Requirements
- Rust 1.56+ ([install](https://rustup.rs/))

### Install

**Linux / macOS:**
```bash
git clone https://github.com/person134/hdu.git
cd hdu
sudo ./install-uninstall/install.sh
```

**Windows (Run as Administrator):**
```batch
git clone https://github.com/person134/hdu.git
cd hdu
install-uninstall\install.bat
```

The binary will be at `target/release/hdu` (or `hdu.exe` on Windows).

### Uninstall

**Linux / macOS:**
```bash
sudo ./install-uninstall/install.sh --uninstall
```

**Windows (Run as Administrator):**
```batch
install-uninstall\install.bat --uninstall
```

## Usage

### TUI mode (default)
```
hdu
```

Once inside the TUI:

| Key | Action |
|-----|--------|
| `↑`/`↓` or `k`/`j` | Select entry |
| `PgUp`/`PgDn` | Page up/down |
| `Home`/`End` | First/last entry |
| `s` | Cycle sort field |
| `S` | Toggle sort order |
| `/` | Enter search mode |
| `Enter` or `→` | Enter directory |
| `Backspace` or `←` | Go up |
| `d` | View entry details |
| `g` | Go to root (current directory) |
| `r` | Rescan current directory |
| `T` | Cycle themes |
| `+`/`-` | Increase/decrease refresh rate |
| `q` or `Esc` | Quit |

### Scan mode
```
hdu scan [path]
```
Prints top 60 entries sorted by size with SIZE, ITEMS, NAME, %.

### Tree mode
```
hdu tree [path]
```
Prints full directory tree with sizes, sorted by size descending.

### Watch mode
```
hdu watch [path]
```
Live-updating TUI (same as default, alias for convenience).

### Options
```
hdu -r 500              # 500ms refresh rate
hdu watch /home -r 2000 # watch mode at 2s interval
hdu --help
hdu --version
```

### Config file
`~/.config/hdu/config.toml` is auto-created with defaults:
```toml
[settings]
refresh_rate = 1000
sort_by = "size"
sort_order = "desc"
theme = "dark"
```

## Development

```bash
cargo build              # debug build
cargo build --release    # release build
cargo test               # run tests
cargo clippy             # lint
cargo fmt --check        # check formatting
```

### Project structure
```
src/
  action.rs    - CLI argument parsing
  backend.rs   - Platform detection
  config.rs    - Configuration file
  scanner.rs   - Directory scanning and data model
  system.rs    - Disk mount info
  ui.rs        - TUI rendering and event handling
  main.rs      - Entry point and dispatch
tests/
examples/
```

## License

MIT. See [LICENSE](LICENSE)
