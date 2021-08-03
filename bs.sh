#!/bin/bash
if [ -z "$2" ]; then
    echo "Usage: boxscript <mode> <path/to/file.bs>"
    exit 1
fi

if [ "$1" = "docker" ]; then
    if hash docker 2>/dev/null; then
        docker build -t bs . 2>/dev/null
        docker run -it --mount src="$(dirname "$(realpath "$2")")",target=/var/tmp,type=bind boxscript "$(basename $2)"
    else
        echo "Docker not installed. Install here: https://docs.docker.com/get-docker/"
        exit 1
    fi
elif [ "$1" = "nix" ]; then
    if hash nix-shell 2>/dev/null; then
        nix-shell --run "cargo build --release; cargo run \"$2\""
    else
        echo "Nix not installed. Install here: https://nixos.org/download.html"
        exit 1
    fi
elif [ "$1" = "rust" ]; then
    if hash cargo 2>/dev/null; then
        if hash llvm-config 2>/dev/null; then
            cargo build --release 2>/dev/null
            ./target/release/boxscript "$2"
        else
            echo "LLVM not installed. Install here: https://releases.llvm.org/download.html"
            exit 1
        fi
    else
        echo "Rust not installed. Install here: https://rustup.rs/"
        exit 1
    fi
fi
