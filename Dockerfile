FROM rust:1.84-alpine3.20 AS builder

# copy all code files into a directory called `work`
WORKDIR /work
COPY . .

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen
RUN npm install -g -D tailwindcss@3.4.0

# install cargo binstall to reduce image size
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN cargo binstall wasm-bindgen-cli
RUN cargo binstall trunk@0.21.1 -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# specify the version of tailwind to use
ENV TRUNK_TOOLS_TAILWINDCSS="3.4.0"

# build the app binary
RUN trunk build --release -M

# create a new layer with just a static file server
FROM caddy:2-alpine

WORKDIR /usr/share/caddy
# Copy only the dist directory containing the built files
COPY --from=builder /work/dist/* ./

# make sure the container exposes port 3000
EXPOSE 3000

# Configure caddy to serve files on port 3000
CMD ["caddy", "file-server", "--listen", ":3000"]
