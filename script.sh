#!/bin/bash
# set -eux pipefail

if ! command -v typst &>/dev/null; then
  echo "Install typst" >&2
  exit 1
fi

categories=("exercices" "cours")

for category in "${categories[@]}"; do
  for file in "frontend/typst/$category"/*.typ; do
    if [ -f "$file" ]; then
      filename=$(basename "$file" .typ)
      typst c "$file" --format html --features html "frontend/html/$category/$filename.html"
    fi
  done
done
