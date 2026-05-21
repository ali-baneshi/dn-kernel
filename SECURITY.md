# Security Policy

## Supported versions

`dn-kernel` is currently pre-release. Security support is best effort; we fix
important issues as they are reported and prioritize fixes for release-impacting
defects in the next release.

## Reporting vulnerabilities

If you believe you found a security issue:

1. Avoid posting it publicly (including issue trackers) until we confirm it.
2. Send details by email (or a private channel used by this project/community).
3. Include:
   - version (or commit SHA),
   - minimal reproduction steps,
   - affected command/profile path,
   - whether the issue can be reproduced with untrusted inputs.

We will acknowledge reports promptly and aim for a practical fix path with clear
verification steps.

## Expected-risk areas

- `--content` previews can include sensitive text. Treat preview output as secret-bearing.
- provider/worker integrations may call local tools or endpoints depending on profile settings.
- scanning large or adversarial repositories can expose CPU/memory pressure; raise limits explicitly if needed.
- JSON/Markdown output includes file paths and finding metadata; treat scan reports as potentially sensitive.
- Running scans in containers does not expand trust boundaries for mounted paths; mounted host directories should remain approved
  and free of credentials unless explicitly intended.

## Hardening recommendations for users

- Keep profile files under version control.
- Avoid running untrusted profiles from unknown locations.
- Review profile settings before enabling `ai` and provider integrations.
- Use `--json` in CI to capture `errors` and verify worker/provider health.
- Be careful when running containerized scans over mounted host paths: treat mounted sources as trusted input and do not mount secrets unless expected.
