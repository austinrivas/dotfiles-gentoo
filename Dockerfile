FROM gentoo/stage3-amd64:latest

COPY target/release/dotfiles /bin/dotfiles

CMD [ "/bin/bash" ]