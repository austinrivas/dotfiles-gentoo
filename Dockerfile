FROM gentoo/stage3-amd64:latest

COPY target/debug/dotfiles /bin/dotfiles

CMD ["/bin/dotfiles"]