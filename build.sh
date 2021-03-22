#!/bin/sh

set -e

TARGET_PATH="MacOSPlatform/MacOSPlatform/Engine"

rm -rf ${TARGET_PATH}
mkdir -p ${TARGET_PATH}/{includes,libs}/

if [ "$1" == "release" ]
  then
    echo Building Raytracer in release for arm64 and x86_64...
    cargo build                             \
      --manifest-path raytracer/Cargo.toml  \
      --release --lib                       \
      -Zmultitarget --target=aarch64-apple-darwin --target=x86_64-apple-darwin \
      > /dev/null

    # Add the built header file to the target.
    cp raytracer/target/raytracer.h ${TARGET_PATH}/includes/

    # In release mode, we want to build for multiple architectures.
    # This combines them to an universal/fat static library.
    ARM=raytracer/target/aarch64-apple-darwin/release/libraytracer.a
    X64=raytracer/target/x86_64-apple-darwin/release/libraytracer.a

    lipo -info ${ARM}
    lipo -info ${X64}
    lipo -create ${ARM} ${X64} -output raytracer/target/libraytracer.a

    # Add the universal static library to the target.
    cp raytracer/target/libraytracer.a ${TARGET_PATH}/libs/

    # https://developer.apple.com/library/archive/technotes/tn2339/_index.html
    echo Building Platform...
    xcodebuild \
      -project MacOSPlatform/MacOSPlatform.xcodeproj  \
      -scheme MacOSPlatform           \
      -configuration Release          \
      -arch x86_64 -arch arm64        \
      ONLY_ACTIVE_ARCH=NO             \
      -derivedDataPath MacOSPlatform/build \
      > /dev/null

    if [ "$2" == "run" ]
      then
        echo "Running"
        open ./MacOSPlatform/build/Build/Products/Release/MacOSPlatform.app
    fi

else
    echo Building Raytracer in debug for host platform...
    cargo build                             \
      --manifest-path raytracer/Cargo.toml  \
      --lib                                 \
      > /dev/null

    # Add the built header file to the target.
    cp raytracer/target/raytracer.h ${TARGET_PATH}/includes/

    # Add the host static library to the target.
    cp raytracer/target/debug/libraytracer.a ${TARGET_PATH}/libs/

    # https://developer.apple.com/library/archive/technotes/tn2339/_index.html
    echo Building Platform...
    xcodebuild \
      -project MacOSPlatform/MacOSPlatform.xcodeproj  \
      -scheme MacOSPlatform           \
      -configuration Debug            \
      ONLY_ACTIVE_ARCH=YES            \
      -derivedDataPath MacOSPlatform/build \
      > /dev/null

    if [ "$2" == "run" ]
      then
        echo "Running"
        open ./MacOSPlatform/build/Build/Products/Debug/MacOSPlatform.app
    fi
fi

