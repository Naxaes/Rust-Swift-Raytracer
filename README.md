Prerequisites IOS:

    rustup target add aarch64-apple-ios x86_64-apple-ios
    cargo install cargo-lipo
    cargo install cbindgen
    cargo lipo --release