# codescope

A fast, extensible command-line tool for exploring codebases — print file contents, count lines of code, and find empty files or folders across any number of files and directories at once.

Built in Rust. Binary name: `cs`.

## Features

- **`cat`** — Print the contents of any file or folder, recursively, with clean relative paths as headers.
- **`loc`** — Count lines of code per file and get a total across all given paths.
- **`empty`** — Detect empty files and folders anywhere in the project.
- **Smart path resolution** — pass a full path *or* just a file/folder name (e.g. `main.rs` instead of `src/main.rs`); codescope searches the project for it automatically.
- **Multiple targets at once** — mix files and folders from different locations in a single command.
- Skips `target/`, `.git/`, and other hidden directories automatically.

## Installation

Clone the repository and build it with Cargo:

```bash
git clone https://github.com/<your-username>/codescope.git
cd codescope
cargo build --release
```

The compiled binary will be available at `target/release/cs`. To install it globally:

```bash
cargo install --path .
```

## Using it globally, across any project

codescope only works with the files and folders in your current directory — it doesn't care whether that's a Rust, Flutter, Node, or any other kind of project. Install it once and it's available everywhere.

Run this inside the `codescope` folder:

```bash
cargo install --path .
```

This installs the `cs` binary to `~/.cargo/bin`, which `rustup` normally adds to your `PATH` automatically. Verify it works from anywhere:

```bash
cs --help
```

If you get `command not found`, add this line to your `~/.zshrc` (or `~/.bashrc`) and restart your terminal:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

From then on, open any project in VS Code (or any terminal) and use it directly:

```bash
cs cat lib main.dart
cs loc lib
cs empty .
```

To update after making changes to codescope's source code:

```bash
cargo install --path . --force
```

## Usage

```
cs <command> <paths...>
```

### `cat` — print file contents

```bash
cs cat main.rs
```

```
src/main.rs:
{
fn main() {
    println!("Hello, world!");
}
}
```

Multiple targets, mixing files and folders, from anywhere in the project:

```bash
cs cat src main.rs empty.rs
```

Empty files are skipped automatically — no clutter in the output.

### `loc` — count lines of code

```bash
cs loc src/
```

```
src/main.rs: 12 lines
src/cli.rs: 28 lines
src/commands/cat.rs: 24 lines
src/commands/loc.rs: 21 lines
src/commands/empty.rs: 33 lines
Total: 118 lines
```

Works across multiple paths too:

```bash
cs loc commands core
```

### `empty` — find empty files and folders

```bash
cs empty .
```

```
[empty file] src/legacy/old.rs
[empty folder] src/drafts
```

## How path resolution works

You don't need to type full paths. codescope first checks if the input is a valid path relative to your current directory. If not, it searches the whole project for a file or folder with that exact name:

```bash
cs cat cli.rs        # works from anywhere in the project
cs loc loc.rs
cs empty commands
```

If more than one match is found, codescope lists all matches and asks you to be more specific — it never guesses silently.

## Project structure

```
codescope/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              # Entry point — parses CLI args and dispatches commands
    ├── cli.rs                # Clap definitions: Cli struct and Commands enum
    ├── commands/
    │   ├── mod.rs
    │   ├── cat.rs             # `cat` command implementation
    │   ├── loc.rs             # `loc` command implementation
    │   └── empty.rs           # `empty` command implementation
    └── core/
        ├── mod.rs
        ├── resolver.rs        # Resolves user input into files/folders
        └── walker.rs          # Shared directory traversal logic
```

The architecture is designed for easy extension: adding a new command means adding one variant to `Commands` in `cli.rs` and one new file under `commands/` — no other files need to change.

## Built with

- [clap](https://crates.io/crates/clap) — command-line argument parsing
- [walkdir](https://crates.io/crates/walkdir) — recursive directory traversal
- [anyhow](https://crates.io/crates/anyhow) — error handling
- [colored](https://crates.io/crates/colored) — terminal output styling

## Roadmap

- [ ] Search for a string/pattern across files
- [ ] Filter by file extension
- [ ] `.gitignore`-aware traversal
- [ ] JSON output mode

## License

MIT