# Provider Model

`dn-kernel` separates two analysis planes:

- **Worker layer**: language-aware analysis via external worker processes (currently Python)
- **Provider layer**: optional AI-style review via pluggable providers

## Provider configuration

Provider configuration is under `[ai]` in a profile:

```toml
[ai]
enabled = true
max_ai_files = 30
max_content_chars = 2048
provider = { type = "mock", message = "Explain risky patterns." }
```

## Provider implementations

### `disabled`

Default state. No AI review calls are made.

### `mock`

Current safe default for deterministic behavior. Mock provider returns a single synthetic finding
(`mock-ai-review`) with the configured message.

### `ollama` (future-ready)

Scaffold exists to call a local chat completion endpoint in the style expected by
`/api/chat/completions`.

The implementation requires:

- `base_url`
- `model`
- optional `api_key`, `timeout_secs`, `temperature`
- optional `extra_system_prompt`

This path is intentionally explicit and isolated so local providers can be added with a minimal rewrite.

## Provider status in reports

`provider` in the report is emitted as `<provider>@<profile_source>`
(e.g. `mock@builtin` or `ollama@builtin` / `disabled@file:...`).

## Provider troubleshooting

- If AI fails for a file, scan continues and the error is collected under `errors`.
- If no provider is enabled, no synthetic AI findings are added.
- Use `--json`/`--markdown` to inspect provider state and diagnostics.

