
# CLI Toolkit Cheat Sheet

Quick reference for the CLI tools we discussed. Install status can vary;
this is a usage guide, not a verified inventory.

## Files and search

### `uutils` / `uu-*`

Rust implementations of common Unix utilities; use them as alternatives
to familiar coreutils commands where compatible. - `uu-ls` -
`uu-cat file.txt` - Check compatibility before relying on them in
scripts.

### `bat`

A syntax-highlighting file viewer; use it when reading source code or
config files, while plain `cat` is better for raw output and
pipelines. - `bat src/main.rs` - `bat --paging=never Cargo.toml` -
Optional Bash alias: `alias cat='bat --paging=never'`

### `eza`

A modern `ls` alternative with icons, detailed listings, Git status, and
tree views. - `eza --icons=auto` - `eza -l --git` - `eza --all` -
`eza --tree --level=2`

### `fd`

A fast, convenient file-name finder; use it to locate files by name or
path. - `fd Cargo.toml .` - `fd '\.rs$' src` - `fd --hidden config ~`

### `ripgrep` (`rg`)

A fast recursive text search that respects ignore files; use it to find
symbols, TODOs, and strings in code. - `rg "TODO|FIXME"` -
`rg -n "unwrap" src` - `rg "fn " -t rs` - `rg --files`

### `ripgrep-all` (`rga`)

Extends ripgrep to search text extracted from supported formats such as
PDFs and office documents; use it when `rg` cannot search inside a
document. - `rga "search phrase" ~/Documents` -
`rga "search phrase" -g '*.pdf' ~/Documents` - `rga --rga-list-adapters`

### `fselect`

A SQL-like file metadata query tool; use it when you need richer
filtering or reports than a simple filename search. -
`fselect name,size from .` -
`fselect name,size from . where size > 100mb` - Run `fselect --help` for
the installed version's query syntax.

## Disk usage and cleanup

### `dust`

A visual, tree-style disk-usage viewer; use it for a quick overview of
which directories are largest. - `dust ~` - `dust -d 3` - `dust -n 20`

### `dua`

A fast disk-usage analyzer with an interactive browser; use `dua i` to
explore space usage and optionally remove selected files. -
`dua --help` - `dua i ~` - Review carefully before deleting anything.

### `kondo`

Finds generated build artifacts that can be cleaned from supported
projects; use it to reclaim space from build outputs such as Rust
`target` directories. - `kondo --help` - Run it from a project/workspace
and review its proposed cleanup before confirming.

## Rust development

### `cargo info`

Cargo's package-information command; use it to inspect a crate before
adding it to a project. - `cargo info serde` -
`cargo info serde@1.0.0` - `cargo info --help`

### `mprocs`

Runs several commands in a terminal UI with separate output panes; use
it to run a dev server, tests, and logs together. -
`mprocs "cargo check" "cargo test"` - `mprocs --help`

### `tokei`

Counts code, comments, and blank lines by language; use it to get a
quick overview of a project's codebase. - `tokei` - `tokei src` -
`tokei --help`

### `just`

A command runner using a project `justfile`; use it to give common tasks
short names such as `check`, `test`, `lint`, and `fmt`. -
`just --list` - `just check` - `just test` - `just lint` - `just fmt`

### `hyperfine`

Benchmarks commands over repeated runs; use it to compare real workloads
more reliably than a single `time` measurement. -
`hyperfine 'fd Cargo.toml .' 'find . -name Cargo.toml'` -
`hyperfine --warmup 3 'cargo test'` - `hyperfine --help`

### `rusty-man` --- install failed

An older Rust documentation viewer whose install failed because its
dependency requires yanked `ahash` versions; don't spend more time on it
unless the project is updated. - Use Neovim hover docs with `K`. -
`cargo doc --open` - `rustup doc --std`

## Git

### `delta`

A syntax-highlighted pager for Git diffs; use it to review changes and
commits more comfortably. - `git diff` - `git log -p -1` - `git show` -
Configure: `git config --global core.pager delta`

### `gitui`

An interactive Git terminal UI; use it to stage changes, inspect diffs,
and manage common Git operations. - Run `gitui` inside a repository. -
Use the UI's help/keybinding hints before destructive actions.

## HTTP and network

### `xh`

A friendly terminal HTTP client; use it to test APIs and inspect JSON
responses without building long `curl` commands. -
`xh https://httpbin.org/get` - `xh GET https://httpbin.org/get` -
`xh https://httpbin.org/post name=test`

### `speedtest-rs` --- optional / not confirmed

An unofficial Rust speed-test client; use it only if you specifically
want that implementation and have checked its limitations. -
`speedtest-rs --help` - `speedtest-rs`\

- For dependable results on a fast connection, consider the official
Ookla CLI instead.

## Terminal apps and shell

### `ncspot`

A keyboard-driven Spotify client for the terminal; use it to control
Spotify without leaving the terminal. - `ncspot` - `ncspot --help`

### `presenterm`

A Markdown slide presenter; use it to give technical talks or walk
through code in the terminal. - `presenterm slides.md` -
`presenterm --help`

### `wiki-tui`

A terminal Wikipedia browser; use it for quick reference lookups, but
note that the upstream project was reported as archived. -
`wiki-tui --help`

### `fish`

An alternative interactive shell with autosuggestions and syntax
highlighting; try it interactively without replacing Bash until you
decide to migrate. - `fish` - `exit` (return to the previous shell) -
Fish config: `~/.config/fish/config.fish`

## Quick notes

- `rg` searches **inside files**; `fd` finds **files by name**.
- `dust` and `dua` analyze disk usage; `kondo` targets generated
    project artifacts for cleanup.
- `delta` improves Git diff display; `gitui` provides an interactive
    Git workflow.
- `just` runs named project tasks; it doesn't replace Cargo.
- `cargo info` is built into modern Cargo; `rusty-man` is not needed
    for Neovim docs.
- `mask` overlaps with `just` by defining project tasks in Markdown;
    try one task runner first.
