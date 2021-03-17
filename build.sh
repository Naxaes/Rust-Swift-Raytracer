#!/bin/sh

set -e

if [ "$1" == "release" ]
  then
    echo Building Engine in release...
    cargo build --release --lib > /dev/null
    cp target/rust_raytracer.h MacOSPlatform/Engine/includes/
    cp target/release/libraytracer.a MacOSPlatform/Engine/libs/
else
    echo Building Engine in debug...
    cargo build --lib > /dev/null
    cp target/rust_raytracer.h MacOSPlatform/Engine/includes/
    cp target/debug/libraytracer.a MacOSPlatform/Engine/libs/
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


open ./MacOSPlatform/build/Build/Products/Debug/MacOSPlatform.app

