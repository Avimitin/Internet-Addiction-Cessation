#!/bin/sh

set -xe
RUST_LOG=info cargo test
cargo build
