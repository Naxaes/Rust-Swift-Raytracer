#!/bin/sh

set -e

TARGET_PATH="MacOSPlatform/MacOSPlatform/Engine"

mkdir -p ${TARGET_PATH}/{includes,libs}/

if [ "$1" == "release" ]
  then
    echo Building Raytracer in release...
    cargo build --manifest-path raytracer/Cargo.toml --release --lib > /dev/null
    cp raytracer/target/raytracer.h ${TARGET_PATH}/includes/
    cp raytracer/target/release/libraytracer.a ${TARGET_PATH}/libs/
else
    echo Building Raytracer in debug...
    cargo build --manifest-path raytracer/Cargo.toml --lib > /dev/null
    cp raytracer/target/raytracer.h ${TARGET_PATH}/includes/
    cp raytracer/target/debug/libraytracer.a ${TARGET_PATH}/libs/
fi

# https://developer.apple.com/library/archive/technotes/tn2339/_index.html
pushd . > /dev/null
echo Building Platform...
cd MacOSPlatform
xcodebuild -workspace MacOSPlatform.xcodeproj/project.xcworkspace \
-scheme MacOSPlatform \
-configuration Debug \
-derivedDataPath build > /dev/null
popd > /dev/null

if [ "$2" == "run" ]
  then
    echo "Running"
    open ./MacOSPlatform/build/Build/Products/Debug/MacOSPlatform.app
fi

