FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && apt-get install -y qemu-system-arm qemu-utils gcc-arm-none-eabi
RUN rustup target add armv7a-none-eabi

CMD ["/bin/bash"]