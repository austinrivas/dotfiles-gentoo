#!/bin/bash
cargo build --release
docker build --no-cache -t dotfiles-image .
docker run --rm -it dotfiles-image