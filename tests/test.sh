#!/bin/sh

set -xe
cargo test -- --nocapture
cargo build
