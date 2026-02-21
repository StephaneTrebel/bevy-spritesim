#!/usr/bin/env bash
set -euo pipefail

input=${1:-toto}
OUTPUT_DIR="assets/sprites/"$input
SPRITE_W=16
SPRITE_H=16
COLS=7
ROWS=7
SECTIONS=4
SECTION_H=$((SPRITE_H * ROWS))  # 112px par section

mkdir -p "$OUTPUT_DIR"

for anim in $(seq 0 $((SECTIONS - 1))); do
  for row in $(seq 0 $((ROWS - 1))); do
    for col in $(seq 0 $((COLS - 1))); do
      variante=$(( row * COLS + col ))
      offset_x=$(( col * SPRITE_W ))
      offset_y=$(( anim * SECTION_H + row * SPRITE_H ))

      convert "assets/sprites/terrain/$input.png" \
        -crop "${SPRITE_W}x${SPRITE_H}+${offset_x}+${offset_y}" \
        +repage \
        "${OUTPUT_DIR}/sprite_terrain_${input}_${variante}_${anim}.png"
    done
  done
done

echo "Done: $((SECTIONS * ROWS * COLS)) sprites extraits dans '$OUTPUT_DIR/'"
