"""Writes THIRD_PARTY_NOTICES.md from `cargo metadata` output.

usage: cargo metadata --format-version 1 | python3 res/offload/third_party_notices.py > THIRD_PARTY_NOTICES.md

Lists every Rust package in the dependency graph of the app with its declared licence and
source. Flutter adds the licences of every Dart package and of the engine to the app bundle
itself (flutter_assets/NOTICES.Z), so they are not repeated here.
"""
import json
import sys

HEADER = """# Third-party software in Offload

Offload is licensed under the GNU AGPL v3 (see `LICENCE`) and is based on RustDesk. It includes
the following Rust packages. Each remains under its own licence; full texts are in each
package's source, available from the listed location or from crates.io.

Native libraries linked into the app: libvpx, libyuv, opus (BSD-3-Clause); libaom
(BSD-2-Clause and the AOM patent licence); FFmpeg (LGPL-2.1-or-later); libsodium (ISC);
Intel Media SDK dispatcher (MIT); AMD AMF and NVIDIA codec headers (MIT). The Flutter engine and
framework (BSD-3-Clause) and the Dart packages the UI uses are listed in the NOTICES file Flutter
bundles with the app.

| Package | Version | Licence | Source |
|---|---|---|---|
"""


def main() -> None:
    meta = json.load(sys.stdin)
    workspace = set(meta.get("workspace_members", []))
    rows = []
    for p in meta["packages"]:
        if p["id"] in workspace:
            continue
        lic = p.get("license") or ("see " + p["license_file"] if p.get("license_file") else "unspecified")
        src = p.get("repository") or p.get("homepage") or (p.get("source") or "").replace("registry+", "")
        rows.append((p["name"].lower(), p["name"], p["version"], lic, src))
    out = [HEADER]
    for _, name, version, lic, src in sorted(set(rows)):
        out.append(f"| {name} | {version} | {lic.replace('|', '/')} | {src} |\n")
    sys.stdout.write("".join(out))


if __name__ == "__main__":
    main()
