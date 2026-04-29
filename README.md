<h1 align="center">r-git</h1>
<p align="center"><strong>A Git Implementation in Rust</strong></p>

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.87+-DEA584?style=flat-square&logo=rust" />
  <img src="https://img.shields.io/badge/Clap-CLI-orange?style=flat-square" />
  <img src="https://img.shields.io/badge/SHA--1-content_addressable-blue?style=flat-square" />
  <img src="https://img.shields.io/badge/Zlib-compression-9cf?style=flat-square" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" />
</p>

<p align="center">
  A from-scratch reimplementation of Git's plumbing and porcelain commands in Rust. Builds the same content-addressable object store, index format, and refs layout that real Git uses — just with <code>.rgit</code> instead of <code>.git</code>.
</p>

---

## The Problem

Git is one of the most used pieces of software in the world, and yet most developers — myself included — treat it as a black box. You run `git commit`, something happens, and your code is "saved." But what actually happens? What's an object? What's the index? Why is `git add` separate from `git commit`?

The best way to answer those questions is to build it.

## The Solution

`r-git` is a Rust implementation of Git's core data model: a content-addressable object store keyed by SHA-1 hashes, a staging area (the index), refs that point to commits, and a HEAD that points to a branch. Each command is implemented from scratch — no calls out to `git` under the hood.

The project is split into a thin command layer that handles CLI parsing and I/O, and an internal layer that does the real work:

| Layer | Module | Responsibility |
|:---:|---|---|
| **1** | `cli` | Clap-based argument parsing, subcommand routing |
| **2** | `internal::commands` | One file per Git command — `init`, `add`, `commit`, `status`, etc. |
| **3** | `internal::utils` | Object hashing, zlib compression, tree building, index parsing, ref management |

Errors flow back up through a single `RGitError` enum with `thiserror`, so every failure carries context about which file or operation went wrong.

---

## Features

- **Content-addressable object store** — blobs, trees, and commits are all SHA-1 hashed and zlib-compressed under `.rgit/objects/`, just like real Git
- **Plumbing commands** — `hash-object`, `cat-file`, `write-tree`, `ls-tree`, `commit-tree` for poking at the object store directly
- **Porcelain commands** — `init`, `add`, `commit`, `status`, `log`, `checkout`, `branch` for the everyday workflow
- **Real staging area** — `add` writes to a `.rgit/index` file that `commit` reads, mirroring Git's two-phase commit model
- **Branches and HEAD** — refs live under `.rgit/refs/heads/` and `HEAD` tracks the current branch (or a detached commit)
- **Detached HEAD checkout** — checkout by branch name to switch branches, or by commit hash to enter detached HEAD state
- **Three-way status** — `status` diffs `HEAD` ↔ `index` ↔ working directory to show staged, unstaged, and untracked changes
- **Structured errors** — every failure path returns a typed `RGitError` variant with the offending path and the underlying cause

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                         User / Shell                         │
│                                                              │
│      rgit init    rgit add .    rgit commit -m "..."         │
│      rgit log     rgit status   rgit checkout <ref>          │
└──────────────────────────┬───────────────────────────────────┘
                           │
┌──────────────────────────┴───────────────────────────────────┐
│              Layer 1: CLI (clap)                             │
│                                                              │
│   Parses argv, dispatches to the matching command handler,   │
│   prints any RGitError to stderr.                            │
└──────────────────────────┬───────────────────────────────────┘
                           │
┌──────────────────────────┴───────────────────────────────────┐
│              Layer 2: Commands                               │
│                                                              │
│   ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐        │
│   │   add    │ │  commit  │ │  status  │ │   log    │        │
│   └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘        │
│        │            │            │            │              │
│   ┌────┴─────┐ ┌────┴─────┐ ┌────┴─────┐ ┌────┴─────┐        │
│   │ checkout │ │  branch  │ │   init   │ │ ls-tree  │  ...   │
│   └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘        │
└────────┼────────────┼────────────┼────────────┼──────────────┘
         │            │            │            │
┌────────┴────────────┴────────────┴────────────┴──────────────┐
│              Layer 3: Internals                              │
│                                                              │
│   ┌────────────────────────────────────────────────────┐     │
│   │  hash_content   →   SHA-1 of (header + payload)    │     │
│   │  zlib_encoder   →   compress + write to .rgit/obj  │     │
│   │  read_object    →   read + decompress + parse      │     │
│   │  parse_index    →   load staging area entries      │     │
│   │  build_tree...  →   index → tree object recursion  │     │
│   │  parse_head     →   walk HEAD → branch → commit    │     │
│   └────────────────────────────────────────────────────┘     │
│                                                              │
│                          .rgit/                              │
│            ┌───────────────┼───────────────┐                 │
│            │               │               │                 │
│        objects/          refs/            HEAD               │
│        (zlib blobs,    (heads/main,    (ref: refs/...)       │
│         trees, commits) (other branches)                     │
└──────────────────────────────────────────────────────────────┘
```

---

## Commands

| Command | Type | Description |
|---|---|---|
| `init` | porcelain | Create a `.rgit/` directory with `objects/`, `refs/heads/`, `HEAD`, and an empty index |
| `add <path>` | porcelain | Hash a file (or `.` for everything) and write its entry into the index |
| `commit -m <msg>` | porcelain | Build a tree from the index, write a commit object, advance the current branch |
| `status` | porcelain | Show staged changes, unstaged changes, and untracked files |
| `log` | porcelain | Walk the commit history from HEAD backwards through `parent` pointers |
| `branch <name>` | porcelain | Create a new branch pointing at the current commit |
| `checkout <ref>` | porcelain | Switch to a branch (by name) or enter detached HEAD (by commit hash) |
| `hash-object <file>` | plumbing | Hash a file's contents as a blob; `-w` to write, `--stdin` to read from stdin |
| `cat-file <hash>` | plumbing | Print an object's contents (`-p`) or its type (`-t`) |
| `write-tree` | plumbing | Write the current index as a tree object and print its hash |
| `ls-tree <hash>` | plumbing | List the entries of a tree object |
| `commit-tree <tree> -m <msg>` | plumbing | Create a commit object from a tree hash and a message |

---

## Quick Start

### Prerequisites

- Rust 1.87+
- Cargo

### 1. Clone and build

```bash
git clone https://github.com/prranavv/r-git.git
cd r-git

cargo build --release
```

### 2. Install (optional)

```bash
cargo install --path .
```

This drops the `rgit` binary into `~/.cargo/bin/`.

### 3. Use it

```bash
mkdir hello && cd hello
rgit init
echo "hello world" > greeting.txt
rgit add .
rgit commit -m "first commit"
rgit log
```

---

## Example: A Complete Workflow

```bash
# Initialize a new repository
rgit init

# Stage a file and inspect the index
echo "fn main() {}" > main.rs
rgit add main.rs
rgit status
# Changes to be commited:
#         new file: main.rs

# Make a commit
rgit commit -m "initial commit"

# Branch off and switch
rgit branch feature
rgit checkout feature

# Modify, stage, and commit
echo "fn main() { println!(\"hi\"); }" > main.rs
rgit add main.rs
rgit commit -m "add print"

# See the history
rgit log

# Drop into detached HEAD on the previous commit
rgit checkout <commit-hash>
# You are in 'detached HEAD' state.
```

### Plumbing commands

The plumbing commands let you poke at the object store directly — useful for understanding what's actually happening underneath.

```bash
# Hash a file as a blob without writing it
echo "hello" | rgit hash-object --stdin
# ce013625030ba8dba906f756967f9e9ca394464a

# Hash and write
rgit hash-object -w main.rs

# Inspect the object
rgit cat-file -t <hash>     # blob
rgit cat-file -p <hash>     # fn main() {}

# Snapshot the index as a tree
rgit write-tree
# 4b825dc642cb6eb9a060e54bf8d69288fbee4904

# List the tree's entries
rgit ls-tree <tree-hash>
```

---

## Project Structure

```
r-git/
├── src/
│   ├── main.rs                     # Entry point — parses CLI, dispatches to commands
│   ├── cli.rs                      # Clap struct
│   ├── error.rs                    # RGitError enum (thiserror)
│   └── internal/
│       ├── mod.rs                  # Re-exports
│       ├── enums.rs                # Commands enum (clap subcommands)
│       ├── commands/
│       │   ├── init.rs
│       │   ├── add.rs
│       │   ├── commit.rs
│       │   ├── status.rs
│       │   ├── log.rs
│       │   ├── branch.rs
│       │   ├── checkout.rs
│       │   ├── hash_object.rs
│       │   ├── cat_file.rs
│       │   ├── write_tree.rs
│       │   ├── ls_tree.rs
│       │   └── commit_tree.rs
│       └── utils/                  # Object I/O, hashing, tree building, index parsing
│
├── Cargo.toml
├── README.md
└── LICENSE
```

---

## Tech Stack

| Component | Crate | Purpose |
|---|---|---|
| **Language** | Rust | Memory-safe systems programming with zero-cost abstractions |
| **CLI** | `clap` | Derive-based argument parsing with subcommands |
| **Errors** | `thiserror` | Ergonomic error enum with `#[from]` and `#[source]` |
| **Hashing** | `sha1` | SHA-1 digests for content addressing |
| **Compression** | `flate2` | Zlib encode/decode for object storage |
| **Hex** | `hex` | Hex-encoding object hashes |

---

## How It Works

**Objects.** Every file you commit, every directory snapshot, and every commit is stored as an *object* under `.rgit/objects/<first 2 chars of hash>/<remaining 38 chars>`. The hash is SHA-1 of `<type> <length>\0<content>`. The file on disk is that same payload, zlib-compressed.

- A **blob** is the contents of a file.
- A **tree** is a list of entries — each one a mode, a name, and a hash pointing to a blob (file) or another tree (subdirectory).
- A **commit** points at a tree, optionally points at a parent commit, and carries the author and message.

**The index.** `.rgit/index` is a plain text file where each line is `<mode> <path> <hash>`. `add` writes to it; `commit` reads from it to build the tree.

**Refs and HEAD.** `.rgit/refs/heads/<branch>` is a file containing a commit hash. `.rgit/HEAD` either contains `ref: refs/heads/<branch>` (attached) or a raw commit hash (detached). `commit` updates whatever branch HEAD points at; `checkout` updates HEAD itself.

**Status.** `status` is just three set comparisons: HEAD-tree vs index gives staged changes, index vs working directory gives unstaged changes, working directory minus index gives untracked.

---

## FAQ's

**"Why build Git from scratch?"**
> Because reading about Merkle trees and content-addressable storage is one thing, and watching `cat-file -p <hash>` actually print back the bytes you stored is another. The data model is genuinely elegant once you've built it yourself, and it stops being magic.

**"Why is the directory called `.rgit` instead of `.git`?"**
> So you can run `r-git` inside a real Git repo without them stomping on each other. Same data model, separate metadata directory.

**"Is this a drop-in replacement for Git?"**
> No. It implements the core data model and the most common commands, but it doesn't have remotes, merges, rebases, stashes, hooks, packfiles, or any of the things that make Git a complete tool. The point is to understand the foundation, not replace `git`.

**"Why SHA-1 when real Git is moving to SHA-256?"**
> SHA-1 is what nearly every Git repository in the wild uses, and it's what every blog post and book describing Git's internals references. Once the model clicks, swapping the hash function is a one-line change.

**"Why one file per command?"**
> Each command is small enough to fit on a screen, and the symmetry makes the project easy to navigate — if you want to know how `commit` works, you open `commit.rs`. The shared logic (hashing, compression, index parsing) lives in `utils/` so commands stay declarative.

---

## License

MIT — see [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Built by <a href="https://github.com/prranavv">Pranav</a></sub>
</p>