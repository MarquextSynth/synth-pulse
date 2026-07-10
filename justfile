# ===============================
# Synth-Pulse Development Commands
# ===============================

default:
    @just --list

run:
    cargo run

build:
    cargo build

release:
    cargo build --release

check:
    cargo check

fmt:
    cargo fmt

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test

clean:
    cargo clean

doc:
    cargo doc --open

fix:
    cargo fmt
    cargo clippy --fix --allow-dirty

all:
    cargo fmt
    cargo clippy --all-targets --all-features
    cargo test
    cargo build
