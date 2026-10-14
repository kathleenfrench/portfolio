#!/usr/bin/env bash
# resize + recompress a photo for the site (macOS only, uses the built-in `sips`)
#
# usage: scripts/optimize-image.sh <input> [output_name] [max_edge_px]
#
#   scripts/optimize-image.sh ~/Downloads/IMG_1234.HEIC trout_bed
#     -> static/assets/images/trout_bed.jpg (longest edge 1000px, ~100-200KB)
#
# photos are shown at most ~300px wide (600px on retina), so 1000px is plenty.
set -euo pipefail

in="${1:?usage: $0 <input> [output_name] [max_edge_px]}"
name="${2:-$(basename "${in%.*}")}"
max_edge="${3:-1000}"
out="$(dirname "$0")/../static/assets/images/${name}.jpg"

w=$(sips -g pixelWidth "$in" | awk '/pixelWidth/ {print $2}')
h=$(sips -g pixelHeight "$in" | awk '/pixelHeight/ {print $2}')
resize=()
# only ever shrink, never upscale
if (( w > max_edge || h > max_edge )); then resize=(-Z "$max_edge"); fi

sips -s format jpeg -s formatOptions 80 ${resize[@]+"${resize[@]}"} "$in" --out "$out" >/dev/null
echo "$(du -h "$out" | cut -f1)  $out"
