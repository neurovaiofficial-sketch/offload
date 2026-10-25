# Offload

Offload lets you use one of your computers from another — anywhere, without port forwarding,
IP addresses or VPN accounts:

**INSTALL → OPEN → ENTER CODE → APPROVE → CONNECT.**

It is built on [RustDesk](https://github.com/rustdesk/rustdesk) 1.5.0 (AGPL-3.0) and adds its
own pairing system, trusted devices, unattended access, a simplified interface and a remote
computing panel. Offload is free software under the GNU AGPL v3.

| Document | |
|---|---|
| [docs/INSTALL.md](docs/INSTALL.md) | installing and using the apps (for everyone) |
| [docs/OFFLOAD-SERVER.md](docs/OFFLOAD-SERVER.md) | running the server and configuring builds |
| [docs/OFFLOAD-ARCHITECTURE.md](docs/OFFLOAD-ARCHITECTURE.md) | design, pairing protocol, security model |
| [docs/OFFLOAD-BUILD.md](docs/OFFLOAD-BUILD.md) | building, signing, CI, local development |
| [docs/OFFLOAD-TEST-PLAN.md](docs/OFFLOAD-TEST-PLAN.md) | end-to-end test plan |
| [STATUS.md](STATUS.md) | what works, what is tested, what is blocked |
| [LICENSING.md](LICENSING.md) | licences, source obligations, trademarks |

## What changed from RustDesk

New code:

* `libs/offload_core/` — pairing codes, trust store, signed payloads, pairing state machine
* `src/offload/` — pairing, device-signature login, trust store service, CLI and UI API
* `src/server/offload_conn.rs` — the controlled side's pairing and login path
* `offload-server/` — pairing directory service, Docker Compose deployment, scripts, end-to-end test
* `flutter/lib/desktop/pages/offload_*.dart`, `flutter/lib/models/offload_model.dart` — UI
* `.github/workflows/offload-build.yml` — Windows and macOS installers
* `res/offload/` — icons and their generator

Modified upstream files (small hooks): `src/server/connection.rs`, `src/client.rs`,
`src/client/io_loop.rs`, `src/port_forward.rs`, `src/ipc.rs`, `src/flutter_ffi.rs`, `src/lib.rs`,
`src/common.rs`, `src/core_main.rs`, `src/hbbs_http.rs`, `src/platform/macos.rs`,
`libs/base/protos/message.proto`, `libs/hbb_common/src/config.rs`, `libs/portable/src/main.rs`,
`build.py`, Flutter `desktop_home_page.dart`, `desktop_tab_page.dart`, `desktop_setting_page.dart`,
`install_page.dart`, `terminal_model.dart`, `consts.dart`, and the Windows/macOS runner
metadata. `git diff 1.5.0` shows every change.

The original RustDesk README is kept as [README.md](README.md).
