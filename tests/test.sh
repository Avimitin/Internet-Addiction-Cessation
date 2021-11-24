#!/bin/sh

set -xe
cargo test
cargo build
target/debug/auto-domain-blocker debug --config fixtures/domains.toml --host fixtures/emptyhosts.txt

