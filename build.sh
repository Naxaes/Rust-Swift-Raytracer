#!/bin/sh

cargo build --release --lib
cp target/rust_raytracer.h MacOSPlatform/Engine/includes/
cp target/release/libraytracer.a MacOSPlatform/Engine/libs/
