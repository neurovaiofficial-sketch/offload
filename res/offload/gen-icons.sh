#!/bin/bash
# Renders Offload's icons from the SVGs in this directory into every place the build reads
# them from. Needs rsvg-convert (librsvg2-bin), ImageMagick and python3. The outputs are
# committed, so builds do not need these tools.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

render() { rsvg-convert -w "$2" -h "$2" "$1" -o "$3"; }

for s in 16 24 32 48 64 128 256 512 1024; do
  render "$here/icon.svg" "$s" "$tmp/icon_$s.png"
done

# Windows: application, tray and installer icons.
convert "$tmp/icon_16.png" "$tmp/icon_24.png" "$tmp/icon_32.png" "$tmp/icon_48.png" \
  "$tmp/icon_64.png" "$tmp/icon_128.png" "$tmp/icon_256.png" "$tmp/app.ico"
cp "$tmp/app.ico" "$root/res/icon.ico"
cp "$tmp/app.ico" "$root/res/tray-icon.ico"
cp "$tmp/app.ico" "$root/flutter/windows/runner/resources/app_icon.ico"
cp "$tmp/app.ico" "$root/flutter/assets/icon.ico"

# macOS: bundle icon and menu bar template (the template is drawn by macOS in its own colour).
python3 "$here/make_icns.py" "$root/flutter/macos/Runner/AppIcon.icns" "$tmp"
rsvg-convert -w 44 -h 44 "$here/tray-template.svg" -o "$root/res/mac-tray-dark-x2.png"
cp "$root/res/mac-tray-dark-x2.png" "$root/res/mac-tray-light-x2.png"
cp "$tmp/icon_1024.png" "$root/res/mac-icon.png"

# Shared PNG sizes and the in-app images.
cp "$tmp/icon_32.png" "$root/res/32x32.png"
cp "$tmp/icon_64.png" "$root/res/64x64.png"
cp "$tmp/icon_128.png" "$root/res/128x128.png"
cp "$tmp/icon_256.png" "$root/res/128x128@2x.png"
cp "$tmp/icon_512.png" "$root/res/icon.png"
cp "$here/icon.svg" "$root/res/scalable.svg"
cp "$here/icon.svg" "$root/flutter/assets/icon.svg"
cp "$tmp/icon_256.png" "$root/flutter/assets/icon.png"
rsvg-convert -w 600 -h 120 "$here/logo.svg" -o "$root/flutter/assets/logo.png"
echo "icons written"
