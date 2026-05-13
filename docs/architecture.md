# Architecture

## High-level flow

1. CLI parses command and arguments
2. Runtime loads configuration and prepares a job directory
3. Runtime serializes a request envelope
4. Runtime launches the Python worker
5. Worker scans and analyzes the repository
6. Worker optionally asks a local-compatible LLM for a concise summary
7. Worker returns structured JSON
8. Runtime stores job artifacts and prints a result

## Design principles

- Local-first execution
- Deterministic output paths
- Process isolation between supervisor and worker
- Schema-based IPC contracts
- Bounded resource usage
- Extensible plugin-ready worker layout

## Transport

- Stdio
- One JSON request line
- One JSON response line

## Artifact layout

- `.dn/jobs/<job-id>/request.json`
- `.dn/jobs/<job-id>/response.json`
- `.dn/jobs/<job-id>/output/report.md`
- `.dn/jobs/<job-id>/output/report.json`

## Future improvements

- Persistent worker mode
- Streaming progress
- Git-aware scanning
- Ignore file support
- Plugin manifests
- Incremental caching
