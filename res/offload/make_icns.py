"""Builds a macOS .icns from PNG renders (PNG-in-ICNS, supported since macOS 10.7).

usage: make_icns.py OUT.icns DIR   where DIR holds icon_<size>.png for 16..1024
"""
import pathlib
import struct
import sys

# ICNS type -> pixel size of the PNG it holds.
TYPES = [
    (b"icp4", 16), (b"icp5", 32), (b"icp6", 64), (b"ic07", 128),
    (b"ic08", 256), (b"ic09", 512), (b"ic10", 1024),
    (b"ic11", 32), (b"ic12", 64), (b"ic13", 256), (b"ic14", 512),
]


def main() -> None:
    out, src = pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])
    chunks = b""
    for kind, size in TYPES:
        data = (src / f"icon_{size}.png").read_bytes()
        chunks += kind + struct.pack(">I", len(data) + 8) + data
    out.write_bytes(b"icns" + struct.pack(">I", len(chunks) + 8) + chunks)


if __name__ == "__main__":
    main()
