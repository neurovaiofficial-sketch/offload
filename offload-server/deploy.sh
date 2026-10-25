#!/bin/bash
# Installs or updates the Offload server on a Linux host (tested on Ubuntu 22.04/24.04).
#
#   sudo ./deploy.sh <public-host>
#
# <public-host> is the DNS name or public IP address the apps will use, e.g. offload.example.com.
# The host needs these ports reachable from the internet:
#   TCP 21115, 21116, 21117, 21118, 21119, 21120 and UDP 21116.
# On success it prints the two values to build into the apps.
set -euo pipefail

host="${1:-}"
if [[ -z "$host" ]]; then
  echo "usage: sudo $0 <public-host>" >&2
  exit 2
fi
if [[ $EUID -ne 0 ]]; then
  echo "run as root (sudo)" >&2
  exit 2
fi
case "$host" in
  *[!A-Za-z0-9.:-]*) echo "invalid host: $host" >&2; exit 2 ;;
esac

src="$(cd "$(dirname "$0")" && pwd)"
dest="${OFFLOAD_SERVER_DIR:-/opt/offload-server}"

echo "== checking dependencies"
if ! command -v docker >/dev/null 2>&1; then
  if command -v apt-get >/dev/null 2>&1; then
    apt-get update -y
    apt-get install -y docker.io docker-compose-v2 || apt-get install -y docker.io docker-compose-plugin
    systemctl enable --now docker
  else
    echo "Docker is not installed and this is not an apt-based system; install Docker and Docker Compose, then rerun." >&2
    exit 1
  fi
fi
if ! docker compose version >/dev/null 2>&1; then
  echo "Docker Compose v2 ('docker compose') is required." >&2
  exit 1
fi

echo "== installing to $dest"
mkdir -p "$dest/data"
chmod 700 "$dest/data"
"$src/build-pairing-context.sh"
rm -rf "$dest/pairing"
cp -r "$src/pairing" "$dest/pairing"
cp "$src/docker-compose.yml" "$dest/docker-compose.yml"
printf 'OFFLOAD_PUBLIC_HOST=%s\n' "$host" > "$dest/.env"

if command -v ufw >/dev/null 2>&1 && ufw status | grep -q "Status: active"; then
  echo "== opening firewall ports (ufw)"
  ufw allow 21115:21120/tcp
  ufw allow 21116/udp
fi

echo "== starting"
cd "$dest"
docker compose up -d --build

echo "== waiting for the server key"
for _ in $(seq 1 60); do
  [[ -s "$dest/data/id_ed25519.pub" ]] && break
  sleep 1
done
if [[ ! -s "$dest/data/id_ed25519.pub" ]]; then
  echo "hbbs did not create its key; see: docker compose -f $dest/docker-compose.yml logs hbbs" >&2
  exit 1
fi
chmod 600 "$dest/data/id_ed25519"

echo "== health checks"
ok=1
for _ in $(seq 1 30); do
  curl -fsS "http://127.0.0.1:21120/healthz" >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS "http://127.0.0.1:21120/healthz" >/dev/null || { echo "pairing service is not answering"; ok=0; }
for p in 21115 21116 21117 21120; do
  if ! timeout 3 bash -c "</dev/tcp/127.0.0.1/$p" 2>/dev/null; then
    echo "nothing is listening on TCP $p"; ok=0
  fi
done
served_key="$(curl -fsS http://127.0.0.1:21120/v1/info | sed -n 's/.*"public_key":"\([^"]*\)".*/\1/p')"
pub="$(cat "$dest/data/id_ed25519.pub")"
if [[ "$served_key" != "$pub" ]]; then
  echo "pairing service signs with a different key than hbbs ($served_key vs $pub)"; ok=0
fi

echo
echo "Offload server is $( [[ $ok == 1 ]] && echo running || echo NOT healthy ) on $host."
echo "Build the apps with these values (GitHub: Settings > Secrets and variables > Actions):"
echo "  OFFLOAD_RENDEZVOUS_SERVER=$host"
echo "  OFFLOAD_RS_PUB_KEY=$pub"
echo "Back up $dest/data/id_ed25519: it is the server's identity."
[[ $ok == 1 ]]
