# Development

## Setup

bash
make setup

## Build

bash
make build

## Format

bash
make fmt

## Lint

bash
make lint

## Test

bash
make test

## Smoke test

bash
make smoke

## Environment

Copy `.env.example` to `.env` and adjust values as needed.

Important variables:

- `DN_USE_LLM`
- `DN_LLM_BASE_URL`
- `DN_LLM_MODEL`
- `DN_LLM_API_KEY`
- `DN_LOG`
- `DN_MAX_FILES`
- `DN_MAX_FILE_BYTES`
- `DN_MAX_TOTAL_BYTES`
- `DN_MAX_FINDINGS`
- `DN_INCLUDE_HIDDEN`
- `DN_WORKER_TIMEOUT_SECS`
- `DN_PROTOCOL_VERSION`
