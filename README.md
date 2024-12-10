# random.pick 😀🥳😎😁
[![Test](https://github.com/MattBast/name_picker/actions/workflows/test.yaml/badge.svg)](https://github.com/MattBast/name_picker/actions/workflows/test.yaml)

A single page website built using Leptos that that allows users to type a list of objects and have a single one randomly picked for them.

Run this command to open the website in dev mode and listen for changes in the codebase. The page will reload whenever a change is detected:
```bash
cargo leptos watch
```

May also need to install these if the above fails:
`rustup target add wasm32-unknown-unknown`
`cargo install cargo-generate`

To test if a release version can be built and run, try these commands:
```bash
docker build -t name-picker .
docker run --rm -it -p 3000:3000 --name first-run name-picker
```

These commands are used in the CI pipeline to check the standard of the Rust code:
```bash
cargo check --verbose --all-targets --all-features
cargo clippy -- -D warnings
cargo fmt --check
cargo test --verbose --all-targets
```

This command is not yet used in the pipeline but can be used to standardise the format of the HTML macros.
```bash
leptosfmt ./**/*.rs
```
