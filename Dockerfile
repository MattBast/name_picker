FROM rust:1.84-alpine3.20 AS builder
# FROM debian:bullseye

# common packages
# RUN apt-get update && \
#     apt-get install --no-install-recommends -y \
#     build-essential \
#     autoconf automake autotools-dev libtool xutils-dev \
#     ca-certificates curl file && \
#     rm -rf /var/lib/apt/lists/*

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen
RUN npm install -g -D tailwindcss@3.4.0

# install toolchain
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
#     sh -s -- --target wasm32-unknown-unknown -y

# ENV PATH=/root/.cargo/bin:$PATH

# install cargo binstall to reduce image size
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

WORKDIR /app
COPY . .

#RUN rustup component add rust-analyzer rustfmt rust-src clippy
RUN cargo binstall trunk@0.21.1 -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

EXPOSE 3000

CMD ["trunk", "serve", "--release", "--address", "0.0.0.0"]
