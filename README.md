# random.pick 😀🥳😎😁
[![Test](https://github.com/MattBast/name_picker/actions/workflows/test.yaml/badge.svg)](https://github.com/MattBast/name_picker/actions/workflows/test.yaml)

A single page website built using Leptos that that allows users to type a list of objects and have a single one randomly picked for them.

Run this command to open the website in dev mode and listen for changes in the codebase. The page will reload whenever a change is detected:
```bash
trunk serve --open
```

May also need to install these if the above fails:
```bash
cargo install trunk
rustup target add wasm32-unknown-unknown
cargo install cargo-generate
```

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

Use these commands to run the end to end tests. These tests use Playwright to open the website, click around and make sure the content renders okay.
```bash
trunk serve
cd end2end
npx playwright test --reporter=line --max-failures=1
```
As Playwright is required, you will likely need to install it first. Go to the `end2end` directory and install all the required packages.
```bash
cd end2end
npm install
npm init playwright@latest
```

Sometimes port 3000 gets reserved after running leptos commands a gew times.
So if the "Address already in use" error appears, try killing the process
occupying the port. Start by find the process PID:
```bash
lsof -i :3000
```

and then killing the process with the PID you found:
```bash
kill <PID>
```
