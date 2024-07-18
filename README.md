# [blog-cross-rpi](https://github.com/alixinne/blog-cross-rpi)

Example repository for the post [I tried cross-compiling for the Raspberry
Pi](https://alixinne.github.io/posts/i-tried-cross-compiling/).

## Getting started

Compile and run for your host platform with `cargo`:

    # Base version
    cargo run

    # With dependencies
    cargo run --all-features

Compile for the Raspberry Pi Zero with [`cross`](https://github.com/rust-embedded/cross):

    # Base version
    cross build --target arm-unknown-linux-gnueabihf

    # With dependencies
    export ENABLE_PYO3=1
    cross build --target arm-unknown-linux-gnueabihf --all-features

## Author

Alixinne <alixinne@pm.me>
