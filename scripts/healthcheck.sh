#!/usr/bin/env bash
set -euo pipefail

cargo run -p dn-cli -- health | grep -q "status=ok"
echo "healthcheck: ok"
