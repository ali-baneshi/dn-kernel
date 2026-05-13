# dn-kernel

A modular Rust-first kernel for repository scanning, worker orchestration, and code review pipelines.

## Status

Current phase:

- Rust workspace builds successfully
- CLI works
- Healthcheck works
- Repository scanner is being implemented
- Python worker placeholder exists

## Workspace

- apps/dn-cli
- crates/dn-runtime
- crates/dn-ipc
- crates/dn-workers
- workers/python
- schemas
- docs
- scripts

## Quick start

Run:

    cp .env.example .env
    make setup
    make build
    cargo run -p dn-cli -- health
    cargo run -p dn-cli -- scan . --json

## Commands

Run:

    make help
    make setup
    make fmt
    make lint
    make test
    make build
    make run
    make check

## Scanner

Example:

    cargo run -p dn-cli -- scan .
    cargo run -p dn-cli -- scan . --json
    cargo run -p dn-cli -- scan . --max-depth 6 --max-files 5000

## Docker

Run:

    docker compose config
    docker compose up --build

## Scanner documentation

See:

docs/scanner.md

## Current prototype rules

The current scanner includes a small built-in rule engine:

- todo-comment
- unsafe-usage
- possible-secret

These are prototype rules and are expected to become configurable in later phases.
