#!/bin/bash
# Checks an Offload server from another machine (run it from outside the server's network).
#
#   ./check.sh <public-host> [expected-public-key]
set -uo pipefail
host="${1:?usage: $0 <public-host> [expected-public-key]}"
want="${2:-}"
fail=0

for p in 21115 21116 21117 21120; do
  if timeout 5 bash -c "</dev/tcp/$host/$p" 2>/dev/null; then
    echo "ok   TCP $p"
  else
    echo "FAIL TCP $p is not reachable"; fail=1
  fi
done

info="$(curl -fsS --max-time 5 "http://$host:21120/v1/info" || true)"
if [[ -z "$info" ]]; then
  echo "FAIL pairing service did not answer"; fail=1
else
  key="$(printf '%s' "$info" | sed -n 's/.*"public_key":"\([^"]*\)".*/\1/p')"
  echo "ok   pairing service, server key $key"
  if [[ -n "$want" && "$key" != "$want" ]]; then
    echo "FAIL server key differs from the one expected"; fail=1
  fi
fi
echo "note UDP 21116 cannot be checked from a shell; if it is blocked, connections still work but fall back to the relay more often."
exit $fail
