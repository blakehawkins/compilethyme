#!/usr/bin/env bash

set -euxo pipefail

rustc <(cargo +nightly run -- "$(cat test.ct)") -o out && ./out
