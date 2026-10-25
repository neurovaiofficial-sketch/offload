# Building Offload

## Release builds (GitHub Actions)

`.github/workflows/offload-build.yml` builds everything on GitHub's runners:

| Job | Runner | Output |
|---|---|---|
| Offload unit tests | ubuntu-22.04 | `offload_core` and pairing service tests |
| Pairing service image | ubuntu-22.04 | Docker image build check |
| Windows x64 | windows-2022 | `Offload-Setup.exe` (+ `.sha256`, signing state) |
| macOS Apple Silicon | macos-14 | `Offload.dmg` (+ `.sha256`, signing state) |
| Draft release | on tags `offload-v*` | a draft GitHub release with the installers |

Required repository settings (Settings → Secrets and variables → Actions):

| Name | Kind | Value |
|---|---|---|
| `OFFLOAD_RENDEZVOUS_SERVER` | variable | your server's public host |
| `OFFLOAD_RS_PUB_KEY` | secret | `data/id_ed25519.pub` from the server |

Optional signing (without them builds are unsigned and labelled so):

| Name | Purpose |
|---|---|
| `WINDOWS_PFX_BASE64`, `WINDOWS_PFX_PASSWORD` | Authenticode code-signing certificate (OV or EV) as base64 .pfx. EV certificates on hardware tokens need a cloud signing service instead (e.g. Azure Trusted Signing); replace the signtool step accordingly. |
| `MACOS_P12_BASE64`, `MACOS_P12_PASSWORD` | "Developer ID Application" certificate (Apple Developer Program, US$99/year) |
| `MACOS_CODESIGN_IDENTITY` | e.g. `"Developer ID Application: Your Name (TEAMID)"` |
| `MACOS_NOTARIZE_JSON` | base64 App Store Connect API key JSON for `rcodesign notary-submit` |

Without a Developer ID, macOS shows "cannot be opened" on first launch (the user must allow it
in Privacy & Security); without Authenticode, Windows SmartScreen warns. Unsigned builds are for
testing, not for distribution as trusted installers.

### Versions

The engine version stays RustDesk's (1.5.0) because peers use it for feature negotiation.
`libs/offload_core` carries Offload's own version (0.1.0). Updates: RustDesk's updater is
disabled for custom-named builds; ship updates as new installers (Windows: running a newer
`Offload-Setup.exe` upgrades in place; macOS: replace the app). A signed update channel is a
remaining task.

## Local development

The Windows and macOS builds need the same toolchains as upstream RustDesk (Rust 1.75/1.81,
LLVM, vcpkg, Flutter 3.24.5, Visual Studio 2022 with the Windows SDK, or Xcode). See
`docs/CONTRIBUTING.md` upstream and the workflow for exact steps.

On Linux (also inside WSL) the Offload code can be built and tested without vcpkg:

```bash
sudo apt install build-essential clang libclang-dev pkg-config nasm libgtk-3-dev libxcb-randr0-dev \
  libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev \
  libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libpam0g-dev libva-dev libvdpau-dev \
  libvpx-dev libyuv-dev libopus-dev libaom-dev
# Ubuntu's libyuv-dev has no pkg-config file: create ~/pkgconfig/libyuv.pc (see below)
cargo test --manifest-path libs/offload_core/Cargo.toml
cargo test --manifest-path offload-server/pairing/Cargo.toml
OFFLOAD_RS_PUB_KEY=<test server key> PKG_CONFIG_PATH=~/pkgconfig \
  cargo build --release --features linux-pkg-config --bin rustdesk
```

`~/pkgconfig/libyuv.pc`:

```
prefix=/usr
includedir=/usr/include
libdir=/usr/lib/x86_64-linux-gnu
Name: libyuv
Description: YUV conversion and scaling
Version: 0.0.1
Libs: -L${libdir} -lyuv
Cflags: -I${includedir}
```

### End-to-end test against a local server

```bash
cd offload-server
sudo ./deploy.sh 127.0.0.1                       # prints OFFLOAD_RS_PUB_KEY
# build the binary with that key (above), then:
sudo ./e2e/headless-e2e.sh ~/target/release/rustdesk
```

### Command line

Every installed Offload also answers on the command line (for support and scripting):

```
Offload --offload info
Offload --offload devices
Offload --offload pair_start          # then pair_wait / pair_decide {"id":..,"approve":true}
Offload --offload link '{"code":"K7QM-4ZP9"}'
Offload --offload probe '{"id":"<peer id>","mode":"files"}'
Offload --offload probe '{"id":"<peer id>","mode":"run","command":"nproc"}'
Offload --offload unattended_set '{"on":false}'
```

### Icons

`res/offload/gen-icons.sh` renders every icon from `res/offload/*.svg` (needs rsvg-convert and
ImageMagick); the outputs are committed.

### Notices

`cargo metadata --format-version 1 | python3 res/offload/third_party_notices.py > THIRD_PARTY_NOTICES.md`
