FROM archlinux:latest

COPY target/release/dotfiles /bin/dotfiles

CMD [ "/bin/bash" ]