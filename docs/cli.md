# CLI Guide

`dn-cli` exposes two equivalent entry points:

- `scan` (preferred): run a review and emit output.
- `review`: explicit alias of `scan` with identical options.

## Commands

### `scan <path>`

Scan the repository/folder at `<path>`.

Examples:

```bash
dn-cli scan . --profile quick
dn-cli scan . --profile security --json
dn-cli scan . --profile architecture --markdown
dn-cli scan . --profile quick --hidden
dn-cli scan . --profile my-security
```

### `review <path>`

Equivalent to `scan`.

Examples:

```bash
dn-cli review . --profile quick
dn-cli review . --profile architecture --json
```

## Global behavior notes

- `--profile` can be a built-in profile name, a local profile name, or a direct file path.
- If the profile is unknown, scan exits with an error and a list of available profiles.
- Only one output mode is allowed: `--json` or `--markdown`.
- `--help` for either subcommand shows the same flag set.

## Flags

- `--profile <name|path>`
  - Built-in profile name (`quick`, `security`, `architecture`, ...), local profile name (`my-security`), or explicit file path.
  - Local names are resolved from `<scan root>/.dn/profiles/<name>.toml|.yml|.yaml`.
  - `<scan root>` is the `path` argument, such as `.` in `dn-cli scan . --profile quick`.
- `--json`
  - Emit machine-readable JSON report.
  - Conflicts with `--markdown` and is optional.
- `--markdown`
  - Emit structured Markdown report for PRs and review notes.
  - Conflicts with `--json` and is optional.
- `--content`
  - Include short `content_preview` for each scanned file in JSON/Markdown output.
  - `content_preview` can expose secrets; avoid posting raw previews in public logs.
- `--hidden`
  - Include dotfiles and dot-directories in discovery.
- `--python-worker`
  - Force Python worker enablement (if supported by the scan context).
  - Requires Python-capable runtime when using the CLI image.

## Docker usage

```bash
# Build locally first:
# docker build -t dn-kernel -f docker/Dockerfile .
docker run --rm -v "$PWD":/workspace -w /workspace dn-kernel \
  scan /workspace --profile quick --json
```

This local image runs `dn-cli` as the entrypoint, so any CLI arguments can be passed after the image name.

Docker readiness note:
this repository's container configuration is reviewed, but container validation was not completed in this environment
because external registry/network TLS timeouts prevented reliable image tooling access. The limitation is environmental,
not a confirmed functional defect in the CLI/runtime code.

The default image is CLI-only; Python worker support requires a custom image with Python available.

## Error behavior

User configuration errors should never panic. Expected failures (unknown profile,
malformed profile, worker/protocol errors) are reported as plain CLI errors and
non-zero exit codes.

Diagnostics from runtime execution are also printed under `errors` in structured
outputs (`--json`/`--markdown`) so failures can be consumed in automation.
