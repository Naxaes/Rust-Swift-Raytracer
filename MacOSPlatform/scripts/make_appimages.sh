#!/usr/bin/env zsh

set -e

IMAGE="$1"
NAME=$(basename -- "$IMAGE")
EXTENSION="${IMAGE##*.}"

mkdir -p AppIcon.iconset
cat > AppIcon.iconset/Contents.json <<- End
{
  "images" : [
    {
      "filename": "icon_16x16.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "1x",
      "size" : "16x16"
    },
    {
      "filename": "icon_16x16@2x.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "2x",
      "size" : "16x16"
    },
    {
      "filename": "icon_32x32.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "1x",
      "size" : "32x32"
    },
    {
      "filename": "icon_32x32@2x.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "2x",
      "size" : "32x32"
    },
    {
      "filename": "icon_128x128.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "1x",
      "size" : "128x128"
    },
    {
      "filename": "icon_128x128@2x.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "2x",
      "size" : "128x128"
    },
    {
      "filename": "icon_256x256.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "1x",
      "size" : "256x256"
    },
    {
      "filename": "icon_256x256@2x.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "2x",
      "size" : "256x256"
    },
    {
      "filename": "icon_512x512.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "1x",
      "size" : "512x512"
    },
    {
      "filename": "icon_512x512@2x.${EXTENSION}",
      "idiom" : "mac",
      "scale" : "2x",
      "size" : "512x512"
    }
  ],
  "info" : {
    "author" : "xcode",
    "version" : 1
  }
}
End

echo "Generating sizes for ${IMAGE}"

convert "${IMAGE}" -resize 16x16!     "AppIcon.iconset/icon_16x16.${EXTENSION}"
convert "${IMAGE}" -resize 32x32!     "AppIcon.iconset/icon_16x16@2x.${EXTENSION}"   # Only Retina
convert "${IMAGE}" -resize 32x32!     "AppIcon.iconset/icon_32x32.${EXTENSION}"
convert "${IMAGE}" -resize 64x64!     "AppIcon.iconset/icon_32x32@2x.${EXTENSION}"   # Only Retina
convert "${IMAGE}" -resize 128x128!   "AppIcon.iconset/icon_128x128.${EXTENSION}"
convert "${IMAGE}" -resize 256x256!   "AppIcon.iconset/icon_128x128@2x.${EXTENSION}" # Only Retina
convert "${IMAGE}" -resize 256x256!   "AppIcon.iconset/icon_256x256.${EXTENSION}"
convert "${IMAGE}" -resize 512x512!   "AppIcon.iconset/icon_256x256@2x.${EXTENSION}"  # Only Retina
convert "${IMAGE}" -resize 512x512!   "AppIcon.iconset/icon_512x512.${EXTENSION}"
convert "${IMAGE}" -resize 1024x1024! "AppIcon.iconset/icon_512x512@2x.${EXTENSION}"  # Only Retina
