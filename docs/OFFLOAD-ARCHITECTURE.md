# Offload architecture and security model

## Decision: fork RustDesk

Offload needs a mature remote-desktop engine: screen capture on Windows (DXGI, including the
login and UAC screens via a SYSTEM service) and macOS (ScreenCaptureKit), hardware
encode/decode, input injection, clipboard, audio, multi-monitor, file transfer, a remote
terminal, NAT traversal with relay fallback, and installers. RustDesk 1.5.0 has all of these, in
Rust and Flutter (no Electron), with a self-hostable server. Writing a new stack would take years
to reach the same reliability, so Offload forks RustDesk and replaces only what the product
needs to be different:

| Area | RustDesk | Offload |
|---|---|---|
| Who may connect | anyone with the ID and a password, or a click on "Accept" | only devices paired with a one-time code and approval |
| Login proof | salted password hash | Ed25519 signature by the paired device's own key |
| Peer identity | trusted to the rendezvous server | pinned at pairing; a changed key is refused |
| Unattended access | permanent password | owner switch, off by default, bound to paired devices |
| Server settings | typed in by the user | compiled in |
| UI | ID/password, address book, many settings | My Computers, link with a code, three actions |

The licence consequences are in `LICENSING.md`.

## Components

```
 Mac (controller)                     Offload server                  Windows PC (controlled)
 ┌─────────────────────┐     ┌────────────────────────────┐     ┌────────────────────────┐
 │ Offload.app (UI)    │     │ hbbs  rendezvous  21115-6   │     │ Offload.exe (UI)       │
 │  └ librustdesk      │◄───►│ hbbr  relay       21117     │◄───►│ Offload service (SYSTEM)│
 │    └ src/offload    │     │ offload-pairing   21120     │     │  └ librustdesk          │
 └─────────────────────┘     └────────────────────────────┘     │    └ src/offload        │
            ▲  direct (hole-punched) or relayed, end-to-end encrypted   └────────────────────────┘
            └──────────────────────────────────────────────────────────────────────┘
```

* `libs/offload_core` — pairing codes, pending-pair state machine, trust store, signed payloads,
  slot records, rate limiter. No RustDesk dependencies; unit tested.
* `src/offload/` — glue: `server.rs` (controlled side), `client.rs` (controller signing),
  `pair.rs` (the side typing a code), `directory.rs` (pairing service client), `ipc.rs`
  (UI ⇄ service), `store.rs` (trust store file), `api.rs` (JSON API for Flutter).
* Hooks in RustDesk (each a few lines): `src/server/connection.rs` (pairing and login hooks,
  disconnect on revoke), `src/client.rs` (login signing, login message field), `src/client/io_loop.rs`
  and `src/port_forward.rs` (record the verified peer key), `src/ipc.rs` (one IPC message),
  `src/flutter_ffi.rs` (one FFI function), `libs/base/protos/message.proto` (three messages).
* `offload-server/pairing` — the pairing directory (Rust, axum).
* Flutter: `offload_home_page.dart`, `offload_settings_page.dart`, `offload_model.dart`.

## Identities

Each installation already has an Ed25519 key pair (RustDesk's device key), registered with hbbs.
During every connection the controlled side signs its ephemeral key exchange with it, and the
controller verifies that signature with the key hbbs vouches for (signed with the server key the
app has built in). Offload reuses this key as the device identity. Private keys never leave the
device; the trust store holds only public keys.

## Pairing

```
A = computer showing the code         B = computer the code is typed into
1. A → directory: POST /v1/slots {id: A}         ← signed {slot, id: A, expires_at}, token
   A makes secret S (4 Crockford symbols, OS CSPRNG) and shows  SLOT-S  for at most 5 minutes.
2. User types SLOT-S on B.
   B → directory: GET /v1/slots/SLOT             ← signed {slot, id: A, expires_at}
   B verifies the signature with the built-in server key.
3. B connects to A through hbbs (direct or relayed). The channel is secured with A's key,
   verified as above. A sends its login challenge (salt, challenge).
4. B → A (inside the encrypted channel): LoginRequest{ offload_pair: {code: SLOT-S, pk_B,
   name, platform, sig_B} }, sig_B = Ed25519_B("OFFLOAD/PAIR/1" ‖ A ‖ B ‖ salt ‖ challenge ‖
   code ‖ pk_B ‖ name ‖ platform).
5. A checks: a code is active and unexpired, it was not used, SLOT-S matches (constant time),
   sig_B is valid for pk_B, and at most 10 pairing requests arrived in the last minute.
   Three wrong codes kill the code. The first correct request locks the code for this
   requester only.
6. A's owner sees "<B name> wants to link with this computer.  [APPROVE] [DENY]" (2 minutes),
   with B's device ID and key fingerprint; B's link dialog shows the same two values, since
   B's name is chosen by B and cannot be trusted on its own.
7. On approval A stores {B, name, platform, pk_B} and answers {name, platform}; B stores
   {A, name, platform, pk_A} where pk_A is the key the channel was secured with.
   The code is spent and its slot is released.
```

Properties:

* The code is not a credential: it expires in 5 minutes, works once, survives 3 wrong guesses,
  and only puts a prompt in front of the owner, who must approve.
* The secret half never reaches the server. A network attacker who sees the slot still has a
  1 in 2^20 chance per guess and 3 guesses.
* The directory cannot be spoofed in transit (signed answers). A malicious *server* could at
  pairing time route B to a different device; the owner's approval prompt on A would then never
  appear for B's request, so the user notices. This is the same trust RustDesk places in its
  server for first contact; after pairing, keys are pinned and the server is no longer trusted
  for identity.

## Logging in after pairing

```
B connects to A. The channel is secured; B requires the key to equal pin(A), else it stops
("identity changed").
A → B: Hash{salt, challenge}            (fresh per connection; 160 random bits)
B → A: LoginRequest{ offload_auth: {pk_B, sig} }, sig = Ed25519_B("OFFLOAD/AUTH/1" ‖ A ‖ B ‖
       salt ‖ challenge ‖ kind ‖ pk_B), kind ∈ desktop | file | terminal | camera | forward
A: requires a secured channel, B in the trust store, pk_B == pin(B), valid sig.
   unattended on  → session starts.
   unattended off → the owner gets RustDesk's accept prompt; nobody there means no session.
   anything else  → refused ("not linked"); passwords and click-to-accept from unknown devices
                    are never offered.
```

Replay is prevented by the per-connection challenge; a signature for a file transfer cannot
open a remote desktop session (the session kind is signed).

## Unattended access and sensitive changes

* "Allow my trusted devices to connect when I am away" is stored with the trust store, off by
  default and after "Remove all".
* Turning it on asks for confirmation and then the operating system's own authentication.
  On Windows the service (LocalSystem) enforces this itself: it accepts the change only from an
  elevated caller, so the app starts an elevated helper through UAC. On macOS the app asks for
  the administrator password (or Touch ID) before sending the request; the per-user service
  cannot verify that, so a program already running as that user could turn it on — a known
  limit, the same boundary RustDesk's own settings have.
* The service refuses to turn it on while any remote session is active, so a remote user
  cannot grant it to themselves.
* No local request can add a trusted device: devices enter the store only at the end of a
  pairing exchange the service ran itself, which needs the other computer's owner to approve.
* RustDesk's "switch sides" (which authorized the reverse session by a one-time id instead of a
  device signature) is refused.
* Removing a device or all devices immediately closes their sessions. "Disconnect everyone now"
  closes all sessions.
* RustDesk's connection window stays visible during every session (active-session indicator).

## Where things are stored

| Item | Location |
|---|---|
| Device key pair | RustDesk config (`Offload.toml`) in the service's profile (Windows: `C:\Windows\ServiceProfiles\LocalService\AppData\Roaming\Offload\config`; macOS: `~/Library/Preferences/com.offloadapp.Offload`) |
| Trust store | `Offload_trust.json` next to it; public keys only; written atomically; an unreadable file grants nothing and is never silently replaced |
| Pending code | service memory only |

## Connectivity

RustDesk's rendezvous protocol registers each device with hbbs over UDP/TCP 21116 and tries,
in order, direct LAN, UDP hole punching, TCP hole punching, and IPv6, falling back to the hbbr
relay on 21117 (or WebSocket 21118/21119). All of it is end-to-end encrypted between the two
devices (X25519 key exchange authenticated by the device key, then XSalsa20-Poly1305);
the relay forwards ciphertext.
