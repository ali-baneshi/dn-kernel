# Output Formats

`dn-kernel` supports three output modes:

- default plain-text report (default)
- JSON (`--json`)
- Markdown (`--markdown`)

Only one output mode is allowed per command.

## Text output

Human-readable key/value lines:

- root
- profile / profile_source
- provider
- worker
- files counters
- severity counts
- findings count
- optional errors

## JSON output

The JSON schema is centered on `ScanReport`.

### Top-level fields

- `root`: canonical scan root
- `profile`: effective profile name
- `provider`: provider identity string
- `worker`: worker summary (`python:python`, `python:python (single-shot)`, or `disabled`)
- `profile_source`: `builtin` or `file:<path>`
- `files_discovered`: matching files before selection and skip decisions
- `files_scanned`: files actually analyzed
- `files_selected`: files included in `files` array (same as scanned)
- `files_skipped`: discovered but not scanned
- `total_files`: compatibility alias for discovered count
- `total_bytes`: bytes read from scanned files
- `skipped_large_files`: files skipped due max-file-size limit
- `truncated`: whether scan was cut off by `max_files`/`max_total_bytes`
- `errors`: diagnostics collected during scan
- `files`: per-file findings and optional `content_preview`
- `severity_breakdown`
- `duration_ms`
- `summary`

`errors` are non-fatal diagnostics (for example: worker/protocol/provider failures).

`--json` and `--markdown` are intentionally mutually exclusive.

`provider`, `worker`, `files`, `errors`, and counter fields are currently a public compatibility surface.
Avoid breaking them without documenting changes.

## Markdown output

Markdown is designed for PR comments and review notes. It includes:

- header with profile/provider/worker metadata
- counters and severity summary
- grouped findings by file
- empty state when no findings
- error list

## Content previews

Use `--content` to include `content_preview` in JSON/Markdown output for each scanned file.

Security note: previews can include secrets, tokens, or credentials. Avoid posting
raw previews to public channels.
