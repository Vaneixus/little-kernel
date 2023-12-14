FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && apt-get install -y qemu-system-arm qemu-utils gcc-arm-none-eabi gh hexedit
RUN rustup toolchain install nightly && rustup default nightly && rustup component add rust-src

CMD ["/bin/bash"]