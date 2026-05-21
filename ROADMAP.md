# Roadmap

`dn-kernel` is pre-release and intentionally scoped for practical use.

## Current direction (implemented)

- Hardened CLI profile handling:
  - built-in + local profile loading
  - inheritance and validation
  - non-panicking user error handling
- Stable scan counters and reporting semantics
- Optional Python worker integration
- Profile- and output-mode-aware markdown/json reporting
- Documentation set for CLI, profiles, provider model, and protocol

## Near-term milestones

1. **Release packaging and onboarding**
   - document reproducible install artifacts
   - add release checks and versioned changelog expectations
2. **Provider reliability**
   - harden and document Ollama/local provider failure modes
   - add explicit fallback strategy and opt-in strictness flags
3. **Operational polish**
   - richer configuration validation diagnostics
   - optional threshold-based non-zero exit for findings
4. **Security and maintenance**
   - add security testing checklist (dependency pinning, content leakage guidance)
   - improve scan diagnostics grouping by category/source

## Out of scope for this release

- No UI/daemon architecture is planned in this stage.
- No behavior changes that alter default include/exclude policy without explicit profile/flag change.
- No breaking report schema changes without migration notes.
