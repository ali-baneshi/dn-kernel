# Profiles

`dn-kernel` uses profile-driven policy to keep scans predictable and team-friendly.
A profile controls **what** is scanned, **how deep** local checks run, and **whether** worker/AI review is enabled.

## Profile sources and discovery

Profiles are resolved in this order:

1. **Explicit file path**
   - If `--profile` points to an existing file, it is loaded directly.
2. **Scan-root local profile**
   - `--profile my-security` resolves to `<root>/.dn/profiles/my-security.toml` (or `.yml`/`.yaml`).
3. **Built-in profiles**
   - Fallback built-ins include names listed in `available profiles`.

`<root>` is the first argument of `scan`/`review`.

## Built-in profiles

The built-ins are:

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

## Local profile format

Profiles are TOML or YAML files with these sections:

- `name` (required)
- `inherits` (optional)
- `[rules]`
- `[file_selection]`
- `[limits]`
- `[worker]`
- `[ai]`
- `[output]`

```toml
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
```

`include_hidden` is accepted at top-level for compatibility and is equivalent to `file_selection.include_hidden`.

## Inheritance and merge behavior

Local and built-in profiles can inherit from another profile by name using `inherits`.
Child profile fields override parent defaults in a simple, deterministic way:

- Scalar `bool`/numbers override when non-zero / set.
- Lists (rules, globs, patterns) override when non-empty.
- `include_binary` and `include_hidden` are additive with parent defaults.
- AI/profile source metadata is inherited when child does not explicitly enable AI.

## Error handling

Profile parsing and resolution errors are surfaced as user-facing CLI errors with hints:

- Unknown profile -> error + available suggestions
- Missing inherited profile -> explicit `unknown inherited profile '<name>'`
- Circular inheritance -> explicit `circular profile inheritance`
- Malformed TOML/YAML -> parse error with file path context

## Practical profile recipes

### Strict security baseline

```toml
name = "security-ops"
inherits = "security"
[limits]
max_file_size_bytes = 1_048_576
max_files = 2000
[ai]
enabled = true
provider = { type = "mock", message = "Summarize high-risk findings only" }
```

### Architecture review

```toml
name = "arch-lite"
inherits = "architecture"
[rules]
prioritize = ["large-file", "todo-comment"]
[output]
include_content_preview = false
```

### AI-generated code audit

```toml
name = "ai-audit"
inherits = "ai-generated-code-review"
include_hidden = true
[rules]
suspicious_patterns = ["TODO", "FIXME", "XXX", "generated", "copy" ]
```

### Hidden-inclusive review

```toml
name = "hidden-plus"
include_hidden = true
[limits]
max_files = 5000
```
