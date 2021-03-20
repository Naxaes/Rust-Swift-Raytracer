#!/bin/sh

set -e

TARGET_PATH="MacOSPlatform/MacOSPlatform/Engine"

mkdir -p ${TARGET_PATH}/{includes,libs}/

if [ "$1" == "release" ]
  then
    echo Building Engine in release...
    cargo build --release --lib > /dev/null
    cp target/rust_raytracer.h ${TARGET_PATH}/includes/
    cp target/release/libraytracer.a ${TARGET_PATH}/libs/
else
    echo Building Engine in debug...
    cargo build --lib > /dev/null
    cp target/rust_raytracer.h ${TARGET_PATH}/includes/
    cp target/debug/libraytracer.a ${TARGET_PATH}/libs/
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

