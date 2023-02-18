cd src/assets/icon
magick convert -background none -size 256 icon.svg -verbose -density 512 \
    \( -clone 0 -resize 8x8 \) \
    \( -clone 0 -resize 16x16 \) \
    \( -clone 0 -resize 32x32 \) \
    \( -clone 0 -resize 48x48 \) \
    \( -clone 0 -resize 64x64 \) \
    \( -clone 0 -resize 128x128 \) \
    \( -clone 0 -resize 256x256 \) \
    icon.ico