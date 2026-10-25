# Offload: licensing, source availability and trademarks

This is the review behind the decision to build Offload on RustDesk. It is not legal advice;
have a lawyer confirm it before you sell or widely distribute Offload.

## What Offload is built from

| Component | Upstream | Licence | How Offload uses it |
|---|---|---|---|
| RustDesk client 1.5.0 (`src/`, `flutter/`, `libs/base`, `libs/clipboard`, `libs/portable`, `libs/remote_printer`, `libs/virtual_display`, `res/`) | github.com/rustdesk/rustdesk | **AGPL-3.0** (`LICENCE`) | Forked and modified (this repository) |
| `libs/hbb_common` (commit 229b904) | github.com/rustdesk/hbb_common | Part of RustDesk, distributed under the same AGPL-3.0 terms | Vendored and modified (server endpoint, app identity) |
| `libs/scrap`, `libs/enigo` | RustDesk forks of quadrupleslap/scrap, enigo-rs | MIT (`libs/enigo/LICENSE`, crate metadata) | Unmodified |
| RustDesk server 1.1.16 (hbbs, hbbr) | github.com/rustdesk/rustdesk-server | AGPL-3.0 | Run unmodified from the official image `rustdesk/rustdesk-server:1.1.16` (pinned by digest) |
| `libs/offload_core`, `src/offload/`, `src/server/offload_conn.rs`, `offload-server/`, Offload Flutter pages | new in Offload | AGPL-3.0 | New code |
| Flutter engine and framework | flutter.dev, plus RustDesk's patched Windows engine build | BSD-3-Clause | Bundled in the apps |
| libvpx, libyuv, opus | webmproject, chromium, xiph | BSD-3-Clause | Statically linked |
| libaom | aomedia.org | BSD-2-Clause + AOM patent licence | Statically linked |
| FFmpeg (hardware codecs) | ffmpeg.org | LGPL-2.1-or-later (built without `--enable-gpl`/`--enable-nonfree`) | Statically linked; see below |
| libsodium (via `sodiumoxide`) | libsodium.org | ISC | Statically linked |
| Rust crates | crates.io / git | Mostly MIT / Apache-2.0 | See `THIRD_PARTY_NOTICES.md` |

`THIRD_PARTY_NOTICES.md` (generated from `cargo metadata` for the Windows and macOS targets)
lists 670+ Rust packages. None declares a GPL-family licence. Two RustDesk-organisation git
dependencies declare no licence in their manifests (`default_net`, a fork of the MIT-licensed
`default-net`, and `impersonate_system`); confirm their repositories' licences before a
commercial distribution.

Not bundled: the `usbmmidd` virtual display driver and the RustDesk printer driver that upstream's
Windows CI downloads (their redistribution terms are unclear and Offload does not need them), and
Claude Code (users install it on their own computer from Anthropic, under Anthropic's terms).

## Consequences of the AGPL

* **Offload must be distributed as free software under the AGPL-3.0.** A closed-source,
  proprietary fork of RustDesk is not permitted. If a proprietary product is ever required, it has
  to be built on a stack without AGPL code (for example a commercial SDK, or a new client on
  top of WebRTC/libwebrtc), not on this repository.
* **Corresponding source.** Everyone who receives an installer must be able to get the complete
  source of that exact version, including the build scripts. The CI builds installers only from
  this repository, the About page links to it (the workflow fills in the repository URL), and
  `LICENCE` ships inside both apps. Publish the repository (or provide a written offer and a
  source archive per release) before handing installers to anyone else.
* **Network use (AGPL section 13).** The pairing service in `offload-server/pairing` is
  AGPL-licensed code you run for others; its source is in this repository and its `/v1/info`
  endpoint identifies it. hbbs/hbbr run unmodified.
* **Notices.** Upstream copyright and licence notices are kept. Product metadata now reads
  "Copyright © 2026 Offload contributors and Purslane Tech Pte. Ltd. Licensed under the GNU
  AGPL v3", replacing upstream's "All rights reserved", which would contradict the licence.
* **Modified files are marked** by this repository's history (upstream tag `1.5.0` is the base)
  and listed in `OFFLOAD.md`.
* **FFmpeg (LGPL) linked statically** into an AGPL program is compatible; because the whole
  program's source and build scripts are provided, recipients can rebuild and relink, which is
  what the LGPL asks.

## Trademarks

"RustDesk" and its logo are trademarks of Purslane Ltd. The AGPL grants no trademark rights, so
Offload uses its own name, icon, bundle identifier (`com.offloadapp.offload`), window titles,
installer names and URL scheme (`offload://`). Translations that mention RustDesk are rewritten
to "Offload" at runtime by the upstream custom-client mechanism. Factual attribution ("Based on
RustDesk") remains, as do internal identifiers that users do not see as branding (crate names,
`librustdesk.dll`, Flutter method-channel names); renaming those is optional and riskier than it
is worth.

`com.offloadapp` is a placeholder organisation: before publishing, change `OFFLOAD_ORG`
(`libs/hbb_common/src/config.rs`) and the bundle identifiers to a domain you control.
