# BoxScript

[![build](https://img.shields.io/github/workflow/status/boxscript/boxscript-rs/Rustup/main?style=for-the-badge)](https://github.com/boxscript/boxscript-rs/actions/workflows/rustup.yaml)
[![codacy](https://img.shields.io/codacy/grade/8b84e8126be94133be438ce24adff256?logo=Codacy&style=for-the-badge)](https://app.codacy.com/gh/boxscript/boxscript-rs)
[![codecov](https://img.shields.io/codecov/c/github/boxscript/boxscript-rs?logo=Codecov&style=for-the-badge&token=K9Yj1EvqFe)](https://codecov.io/gh/boxscript/boxscript-rs)
![size](https://img.shields.io/github/languages/code-size/boxscript/boxscript-rs?style=for-the-badge)
[![rust](https://img.shields.io/static/v1?label=Rust&message=1.54.0&color=red&logo=Rust&style=for-the-badge)](https://www.rust-lang.org)
[![license](https://img.shields.io/github/license/boxscript/boxscript-rs?style=for-the-badge)](https://github.com/boxscript/boxscript-rs/blob/main/LICENSE)

BoxScript, or BS for short, is a language based on the idea of "boxes".

What are boxes, exactly? Boxes, and their younger sibling, blocks, are simply units of code. They can be loops, conditionals, or anything else, really. Expressions with different purposes go into different blocks, and blocks with different functions go into different boxes. Sounds simple, right?

BoxScript's most defining feature is encouraging **thinking inside the box** when writing code—literally, since no code can exist outside of a box. If that's not BS, then what is?

## Requirements

[Rust](https://rustup.rs/)

[LLVM](https://releases.llvm.org/download.html)

If you have [Nix](https://nixos.org/download.html) or [Docker](https://docs.docker.com/get-docker/) installed, you will not need to install these.

## Installation

Clone the repository

```sh
git clone https://github.com/boxscript/boxscript-rs.git
```

## Usage

CD to where you cloned the repository:

```sh
cd [path/to/boxscript]
```

Then run `bs.sh`:

```sh
sh bs.sh [docker|nix|rust] [path/to/file.bs]
```

**The Docker option does not work, as LLVM 12 is unavailable for Alpine Linux.**
