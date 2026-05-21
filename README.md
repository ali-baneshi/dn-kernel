# dn-kernel

`dn-kernel` is a terminal-first repository review CLI for scanning source trees and producing structured, repeatable findings.

It is designed for developers, maintainers, security reviewers, and automation workflows that need a fast local way to inspect a codebase for architectural issues, suspicious patterns, maintainability concerns, and profile-driven review signals.

In short: **point it at a repository, choose a profile, and get a deterministic review report in plain text, JSON, or Markdown.**

---

## Who this is for

`dn-kernel` is useful for:

- **developers** who want a quick local review before opening a PR
- **maintainers** who want a consistent way to inspect repositories
- **security-minded teams** who want lightweight scans for suspicious patterns and risky usage
- **tooling and automation workflows** that need machine-readable review output
- **experimentation and extension work** where optional workers or provider-backed review flows may be added later

If you want a local-first CLI that is predictable, scriptable, and transparent about its limits, this project is built for that use case.

---

## What it does

`dn-kernel` scans a repository or directory and generates structured findings based on built-in or custom profiles.

Core capabilities include:

- deterministic repository scanning
- built-in review profiles such as:
  - `quick`
  - `security`
  - `architecture`
  - `deep`
  - `performance`
  - `maintainability`
  - `strict`
  - and other specialized profiles
- local custom profiles from `.dn/profiles/<name>.toml|yml|yaml`
- plain text output for terminal use
- JSON output for automation and pipelines
- Markdown output for human-readable review reports
- optional hidden-file scanning with `--hidden`
- optional content previews with `--content`
- optional extension points for:
  - Python worker-based analysis
  - provider-backed review flows (`disabled`, `mock`, current `ollama` scaffold)

The project aims to keep scanning behavior predictable and explicit. When optional integrations fail, errors are reported in diagnostics instead of crashing the CLI.

---

## Typical use cases

Common ways to use `dn-kernel` include:

- checking a repository before review or merge
- running a quick local security-oriented scan
- generating JSON output for CI or internal tooling
- producing Markdown review artifacts for discussion
- experimenting with custom profile rules for a specific codebase
- inspecting hidden paths such as `.github`, `.env`, or other dotfiles when needed

---

## Quick start

### Build from source
```bash
cargo build --workspace
cargo run -p dn-cli -- scan . --profile quick

### Install locally

bash
cargo install --path apps/dn-cli
dn-cli scan . --profile quick

### Example commands

bash
dn-cli scan . --profile quick
dn-cli scan . --profile security --json
dn-cli scan . --profile architecture --markdown
dn-cli scan . --profile my-security --hidden --content
dn-cli review . --profile architecture --json

`review` is an alias of `scan`.

---

## What the output looks like

`dn-kernel` supports three practical output styles:

- **plain text** for direct terminal use
- **JSON** for machine-readable automation
- **Markdown** for readable reports and sharing

Examples:

bash
dn-cli scan . --profile quick
dn-cli scan . --profile security --json
dn-cli scan . --profile architecture --markdown

Only one output mode can be selected per command.

---

## Profiles

Profiles define how a scan behaves: which rules are active, what files are considered, and which limits or integrations apply.

Built-in profile examples include:

- `quick`
- `security`
- `architecture`
- `deep`
- `performance`
- `maintainability`
- `ai-generated-code-review`
- `legacy-modernization`
- `pre-merge`
- `strict`
- `educational`
- `production-readiness`

You can also add local custom profiles under:

text
<scan-root>/.dn/profiles/

Resolution order is:

1. explicit profile path passed via `--profile`
2. local profile under `<scan-root>/.dn/profiles/<name>.toml|yml|yaml`
3. built-in profile

Example custom profile:

toml
name = "my-security"
inherits = "security"
include_hidden = true

[rules]
deterministic_rules = ["todo-comment", "possible-secret", "unsafe-usage"]
suspicious_patterns = ["password", "api_key", "token"]
prioritize = ["possible-secret", "unsafe-usage"]
min_severity = "info"

[file_selection]
include_binary = false
include_hidden = true
exclude_globs = [".git/**", "target/**"]

[limits]
max_file_size_bytes = 2_097_152
max_file_read_bytes = 16_384
max_total_bytes = 50_000_000
max_files = 10_000

[worker]
enabled = true

[ai]
enabled = true
max_ai_files = 30
max_content_chars = 1000
provider = { type = "mock", message = "Explain top risks briefly" }

For more detail, see [`docs/profiles.md`](docs/profiles.md).

---

## Hidden files and content previews

By default, hidden files and directories are excluded.

Use:

bash
dn-cli scan . --profile quick --hidden

to include entries such as:

- `.env`
- `.github`
- other dotfiles and hidden directories

You can also enable file content previews:

bash
dn-cli scan . --profile quick --content

This includes a limited preview in scan results.

> Warning: `--content` can expose secrets or other sensitive material in output. Use it carefully, especially in shared environments or public logs.

---

## Worker and provider integrations

`dn-kernel` has two optional extension layers:

- **worker layer**: external language-aware analysis
  - **Python worker**: AST-based analysis for Python code
  - **C worker**: Linux kernel coding style checks (checkpatch.pl rules)
    - Offline, no network dependencies
    - Implements 5 key kernel style rules (line-length, space-before-tab, trailing-whitespace, keyword-spacing, brace-style)
- **provider layer**: AI-style or provider-backed review flows

Current provider states include:

- `disabled`
- `mock`
- `ollama` scaffold

These integrations are optional. If they are unavailable or fail, the CLI continues and reports diagnostics in the output instead of terminating unexpectedly.

---

## Docker

A CLI-focused Docker image is included for quick evaluation:

bash
docker build -t dn-kernel -f docker/Dockerfile .
docker run --rm -v "$PWD":/workspace -w /workspace dn-kernel scan /workspace --profile quick --json

Notes:

- the default image is focused on the Rust CLI runtime
- Python worker support is **not installed** in the default image
- `--python-worker` may therefore report integration or startup diagnostics unless you build a custom image with Python and worker dependencies included

Current status:

- Docker configuration is present and intended for CLI usage
- full end-to-end Docker validation has **not** been confirmed in this environment due to external registry/network TLS timeout issues

---

## Installation requirements

To build or run from source, you typically need:

- Rust toolchain and `cargo`
- `git` for repository-root/profile workflows
- Python 3 only if you want Python worker-based analysis

---

## Design goals

This project is built around a few practical goals:

- **predictable local behavior**
- **deterministic scanning**
- **stable, script-friendly output**
- **clear failure reporting**
- **optional integrations instead of mandatory remote dependencies**

That means the CLI prefers explicit diagnostics over silent behavior, and integration failures are surfaced in reports rather than hidden.

---

## Current project status

`dn-kernel` is currently **pre-release**.

What that means in practice:

- core CLI behavior is usable
- profile-driven local scanning is the main focus
- documentation and behavior are being tightened for public use
- some optional integration paths are still evolving
- Docker validation is not fully confirmed in this environment
- `ollama` support is scaffolded, not production-hardened

If you are evaluating the project today, it is best treated as an early public release aimed at real usage, feedback, and iteration.

---

## Documentation

Project documentation:

- [`docs/cli.md`](docs/cli.md)
- [`docs/architecture.md`](docs/architecture.md)
- [`docs/scanner.md`](docs/scanner.md)
- [`docs/profiles.md`](docs/profiles.md)
- [`docs/providers.md`](docs/providers.md)
- [`docs/output.md`](docs/output.md)
- [`docs/protocol.md`](docs/protocol.md)
- [`docs/development.md`](docs/development.md)
- [`docs/troubleshooting.md`](docs/troubleshooting.md)

Project metadata and process docs:

- [`CHANGELOG.md`](CHANGELOG.md)
- [`SECURITY.md`](SECURITY.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`ROADMAP.md`](ROADMAP.md)

---

## Quick troubleshooting

- **Unknown profile**: ensure the profile exists under `<scan root>/.dn/profiles` or pass an explicit file path
- **Worker appears inactive**: check JSON or Markdown output for `worker` and `errors`; worker analysis only runs in supported cases
- **Hidden files not showing up**: verify the scan root and check ignore rules or `exclude_globs`
- **Unexpected output differences**: confirm you are using the intended profile and only one output mode flag

---

## Contributing

Contributions are welcome.

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) before changing public CLI behavior, docs, or profile semantics.

A good default contribution style is:

- keep behavior and documentation aligned
- prefer explicit error handling
- add or update tests when changing public-facing behavior

---

## Security

If you believe you have found a security issue, please see [`SECURITY.md`](SECURITY.md) for reporting guidance.

---

## Roadmap and limitations

Current limitations include:

- provider calls are synchronous and profile-driven
- worker protocol currently centers on Python implementation
- `ollama` integration is scaffolded and not yet production-hardened

Planned next steps are tracked in [`ROADMAP.md`](ROADMAP.md).
