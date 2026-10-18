# memo-cho

A small command-line memo tool written in Rust. Memos are plain Markdown files in a directory of your choice, opened with your favorite editor and picked with a fuzzy finder.

## Features

| Command | Alias | Description |
|---------|-------|-------------|
| `memo-cho new` | `n` | Ask for a title, create `YYYY-MM-DD-<title>.md` from a template, and open it in your editor |
| `memo-cho edit` | `e` | Pick an existing memo with a fuzzy finder and open it in your editor |
| `memo-cho delete` | `d` | Pick an existing memo with a fuzzy finder and delete it |
| `memo-cho daily-note` | `dn` | Create (or reopen) today's `YYYY-MM-DD.md` |

### In the future

The following commands are already declared in the CLI but are **not implemented yet**. They are planned for future releases.

| Command | Status | Planned description |
|---------|--------|---------------------|
| `memo-cho list` | Planned | List all memos |
| `memo-cho grep <pattern>` | Planned | Search memo contents for a pattern |
| `memo-cho serve` | Planned | Serve memos as a web page |

## Requirements

| Tool | Why | Notes |
|------|-----|-------|
| Rust toolchain (stable) | Building | Install via [rustup](https://rustup.rs) |
| An editor (`nvim`, `vim`, `nano`, ...) | Editing memos | Configurable |
| [fzf](https://github.com/junegunn/fzf) | Selecting a memo in `edit` / `delete` | Configurable (`cmdselector`) |
| `sh`, `ls`, `xargs` | Used internally by `edit` / `delete` | Preinstalled on macOS |

Supported platform: **macOS**. Linux and Windows support is planned (Windows will need the shell-based parts to be replaced).

## Installation

### Option A: install directly from GitHub

```sh
# Prerequisites (macOS, Homebrew)
brew install rustup fzf neovim
rustup-init -y

cargo install --git https://github.com/jacoloves/memo-cho
```

### Option B: clone and build

```sh
git clone https://github.com/jacoloves/memo-cho.git
cd memo-cho

# Install to ~/.cargo/bin
cargo install --path .

# ...or just build and run from the source tree
cargo build --release
./target/release/memo-cho --help
```

Make sure `~/.cargo/bin` is on your `PATH`:

```sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
```

### Updating

```sh
# Option A
cargo install --git https://github.com/jacoloves/memo-cho --force

# Option B
git pull && cargo install --path . --force
```

## Configuration

The config file lives at `~/.config/memo-cho/config.yaml`. If it does not exist, a default one is generated on first run.

```yaml
memodir: $HOME/memo-cho                          # where memos are stored
template: $HOME/memo-cho/template/template.md    # template for `new`
editor: nvim                                     # editor command
cmdselector: "fzf"                               # fuzzy finder command
```

- `$HOME` is expanded to your home directory.
- **`memodir` must already exist** — create it first (`mkdir -p ~/memo-cho`). The auto-generated default points `memodir` at your home directory, so edit it after the first run.
- The sample `config.yaml` in this repository (which uses `$HOME/tmp/memo-cho`) can be copied as a starting point. Adjust `memodir` / `template` if you prefer another location, e.g. `$HOME/memo-cho`:

  ```sh
  mkdir -p ~/.config/memo-cho ~/tmp/memo-cho/template
  cp config.yaml ~/.config/memo-cho/config.yaml
  cp template.md ~/tmp/memo-cho/template/template.md
  ```

### Template

`{{ title }}` and `{{ date }}` are replaced when a memo is created. If the template file is missing or empty, a memo starting with `# ` is created.

```markdown
# {{ title }}

**Date:** {{ date }}
```

## Syncing memos across machines

memo-cho only reads and writes files in `memodir`, so any file-sync mechanism works. Keep the **config** local to each machine (editor and paths may differ) and sync the **memos**.

| Method | Pros | Cons |
|--------|------|------|
| Git repository (private) | History, works everywhere, explicit sync | Manual `git pull` / `git push` |
| iCloud Drive / Dropbox | Automatic | No history by default; conflicts are possible; path differs per OS |

### Recommended: a private Git repository

On the first machine:

```sh
cd ~/memo-cho
git init
git remote add origin git@github.com:<you>/<your-private-memo-repo>.git
git add . && git commit -m "init" && git push -u origin main
```

On every other machine:

```sh
git clone git@github.com:<you>/<your-private-memo-repo>.git ~/memo-cho
# then set `memodir: $HOME/memo-cho` in ~/.config/memo-cho/config.yaml
```

Day to day: `git pull` before writing, `git add . && git commit && git push` afterwards. Because `new` prefixes file names with the date, two machines rarely create the same file name; `daily-note` is the one place where conflicts can occur if you edit the same day on both machines without pulling first.

Keeping `template.md` inside the synced `memodir` (e.g. `memodir/template/template.md`) means templates are shared too.

### Using a cloud folder

Point `memodir` at the synced folder, e.g. `memodir: $HOME/Library/Mobile Documents/com~apple~CloudDocs/memo-cho` or `$HOME/Dropbox/memo-cho`. Note that `edit` / `delete` build a shell command from this path, so **paths containing spaces are not supported yet** (the iCloud path above contains none, but `Mobile Documents` does — prefer a symlink such as `~/memo-cho` → the synced folder).

## Development

```sh
cargo build
cargo test   # note: the current test reads your real ~/.config/memo-cho/config.yaml
```

## Known limitations

- The loaded config is printed to stdout on every run (debug output).
- `list`, `grep` and `serve` are declared but not implemented.
- `edit` / `delete` rely on `sh`, `ls` and `xargs`, so they do not work on Windows.
- Paths with spaces in `memodir` are not supported by `edit` / `delete`.
