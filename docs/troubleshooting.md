# Troubleshooting

## Unknown profile

Use a known profile name:

```bash
cargo run -p dn-cli -- scan . --profile quick
```

If using a local profile, ensure the file is under `<root>/.dn/profiles/<name>.toml|yml|yaml>`.

The error message includes available profile hints.

## Worker not used / silent

Worker checks are currently triggered only for suspicious files and supported file languages.
If no findings and no errors appear, it may mean no suspicious patterns matched.

Enable verbose diagnostics with `--json` and inspect `errors`.

## `--hidden` appears ineffective

- Ensure running from the intended root path.
- Confirm hidden path is not excluded by `.gitignore` plus profile `exclude_globs`.

## JSON/Markdown output looks incomplete

- Use `--json` or `--markdown` explicitly.
- Ensure `--content` is enabled if you need file previews.

## I get unknown profile errors

- The local profile directory for a scan is `<scan-root>/.dn/profiles`.
- If you want explicit file mode, pass an existing file path to `--profile`.
- When a profile cannot be resolved, help text now includes available profile hints.

## Untrusted profile files

- Review local profile content before use, especially when using profiles from shared repositories.
- Prefer explicit path mode only for trusted files under review.

## `--content` leaks sensitive text

- `--content` is intended for local review; output should remain local and access-limited.
- do not paste output that contains secrets, credentials, or tokens into public chat/dev logs.

## Docker validation blocked by network TLS

If build or run commands fail with registry/network TLS errors, this is commonly an environment issue (for example, restricted
egress or certificate interception) rather than a proven CLI/runtime defect.

Recommended response:

- Keep Docker instructions available for users.
- Record the exact `docker` error and network context in release notes.
- Continue validating via local CLI commands (`cargo run ...`) and smoke workflows.
