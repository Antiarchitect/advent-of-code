#!/bin/bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

cargo run --bin part1
cargo run --bin part2
