# Runtime Architecture

`dn-kernel` is separated into a small CLI and a reusable runtime crate.

## Components

- `apps/dn-cli`
  - parses commands and flags
  - loads profile suggestions for unknown-profile hints
  - renders report formats
- `crates/dn-runtime`
  - loads and merges profiles
  - walks files with ignore-aware traversal
  - runs deterministic rules
  - orchestrates optional worker and provider steps
  - generates structured report
- `crates/dn-ipc`
  - shared request/response data models for workers
- `workers/python`
  - external language worker implementation

## Data flow

1. CLI selects root + profile + output mode.
2. Runtime resolves profile (builtin / local file / explicit path) and applies inheritance.
3. Scanner builds include/exclude selectors and walks files.
4. Each candidate file is:
   - filtered by include/exclude
   - filtered out when not text/binary policy allows
   - skipped if too large or over global limits
   - read and analyzed by local rules
   - optionally analyzed by worker when suspicious patterns match
   - optionally analyzed by provider when enabled
5. Findings are merged, sorted, counted, and serialized into a `ScanReport`.

## Extensibility points

- Add deterministic checks in `run_local_rules`.
- Add new runtime workers via `WorkerRegistry`/`WorkerSession` and keep protocol compatibility with `crates/dn-ipc`.
- Add providers in `crates/dn-runtime/src/provider.rs` using `Provider::from_config`.
- Update docs (`docs/protocol.md`, `docs/providers.md`) alongside any protocol or provider changes.

## Extensibility checklist

- add parsing + validation tests for new config fields
- keep default behavior opt-out compatible
- keep worker/protocol failures non-fatal when possible
- surface integration mode and failures explicitly in `provider`/`worker` fields
- add integration coverage under `apps/dn-cli/tests`

The worker layer and provider layer remain independent, so workers can exist even when provider is disabled and vice versa.
