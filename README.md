# Name Picker
A single page website built using Leptos that (in the future will) select a random name from a list the user created.

## Dev commands
Check and test:
```bash
cargo check
cargo test
```

Linting:
```bash
cargo clippy
leptosfmt ./**/*.rs
```

Run in dev mode:
```bash
cargo leptos watch
```

May also need to install these:
`rustup target add wasm32-unknown-unknown`
`cargo install cargo-generate`

Build and run a Docker image:
```bash
docker build -t name-picker .
docker run --rm -it -p 3000:3000 --name first-run name-picker
```
