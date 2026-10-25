# Offload end-to-end test plan

A test passes only when it was executed on the machines it names. Record results in
`STATUS.md` with the date, build (commit, workflow run), machines and network.

**Setup.** A deployed Offload server (`docs/OFFLOAD-SERVER.md`); installers from one workflow
run built for that server; a Windows 11 x64 PC ("PC") and an Apple Silicon Mac ("Mac"); for
tests 10–11 the two on different internet connections (e.g. the Mac on a phone hotspot).

| # | Test | Steps | Pass when |
|---|---|---|---|
| 1 | Install on Windows | Run `Offload-Setup.exe` → Install. | Offload is in Start menu and Apps; service "Offload" running (`sc query Offload`); app shows the home screen and "Ready". |
| 2 | Install on macOS | Open `Offload.dmg`, drag to Applications, open; grant Screen Recording + Accessibility; install the helper when asked. | App opens; menu bar icon; "Ready"; `launchctl list \| grep offload` shows the agent. |
| 3 | Generate a pairing code | PC: THIS COMPUTER'S CODE. | An `XXXX-XXXX` code and a 5:00 countdown appear. |
| 4 | Enter the code | Mac: LINK A COMPUTER → type the code. | Mac shows "Waiting for approval…"; PC shows "MacBook … wants to link with this computer." |
| 5 | Approve, persistent trust | PC: APPROVE. Quit and reopen both apps; reboot the PC. | Both list each other under My Computers after restart. Typing the same code again fails ("already been used"). |
| 6 | Control Windows from macOS | Mac: select PC → CONNECT; PC: Accept (unattended off). Type, click, scroll with the trackpad, copy/paste text both ways, full screen and back, switch monitors (if two), try FAST/BALANCED/HIGH QUALITY. | All work; PC shows the session window while connected. |
| 7 | Control macOS from Windows | PC: select Mac → CONNECT; Mac: Accept. Same checks. | All work. |
| 8 | Transfer files both ways | Mac: TRANSFER FILES. Copy a folder with subfolders and a >2 GB file to the PC; copy files back; copy a file onto an existing name. | Contents and sizes match (compare `shasum -a 256` / `Get-FileHash`); progress and speed shown; existing file prompts before replacing; cancel stops a transfer. |
| 9 | Reconnect without pairing | Close the session; disconnect Wi-Fi for 30 s mid-session; reconnect. | Reconnects without a code; mid-session drop recovers automatically. |
| 10 | Separate internet connections | Mac on a phone hotspot, PC on home broadband; CONNECT and TRANSFER FILES. | Works; note "direct" vs "relay" in the session's connection info. |
| 11 | Forced relay | Mac: in the session toolbar (or Advanced settings) enable "Always connect via relay"; connect. Or CLI: `Offload --offload probe '{"id":"<PC id>","mode":"files","relay":true}'`. | Works; connection shows "Relay"; `docker compose logs hbbr` shows the relayed session. |
| 12 | Unattended access | PC: Settings → enable unattended (confirm + UAC). Lock the PC / sign out; from the Mac CONNECT. | Session starts without anyone at the PC, including the Windows lock/login screen. With it off, the same attempt waits for Accept and times out. |
| 13 | Revoke | PC: Settings → Trusted devices → Remove Mac, while the Mac is connected. | Mac's session ends at once; new attempts say "not linked". Re-linking with a new code works. |
| 14 | Remote computation on Windows | Mac: REMOTE COMPUTING → project folder → Open a terminal; run e.g. `python -c "import os; print(os.cpu_count())"` or a build; then Run Claude Code (after installing it). Close the Mac's session window mid-task and reopen. | Output shows the PC's CPU count/results; Task Manager on the PC shows the work; the terminal is still running after reopening; Ctrl+C cancels. |

## Security checks (run with the above)

* Pairing: three wrong codes cancel the code; a code expires after 5 minutes; a code shown on
  the PC cannot be approved from the Mac (the prompt only appears on the PC).
* While a remote session is active, try to enable unattended access from inside it:
  it must be refused ("only while nobody is connected remotely").
* Connect from a stock RustDesk client or an Offload build with a different server key: refused.
* Reinstall Offload on the PC (new identity key) without re-linking: the Mac refuses to connect
  ("identity has changed").

## Automated coverage

* `cargo test` in `libs/offload_core` and `offload-server/pairing` (CI job "Offload unit tests").
* `offload-server/e2e/headless-e2e.sh` — two simulated devices on one Linux host against a real
  server: pairing, approval, single use, attended refusal, unattended login, a remote command,
  forced relay, reconnect, reverse direction, wrong-code lockout, revocation, remove-all.
  It covers tests 3, 4, 5, 9, 11, 12, 13 and a Linux stand-in for 7 and 14. It does not
  replace the Windows/macOS runs above. Pass the Linux app bundle directory instead of the
  binary and set `E2E_DESKTOP=1` on a Linux machine with a graphical login to add a remote
  desktop streaming check.
