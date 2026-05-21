# Scanner

`dn-kernel` scans repositories with predictable policy:

- file selection via include/exclude patterns
- size/resource limits
- deterministic local rules
- optional worker analysis
- optional provider review

## File discovery

Scanning starts at the target root and uses ignore-aware traversal.
Hidden paths are excluded by default; use `--hidden` to include dotfiles and dot-directories.

Rules:

- `ignore` handles `.gitignore` semantics.
- `WalkBuilder.hidden` is toggled by effective `include_hidden`.
- default excluded globs are `target/**`, `node_modules/**`, `.git/**`.

## Counters and semantics

- `files_discovered`: files that passed include/exclude globs.
- `files_scanned`: files that were analyzed.
- `files_selected`: files emitted in `files` output (equivalent to scanned).
- `files_skipped`: discovered files that were not scanned.
- `total_files`: compatibility alias of discovered count.
- `skipped_large_files`: files skipped because `max_file_size_bytes` was exceeded.
- `truncated`: true when scan terminated due to `max_files` or `max_total_bytes` policy.

## Limits

Per-profile or CLI-overridden limits:

- `max_file_size_bytes`
- `max_file_read_bytes`
- `max_total_bytes`
- `max_files`

These are enforced before expensive analysis.

## Content preview

`--content` (or profile output settings) adds short `content_preview` fields to report entries.
By default previews are omitted.

Security note: previews can leak secrets; this is expected and flagged in docs/help.

## Deterministic vs suspicious checks

- Deterministic rules always run against scanned file content (for supported text files).
- Suspicious checks are pattern-based; worker/provider are only attempted when `suspicious_patterns` matches.
- provider calls are bounded by `ai.max_ai_files` and content-length.
