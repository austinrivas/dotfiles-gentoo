#!/bin/bash
cargo build --release
docker build --no-cache -t dotfiles-image .
docker run -it dotfiles-image