# Offload status

Last updated: 2026-10-25. Base: RustDesk 1.5.0 (`git diff 1.5.0` shows every change).

Legend: **Done** = implemented and verified as stated · **Built** = implemented and compiles,
not yet run on the target OS · **Partial** · **Blocked** (with what is needed).

"Tested" means executed, and says where. **Nothing has been run on Windows or macOS yet.**
This development machine cannot build Windows binaries (the Windows SDK is not installed, so
MSVC cannot link), there is no Mac, and there is no public server or GitHub repository yet.
All execution so far happened in an Ubuntu 24.04 WSL environment on this PC, using the Linux
build of the same code. See *Blockers*.

## Milestones

| # | Milestone | State | Evidence |
|---|---|---|---|
| M1 | Remote desktop engine builds | **Built** (Linux) | Core compiles with and without the `flutter` feature; release core binary and Linux Flutter app built (WSL). Windows/macOS builds are defined in CI but have not run. |
| M2 | Two computers can connect | **Done** (Linux, one host) | e2e: TCP hole-punched direct sessions between two simulated computers through hbbs. |
| M3 | Pairing and trusted devices | **Done** (Linux, one host) | e2e: code → lookup → encrypted request → approval prompt → mutual key pinning; single use; 3-wrong-code lockout. |
| M4 | Bidirectional file transfer | **Built** | RustDesk's two-pane file manager (folders, multi-select, drag-and-drop, progress, speed, cancel, resume, overwrite prompts) now behind Offload login. Login to a file session verified (e2e); actual transfers need a signed-in desktop on both ends (the listing is served by the session window), so they are untested. |
| M5 | Unattended access | **Done** (Linux) | e2e: off → no session without someone accepting; on → session starts; Remove-all turns it off. OS-authentication prompt for turning it on is Windows/macOS UI, untested. |
| M6 | Zero-config relay infrastructure | **Done** (local) / **Blocked** (internet) | `deploy.sh` deployed hbbs + hbbr + pairing service and passed its health checks; e2e session through hbbr. No public server yet. |
| M7 | Simplified branded application | **Built** | Home, linking, code/approval, remote computing and settings screens; `flutter analyze` clean; icons/names/bundle ids replaced. |
| M8 | macOS DMG and Windows EXE build | **Blocked** | Workflow `.github/workflows/offload-build.yml` written; needs the repository on GitHub with the two server settings. |
| M9 | Remote computing | **Done** (Linux) / **Built** (UI) | e2e: command executed on the other computer over an authenticated terminal session and its output returned. Panel: terminal in a project folder, Run Claude Code, install Claude Code, project file transfer; terminals persist across disconnects. |
| M10 | Security review and end-to-end validation | **Partial** | Protocol in `docs/OFFLOAD-ARCHITECTURE.md`; unit tests; headless e2e; independent code review (see below). Platform test plan 1–14 not run. |

## Test results

| Suite | Where | Result |
|---|---|---|
| `libs/offload_core` unit tests | WSL, Rust 1.81 | 33 passed, 0 failed |
| `offload-server/pairing` unit + HTTP tests | WSL, Rust 1.81 | 10 passed, 0 failed |
| `flutter analyze` (Offload and modified Dart files) | WSL, Flutter 3.24.5 | 0 errors / 0 warnings (4 upstream deprecation infos) |
| `cargo check` core, default and `flutter` features | WSL, Rust 1.81 | pass |
| Server deployment `deploy.sh` | WSL, Docker 29.1 | deployed; health checks passed |
| Headless e2e `offload-server/e2e/headless-e2e.sh` | WSL, one host, two Linux accounts, real server | **20 passed, 0 failed** — log: `docs/test-results/2026-10-25-headless-e2e.log` |
| Test plan 1–14 (`docs/OFFLOAD-TEST-PLAN.md`) on Windows 11 + Apple Silicon | — | **not run** |

### Test plan coverage so far

| # | Test | Status |
|---|---|---|
| 1 | Install on Windows | not run (no Windows build yet) |
| 2 | Install on macOS | not run (no Mac) |
| 3 | Generate a pairing code | passed on Linux (e2e) |
| 4 | Enter the code on the second machine | passed on Linux (e2e) |
| 5 | Approve and establish persistent trust | passed on Linux (e2e) |
| 6 | Control Windows from macOS | not run |
| 7 | Control macOS from Windows | not run (reverse-direction login passed on Linux) |
| 8 | Transfer files both ways | not run |
| 9 | Reconnect without pairing | passed on Linux (e2e) |
| 10 | Separate internet connections | not run (needs a public server and two networks) |
| 11 | Forced relay | passed on Linux (e2e, one host) |
| 12 | Unattended access | passed on Linux (e2e) |
| 13 | Revoke a device | passed on Linux (e2e) |
| 14 | Remote computation on Windows | Linux stand-in passed (command ran on the other computer) |
