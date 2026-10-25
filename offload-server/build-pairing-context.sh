#!/bin/bash
# Copies libs/offload_core next to the pairing service so its Docker build context is
# self-contained. Run from anywhere inside the repository.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
rm -rf "$here/pairing/vendor"
mkdir -p "$here/pairing/vendor"
cp -r "$here/../libs/offload_core" "$here/pairing/vendor/offload_core"
rm -rf "$here/pairing/vendor/offload_core/target"
echo "pairing build context ready"
