# commit-filter

Small CLI utility written in Rust that scans a Git repository and lists every commit that touched a given path (file or directory), printing a concise one‑line summary for each.

## Motivation

When you want to understand the evolution of a specific part of your code‑base (e.g. *"who changed `src/auth/` and when?"*) you normally reach for `git log -- <path>`.  `commit-filter` provides a friendlier, dedicated command that outputs only what you need, with clear formatting and robust error‑handling.

## Features

* **Path‑based filtering** – show commits that modified a single file or an entire directory.
* **De‑duplicated output** – the same commit is printed only once, even if it touched multiple files under the path.
* **Human‑readable timestamps** – powered by [`chrono`](https://crates.io/crates/chrono).
* **Works with bare or working repositories** – uses the libgit2 bindings (`git2` crate).
* Graceful error messages thanks to [`anyhow`](https://crates.io/crates/anyhow).

## Installation

### Prerequisites

* Rust toolchain ≥ 1.77 (edition 2024)
* A C compiler and build tools (required by `git2` / libgit2)

### Build from source

```bash
$ git clone https://github.com/nimec77/commit-filter
$ cd commit-filter
$ cargo build --release
```

The resulting binary will be at `target/release/commit-filter`.

### (Planned) `cargo install`

Once the crate is published you will be able to install it with:

```bash
cargo install commit-filter
```

---

## Usage

```bash
commit-filter <REPO_PATH> <FILTER_PATH>
```

| Argument      | Description                                                        |
| ------------- | ------------------------------------------------------------------ |
| `REPO_PATH`   | Path to the Git repository (can be `.` for the current directory). |
| `FILTER_PATH` | File or directory inside the repository to match.                  |

### Example

```bash
commit-filter ~/projects/my-app src/components/button.rs
```

Sample output:

```
commit: 5e1dbfc0ab1 | time: 2025-05-26 18:22:14 +0300 | summary: refactor: migrate button to new theme
commit: 27a34cd8e9b | time: 2025-05-17 09:05:01 +0300 | summary: fix: prevent double‑click on button
```

## How it works

1. Opens the repository using the [`git2`](https://github.com/rust-lang/git2-rs) bindings.
2. Walks the commit graph starting from `HEAD`.
3. For each commit, computes a diff against its first parent (or an empty tree for root commits).
4. For every changed file, checks if the path begins with `FILTER_PATH`; if so, the commit is added to a `HashSet`.
5. Finally prints each unique commit with its abbreviated ID, author date and summary.

The core logic lives in [`src/app/repositories/git_filter_repository.rs`](src/app/repositories/git_filter_repository.rs).

## Roadmap

* Sort output chronologically (currently emitted in Revwalk order).
* Support glob patterns (e.g. `**/*.rs`).
* JSON / CSV output for scripting.
* CLI flags to filter by author, date range or commit message.

## Contributing

Bug reports and pull requests are welcome!  Please open an issue first if you plan to work on a major feature so we can discuss the design.

## License

`commit-filter` is currently unlicensed.  A permissive license (MIT or Apache‑2.0) will be added soon – feel free to open an issue if you need one sooner.
