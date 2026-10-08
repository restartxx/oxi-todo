# oxi-todo

[Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)
[Version](https://img.shields.io/badge/version-0.1.0--alpha-orange?style=for-the-badge)
[No JS](https://img.shields.io/badge/no%20js%20%2F%20no%20gc%20%2F%20just%20rust-blue?style=for-the-badge)

A blazingly fast, minimal todo CLI written in Rust. Because life's too short for slow todo apps.

> Built as a learning project to de-rust Rust skills — from file handling to idiomatic error handling.

## Features

- ⚡ Instant — single binary, no runtime
- 📁 XDG compliant — stores data in your config dir
- 🧠 Idiomatic Rust — `anyhow`, `clap`, `NonZeroU8`, `BufReader`
- 🧹 Clean architecture — lean `main.rs`, logic isolated in `todo.rs`

## Installation

### From source

```bash
git clone https://github.com/youruser/oxi-todo
cd oxi-todo
cargo build --release
# binary at ./target/release/oxi-todo
```

Optional: move to PATH
```bash
cp ./target/release/oxi-todo ~/.local/bin/
```

## Demo

```bash
$ oxi-todo add "RIIR everything"
✓ Added: RIIR everything

$ oxi-todo list
1. [ ] RIIR everything

$ oxi-todo done 1
✓ Done: RIIR everything

$ oxi-todo list
1. [x] RIIR everything
```

## Usage

```bash
oxi-todo list
oxi-todo add "buy milk"
oxi-todo add "RIIR everything"
oxi-todo done 1
oxi-todo remove 2
```

### Commands

| Command | Description |
| :--- | :--- |
| `list` | List all todos |
| `add <txt>` | Add a new todo |
| `done <index>` | Mark todo as done (1-indexed) |
| `remove <index>` | Remove todo by index (1-indexed, 0 is blocked) |

Index validation uses `NonZeroU8` — no more panics on `0`.

## Project Structure

```
src/
├── main.rs  # Only CLI orchestration — Args parsing + match
└── todo.rs  # All logic: file_path(), open_file(), list/add/remove/done
```

`main.rs` is intentionally debloated. The goal: main only parses, `todo.rs` does the work.

## Tech Stack

- **clap** (derive) — CLI parsing
- **anyhow** — ergonomic error handling with `Context` and `ensure!`
- **dirs** — cross-platform config path
- Std lib: `OpenOptions`, `BufReader`, `create_dir_all`

Storage currently is plain `.txt`, one task per line. Next iteration will migrate to JSON with `serde`.

## Roadmap

- [x] Lean `main.rs` + auxiliary module
- [x] Proper error handling (no `unwrap()` in prod path)
- [ ] JSON storage with `serde_json` — `Task { done: bool, txt: String }`
- [ ] TUI with `ratatui`

## Why Rust?

Because after you fix that `\n` ghost bug with `find("[]")` vs `join("\n") + "\n"`, you earn the right to RIIR everything.

## License

MIT
