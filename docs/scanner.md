# Scanner

The scanner walks a repository and produces a structured report.

## Current capabilities

- Respects gitignore and standard ignore files
- Detects binary files using NUL-byte probing
- Tracks file path, size, extension, and binary status
- Supports optional content previews
- Enforces maximum depth, maximum files, maximum total bytes, and maximum report size
- Runs a basic built-in rule engine

## Built-in rules

Current prototype rules:

- `todo-comment`
- `unsafe-usage`
- `possible-secret`

These rules are intentionally simple and will be replaced or extended by a configurable rule engine.

## Examples

Text summary:
```bash
cargo run -p dn-cli -- scan .

JSON report:

bash
cargo run -p dn-cli -- scan . --json

JSON report with content previews:

bash
cargo run -p dn-cli -- scan . --json --content

Limit scan depth:

bash
cargo run -p dn-cli -- scan . --max-depth 4

Limit number of files:

bash
cargo run -p dn-cli -- scan . --max-files 1000
