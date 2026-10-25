# Running the Offload server

Offload needs one small server on the internet. It does three jobs:

* **Rendezvous (hbbs)** — your computers register here, find each other, and try to connect
  directly (hole punching through home routers and most carrier-grade NAT).
* **Relay (hbbr)** — when a direct connection is impossible, traffic goes through here. It stays
  end-to-end encrypted between your computers; the relay cannot read it.
* **Pairing directory** — maps the first half of a pairing code to the computer showing it.

Nobody needs to configure anything in the apps: the server's address and public key are built
into them.

## 1. Get a server

Any Linux VPS with a public IPv4 address works (1 vCPU / 1 GB RAM is plenty for personal use).
Relay traffic is the main cost: a remote desktop session uses roughly 1–10 Mbit/s while it is
relayed; direct connections use none.

Give it a DNS name (for example `offload.example.com`) or use its IP address.

Open these ports in the provider's firewall:

| Port | Protocol | Purpose |
|---|---|---|
| 21115 | TCP | NAT type test |
| 21116 | TCP + UDP | rendezvous / hole punching |
| 21117 | TCP | relay |
| 21118, 21119 | TCP | WebSocket rendezvous / relay (optional, for restrictive networks) |
| 21120 | TCP | pairing directory |

## 2. Install

Copy the `offload-server` folder of this repository to the server and run:

```bash
sudo ./deploy.sh offload.example.com
```

It installs Docker if needed, starts the three services, checks them, and prints:

```
OFFLOAD_RENDEZVOUS_SERVER=offload.example.com
OFFLOAD_RS_PUB_KEY=<base64 key>
```

Keep `/opt/offload-server/data/id_ed25519` secret and back it up. If it is lost, every app has
to be rebuilt with the new key and every computer paired again.

## 3. Build the apps for this server

In the GitHub repository: **Settings → Secrets and variables → Actions**

* Variables: `OFFLOAD_RENDEZVOUS_SERVER` = the host above
* Secrets: `OFFLOAD_RS_PUB_KEY` = the key above

Then run the **Offload build** workflow (Actions tab → Offload build → Run workflow). Its
artifacts are `Offload-Setup.exe` and `Offload.dmg`.

## 4. Check it from outside

From any other machine:

```bash
./check.sh offload.example.com <base64 key>
```

## Operations

* Logs: `cd /opt/offload-server && docker compose logs -f`
* Update: copy a newer `offload-server` folder and rerun `deploy.sh` (data and key are kept).
* Stop: `docker compose down` in `/opt/offload-server`.
* The server holds no recordings or file contents. hbbs keeps device ids, public keys and last
  known addresses in `data/db_v2.sqlite3`; the pairing directory keeps codes for at most five
  minutes, in memory only.

## Known server limitations (upstream hbbr 1.1.16)

* hbbr treats connections from `127.0.0.1` as its admin console, so the relay cannot be used by
  an Offload app running on the server machine itself (use the server's real address).
* hbbr pairs the two ends of a relayed session with a non-atomic lookup-then-insert. If both
  ends arrive within microseconds of each other that relay attempt fails (connecting again
  works).
  On real networks the second end arrives at least one round trip later, so this is rare; it
  was reproduced only with both ends on one machine. A fixed hbbr would need a fork of
  rustdesk-server.

## Security notes

* hbbs and hbbr run with `-k _`: only apps built with this server's key can use it.
* Apps pin each paired computer's identity key, so after pairing even a compromised server
  cannot impersonate one of your computers; it could only refuse service.
* The pairing directory signs every answer with the server key, so a network attacker cannot
  point a pairing at another computer. The secret half of a code never reaches the server.
* The directory rate-limits by client address (10 codes and 30 lookups per minute).
