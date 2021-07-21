# BoxScript

BoxScript, or BS for short, is a language based on the idea of "boxes".

What are boxes, exactly? Boxes, and their younger sibling, blocks, are simply units of code. They can be loops, conditionals, or anything else, really. Expressions with different purposes go into different blocks, and blocks with different functions go into different boxes. Sounds simple, right?

BoxScript's most defining feature is encouraging **thinking inside the box** when writing code—literally, since no code can exist outside of a box. If that's not BS, then what is?

## Requirements

- [Nix](https://nixos.org/download.html)

## Installation

1. Clone the repository

```sh
git clone https://github.com/boxscript/boxscript-rs.git
```

## Usage

1. Start Nix's shell

```sh
nix-shell
```

2. Use cargo to run the project

```sh
cargo run [path/to/file.bs]
```
