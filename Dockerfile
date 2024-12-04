# Get started with a build env with Rust nightly
FROM rust:1.82-alpine3.20 AS builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g -D tailwindcss@3.4.0

# RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

ENV LEPTOS_TAILWIND_VERSION="3.4.0"
ENV LEPTOS_WASM_OPT_VERSION="0.2.95"

# Install cargo-leptos
RUN cargo binstall cargo-leptos@0.2.21 -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

# RUN cargo install cargo-leptos
RUN cargo update -p wasm-bindgen --precise 0.2.95
RUN cargo leptos build --release -vv

FROM rust:1.82-alpine3.20 AS runner

WORKDIR /app

COPY --from=builder /work/target/release/name_picker /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 3000

CMD ["/app/name_picker"]
