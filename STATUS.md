# Offload status

Last updated: 2026-10-25. Base: RustDesk 1.5.0 (`git diff 1.5.0` shows every change).

Legend: **Done** = implemented and verified as stated · **Built** = implemented and compiles,
not yet run on the target OS · **Partial** · **Blocked** (with what is needed).

"Tested" means executed, and says where. **Nothing has been run on Windows or macOS yet.**
This development PC cannot build Windows binaries (the Windows SDK is missing, so MSVC cannot
link), there is no Mac, and there is no public server or GitHub repository yet. Everything
below was executed in an Ubuntu 24.04 WSL environment on this PC using the Linux build of the
same code. See *Blockers*.

## Milestones

| # | Milestone | State | Evidence |
|---|---|---|---|
| M1 | Remote desktop engine builds | **Done** (Linux) / **Built** (Win, mac in CI) | Core compiles with and without the `flutter` feature; release core binary and the Linux Flutter app built and run (WSL). Windows/macOS builds are defined in CI but have not run. |
| M2 | Two computers can connect | **Done** (Linux, one host) | e2e: TCP hole-punched direct sessions between two simulated computers through hbbs. |
| M3 | Pairing and trusted devices | **Done** (Linux, one host) | e2e: code → lookup → encrypted request → approval → mutual key pinning; single use; 3-wrong-code lockout. GUI: code dialog, approval prompt and My Computers list exercised in the Linux app (screenshots in `docs/test-results/`). |
| M4 | Bidirectional file transfer | **Built** | RustDesk's two-pane file manager (folders, multi-select, drag-and-drop, progress, speed, cancel, resume, overwrite prompts, mtime preservation) behind Offload login. Login to a file session verified (e2e). Real transfers need a signed-in desktop at both ends; untested. |
| M5 | Unattended access | **Done** (Linux) | e2e: off → no session without someone accepting; on → session starts; Remove-all turns it off. Windows UAC enforcement and macOS password prompt untested. |
| M6 | Zero-config relay infrastructure | **Done** (local) / **Blocked** (internet) | `deploy.sh` deployed hbbs + hbbr + pairing service, health checks passed; e2e session through hbbr. No public server yet. |
| M7 | Simplified branded application | **Done** (Linux) / **Built** (Win, mac) | Linux app runs: home, code dialog, approval, linked-computer list. Windows/macOS look and installers unverified. |
| M8 | macOS DMG and Windows EXE build | **Blocked** | `.github/workflows/offload-build.yml` written; needs the repository on GitHub with `OFFLOAD_RENDEZVOUS_SERVER` and `OFFLOAD_RS_PUB_KEY`. |
| M9 | Remote computing | **Done** (Linux) / **Built** (UI) | e2e: a command ran on the other computer over an authenticated terminal session and its output came back. Panel: terminal in a project folder, Run Claude Code, install Claude Code, project file transfer; terminals persist across disconnects. |
| M10 | Security review and end-to-end validation | **Partial** | Independent code review done and its findings fixed (below); unit tests; headless e2e; platform test plan 1–14 not run. |

## Test results

| Suite | Where | Result |
|---|---|---|
| `libs/offload_core` unit tests | WSL, Rust 1.81 | 34 passed, 0 failed |
| `offload-server/pairing` unit + HTTP tests | WSL, Rust 1.81 | 10 passed, 0 failed |
| `flutter analyze` (Offload and modified Dart files) | WSL, Flutter 3.24.5 | 0 errors, 0 warnings (4 upstream deprecation infos) |
| `cargo check`, default and `flutter` features; release build; `flutter build linux` | WSL | pass |
| Server deployment `deploy.sh` | WSL, Docker 29.1 | deployed; health checks passed |
| Headless e2e `offload-server/e2e/headless-e2e.sh` | WSL, one host, two Linux accounts, real server | **20 passed, 0 failed** (after the security fixes) — `docs/test-results/2026-10-25-headless-e2e.log` |
| Headless e2e against the Linux Flutter app bundle | WSL | same 20 checks passed |
| GUI pairing (Linux app shows code → second computer links → prompt → approve → listed) | WSL, Xvfb | worked; screenshots `docs/test-results/*.png` |
| Remote desktop video (probe `desktop` mode, `E2E_DESKTOP=1`) | WSL, Xvfb | **not achieved**: the session window (connection manager) that desktop and file sessions need would not start without a graphical login session. Untested until run on real desktops. |
| Test plan 1–14 on Windows 11 + Apple Silicon | — | **not run** |

### Test plan (`docs/OFFLOAD-TEST-PLAN.md`)

| # | Test | Status |
|---|---|---|
| 1 | Install on Windows | not run (no Windows build yet) |
| 2 | Install on macOS | not run (no Mac) |
| 3 | Generate a pairing code | passed on Linux (e2e + GUI) |
| 4 | Enter the code on the second machine | passed on Linux (e2e) |
| 5 | Approve and establish persistent trust | passed on Linux (e2e + GUI) |
| 6 | Control Windows from macOS | not run |
| 7 | Control macOS from Windows | not run (reverse-direction login passed on Linux) |
| 8 | Transfer files both ways | not run |
| 9 | Reconnect without pairing | passed on Linux (e2e) |
| 10 | Separate internet connections | not run (needs a public server and two networks) |
| 11 | Forced relay | passed on Linux (e2e, one host, 10 ms emulated latency) |
| 12 | Unattended access | passed on Linux (e2e) |
| 13 | Revoke a device | passed on Linux (e2e) |
| 14 | Remote computation on Windows | Linux stand-in passed |

## Security review

An independent review of the Offload code found, and these were fixed (commit "Security review fixes"):

| Finding | Fix |
|---|---|
| Any process able to use the local IPC could add a trusted device or turn on unattended access | No request adds devices any more (pairing runs inside the service and writes the result itself). On Windows the service grants unattended access only to an elevated caller (UAC). On macOS the app asks for the password first, but the per-user service cannot verify it — **known limit**. |
| RustDesk's "switch sides" authorized a session without a device signature | Refused in Offload; toolbar entry hidden |
| The approval prompt showed only the requester's self-chosen name | Prompt shows the requester's ID and key fingerprint; the link dialog shows the same values to compare |
| Login challenge was 6 characters | 160 random bits |
| Every login rewrote the trust store from a new thread | At most once per 10 minutes per device |

Checked and found sound: payload domain separation, Ed25519 handling, constant-time comparisons,
pending-pair state machine, slot record signatures, trust-store corruption handling, pairing
directory limits.

## Feature checklist

| Requirement | State |
|---|---|
| First run: LINK A COMPUTER / THIS COMPUTER'S CODE | Done (Linux) |
| Short one-time code, 5-min expiry, single use, rate limits, lockout | Done (unit + e2e) |
| "<name> wants to link with this computer" APPROVE/DENY | Done (Linux GUI) |
| Mutual identity verification, pinned keys, no code reuse as credential | Done (e2e) |
| Trusted-device list, Online/Offline | Done (Linux GUI) |
| Remote control (view, keyboard, mouse, scroll, clipboard, full screen, monitors, scaling, audio, reconnect, hardware codecs) | RustDesk features, built; untested here |
| FAST / BALANCED / HIGH QUALITY | Built (Settings default + in-session menu) |
| File manager (two panes, folders, multi-select, progress, speed, cancel, resume, conflicts) | RustDesk, built; untested here |
| Unattended access, off by default; revoke; remove all; disconnect all | Done (e2e) / Windows UAC untested |
| Remote terminal, project folder, Run Claude Code, persistent tasks, cancel (Ctrl+C) | Done (Linux) / UI built |
| Windows service, Start-menu shortcuts, uninstaller | RustDesk installer (renamed); untested |
| macOS menu bar, permissions flow, no Electron | RustDesk/Flutter; untested |
| Version reporting | `Offload --version`, Settings → About |
| Crash/diagnostic logs | RustDesk log files (`%APPDATA%\Offload\log`, `~/Library/Logs/Offload`); codes and keys are never logged; no telemetry |
| Update architecture | Partial: RustDesk's updater is disabled for custom builds; updates are new installers. A signed update feed is not built. |
| Signing / notarisation | Configured in CI, **needs certificates** |

## Blockers — what is needed from you

1. **A GitHub repository** for this code (public is simplest: AGPL source availability, and free
   macOS/Windows CI minutes). Then push the `offload` branch. I have not pushed anything.
2. **A server**: any Linux VPS with a public IP and ports TCP 21115–21120, UDP 21116 open.
   Run `offload-server/deploy.sh <host>`; it prints the two values for step 3.
3. **Repository settings**: variable `OFFLOAD_RENDEZVOUS_SERVER`, secret `OFFLOAD_RS_PUB_KEY`.
   Then run the *Offload build* workflow to get `Offload-Setup.exe` and `Offload.dmg`.
4. **Hardware tests**: run `docs/OFFLOAD-TEST-PLAN.md` on your Windows PC and Mac, including
   one test with the Mac on a different network (e.g. a phone hotspot).
5. **Signing (for trusted installers)**: an Authenticode certificate (Windows) and an Apple
   Developer ID + notarization key (macOS). Without them, installers are unsigned test builds:
   SmartScreen warns on Windows; macOS needs "Open Anyway".
6. **Organisation id**: replace the placeholder `com.offloadapp` with a domain you control
   (`OFFLOAD_ORG`, bundle identifiers) before publishing.
7. Optional, for local Windows builds on this PC: install Visual Studio Build Tools with the
   "Desktop development with C++" workload and the Windows 11 SDK.

## Known limits

* macOS: turning on unattended access is protected by the app's password prompt, not enforced
  by the service.
* hbbr 1.1.16 cannot relay for clients connecting from 127.0.0.1 and has a narrow relay pairing
  race (see `docs/OFFLOAD-SERVER.md`).
* A file listing in a file-transfer session is produced by the controlled computer's session
  window, so file transfer needs someone signed in there (the login screen alone is not enough).
* Terminal sessions run as the signed-in user; RustDesk's "terminal as administrator" login is
  not offered in Offload.
* If a computer is reinstalled it gets a new identity; link it again.
