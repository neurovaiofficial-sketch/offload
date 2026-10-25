# Third-party software in Offload

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
| addr2line | 0.22.0 | Apache-2.0 OR MIT | https://github.com/gimli-rs/addr2line |
| adler | 1.0.2 | 0BSD OR MIT OR Apache-2.0 | https://github.com/jonas-schievink/adler.git |
| adler2 | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | https://github.com/oyvindln/adler2 |
| aead | 0.5.2 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| aes | 0.8.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/block-ciphers |
| aes-gcm | 0.10.3 | Apache-2.0 OR MIT | https://github.com/RustCrypto/AEADs |
| aho-corasick | 1.1.3 | Unlicense OR MIT | https://github.com/BurntSushi/aho-corasick |
| alloc-no-stdlib | 2.0.4 | BSD-3-Clause | https://github.com/dropbox/rust-alloc-no-stdlib |
| alloc-stdlib | 0.2.2 | BSD-3-Clause | https://github.com/dropbox/rust-alloc-no-stdlib |
| ansi_term | 0.12.1 | MIT | https://github.com/ogham/rust-ansi-term |
| anstream | 0.6.14 | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| anstyle | 1.0.13 | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| anstyle-parse | 0.2.4 | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| anstyle-query | 1.1.0 | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle |
| anstyle-wincon | 3.0.3 | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle.git |
| anyhow | 1.0.98 | MIT OR Apache-2.0 | https://github.com/dtolnay/anyhow |
| arboard | 3.4.0 | MIT OR Apache-2.0 | https://github.com/1Password/arboard |
| arc-swap | 1.7.1 | MIT OR Apache-2.0 | https://github.com/vorner/arc-swap |
| arrayref | 0.3.9 | BSD-2-Clause | https://github.com/droundy/arrayref |
| arrayvec | 0.7.6 | MIT OR Apache-2.0 | https://github.com/bluss/arrayvec |
| asn1-rs | 0.6.2 | MIT OR Apache-2.0 | https://github.com/rusticata/asn1-rs.git |
| asn1-rs-derive | 0.5.1 | MIT OR Apache-2.0 | https://github.com/rusticata/asn1-rs.git |
| asn1-rs-impl | 0.2.0 | MIT/Apache-2.0 | https://github.com/rusticata/asn1-rs.git |
| associative-cache | 1.0.1 | MIT/Apache-2.0 | https://github.com/fitzgen/associative-cache |
| async-compression | 0.4.32 | MIT OR Apache-2.0 | https://github.com/Nullus157/async-compression |
| async-recursion | 1.1.1 | MIT OR Apache-2.0 | https://github.com/dcchut/async-recursion |
| async-trait | 0.1.80 | MIT OR Apache-2.0 | https://github.com/dtolnay/async-trait |
| atomic-waker | 1.1.2 | Apache-2.0 OR MIT | https://github.com/smol-rs/atomic-waker |
| atty | 0.2.14 | MIT | https://github.com/softprops/atty |
| auto_impl | 1.3.0 | MIT OR Apache-2.0 | https://github.com/auto-impl-rs/auto_impl/ |
| autocfg | 0.1.8 | Apache-2.0 OR MIT | https://github.com/cuviper/autocfg |
| autocfg | 1.3.0 | Apache-2.0 OR MIT | https://github.com/cuviper/autocfg |
| backtrace | 0.3.73 | MIT OR Apache-2.0 | https://github.com/rust-lang/backtrace-rs |
| base16ct | 0.2.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/base16ct |
| base32 | 0.4.0 | MIT OR Apache-2.0 | https://github.com/andreasots/base32 |
| base64 | 0.21.7 | MIT OR Apache-2.0 | https://github.com/marshallpierce/rust-base64 |
| base64 | 0.22.1 | MIT OR Apache-2.0 | https://github.com/marshallpierce/rust-base64 |
| base64ct | 1.6.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/base64ct |
| bincode | 1.3.3 | MIT | https://github.com/servo/bincode |
| bindgen | 0.59.2 | BSD-3-Clause | https://github.com/rust-lang/rust-bindgen |
| bindgen | 0.69.4 | BSD-3-Clause | https://github.com/rust-lang/rust-bindgen |
| bindgen | 0.72.1 | BSD-3-Clause | https://github.com/rust-lang/rust-bindgen |
| bit_field | 0.10.2 | Apache-2.0/MIT | https://github.com/phil-opp/rust-bit-field |
| bitflags | 1.3.2 | MIT/Apache-2.0 | https://github.com/bitflags/bitflags |
| bitflags | 2.9.1 | MIT OR Apache-2.0 | https://github.com/bitflags/bitflags |
| block | 0.1.6 | MIT | http://github.com/SSheldon/rust-block |
| block-buffer | 0.10.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| block-padding | 0.3.3 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| block2 | 0.5.1 | MIT | https://github.com/madsmtm/objc2 |
| block2 | 0.6.2 | MIT | https://github.com/madsmtm/objc2 |
| brotli | 3.5.0 | BSD-3-Clause OR MIT | https://github.com/dropbox/rust-brotli |
| brotli-decompressor | 2.5.1 | BSD-3-Clause/MIT | https://github.com/dropbox/rust-brotli-decompressor |
| bytecodec | 0.4.15 | MIT | https://github.com/sile/bytecodec |
| bytemuck | 1.23.2 | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/bytemuck |
| bytemuck_derive | 1.10.1 | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/bytemuck |
| byteorder | 1.5.0 | Unlicense OR MIT | https://github.com/BurntSushi/byteorder |
| bytes | 1.11.1 | MIT | https://github.com/tokio-rs/bytes |
| cbc | 0.1.2 | MIT OR Apache-2.0 | https://github.com/RustCrypto/block-modes |
| cc | 1.2.13 | MIT OR Apache-2.0 | https://github.com/rust-lang/cc-rs |
| ccm | 0.5.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/AEADs/tree/master/ccm |
| cexpr | 0.6.0 | Apache-2.0/MIT | https://github.com/jethrogb/rust-cexpr |
| cfg-if | 0.1.10 | MIT/Apache-2.0 | https://github.com/alexcrichton/cfg-if |
| cfg-if | 1.0.0 | MIT/Apache-2.0 | https://github.com/alexcrichton/cfg-if |
| cfg_aliases | 0.1.1 | MIT | https://github.com/katharostech/cfg_aliases |
| cfg_aliases | 0.2.1 | MIT | https://github.com/katharostech/cfg_aliases |
| chrono | 0.4.41 | MIT OR Apache-2.0 | https://github.com/chronotope/chrono |
| cidr-utils | 0.5.11 | MIT | https://github.com/magiclen/cidr-utils |
| cipher | 0.4.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| clang-sys | 1.8.1 | Apache-2.0 | https://github.com/KyleMayes/clang-sys |
| clap | 2.34.0 | MIT | https://github.com/clap-rs/clap |
| clap | 4.5.53 | MIT OR Apache-2.0 | https://github.com/clap-rs/clap |
| clap_builder | 4.5.53 | MIT OR Apache-2.0 | https://github.com/clap-rs/clap |
| clap_lex | 0.7.6 | MIT OR Apache-2.0 | https://github.com/clap-rs/clap |
| clipboard-master | 4.0.0-beta.6 | MIT | https://github.com/DoumanAsh/clipboard-master |
| clipboard-win | 5.4.0 | BSL-1.0 | https://github.com/DoumanAsh/clipboard-win |
| cocoa | 0.20.2 | MIT / Apache-2.0 | https://github.com/servo/core-foundation-rs |
| cocoa | 0.24.1 | MIT / Apache-2.0 | https://github.com/servo/core-foundation-rs |
| cocoa | 0.25.0 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| cocoa-foundation | 0.1.2 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| color_quant | 1.1.0 | MIT | https://github.com/image-rs/color_quant.git |
| colorchoice | 1.0.1 | MIT OR Apache-2.0 | https://github.com/rust-cli/anstyle |
| compression-codecs | 0.4.31 | MIT OR Apache-2.0 | https://github.com/Nullus157/async-compression |
| compression-core | 0.4.29 | MIT OR Apache-2.0 | https://github.com/Nullus157/async-compression |
| confy | 0.4.0-2 | MIT/X11 OR Apache-2.0 | https://github.com/rust-cli/confy |
| const-oid | 0.9.6 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/const-oid |
| const_fn | 0.4.10 | Apache-2.0 OR MIT | https://github.com/taiki-e/const_fn |
| const_format | 0.2.32 | Zlib | https://github.com/rodrimati1992/const_format_crates/ |
| const_format_proc_macros | 0.2.32 | Zlib | https://github.com/rodrimati1992/const_format_crates/ |
| constant_time_eq | 0.2.6 | CC0-1.0 OR MIT-0 OR Apache-2.0 | https://github.com/cesarb/constant_time_eq |
| core-foundation | 0.10.1 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-foundation | 0.7.0 | MIT / Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-foundation | 0.9.4 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-foundation-sys | 0.7.0 | MIT / Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-foundation-sys | 0.8.7 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-graphics | 0.19.2 | MIT / Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-graphics | 0.22.3 | MIT / Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-graphics | 0.23.2 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-graphics-types | 0.1.3 | MIT OR Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-media-sys | 0.1.2 | MIT | https://github.com/luozijun/rust-core-media-sys |
| core-text | 19.2.0 | MIT/Apache-2.0 | https://github.com/servo/core-foundation-rs |
| core-video-sys | 0.1.4 | MIT | https://github.com/luozijun/rust-core-video-sys |
| core_maths | 0.1.1 | MIT | https://github.com/robertbastian/core_maths |
| coreaudio-rs | 0.11.3 | MIT/Apache-2.0 | https://github.com/RustAudio/coreaudio-rs.git |
| coreaudio-sys | 0.2.15 | MIT | https://github.com/RustAudio/coreaudio-sys.git |
| cpal | 0.15.3 | Apache-2.0 | https://github.com/rustaudio/cpal |
| cpufeatures | 0.2.12 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| crc | 3.3.0 | MIT OR Apache-2.0 | https://github.com/mrhooray/crc-rs.git |
| crc-catalog | 2.4.0 | MIT OR Apache-2.0 | https://github.com/akhilles/crc-catalog.git |
| crc32fast | 1.4.2 | MIT OR Apache-2.0 | https://github.com/srijs/rust-crc32fast |
| crossbeam-channel | 0.5.13 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| crossbeam-deque | 0.8.5 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| crossbeam-epoch | 0.9.20 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| crossbeam-queue | 0.3.11 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| crossbeam-utils | 0.8.20 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| crypto-bigint | 0.5.5 | Apache-2.0 OR MIT | https://github.com/RustCrypto/crypto-bigint |
| crypto-common | 0.1.6 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| ctr | 0.9.2 | MIT OR Apache-2.0 | https://github.com/RustCrypto/block-modes |
| ctrlc | 3.4.4 | MIT/Apache-2.0 | https://github.com/Detegr/rust-ctrlc.git |
| curve25519-dalek | 4.1.3 | BSD-3-Clause | https://github.com/dalek-cryptography/curve25519-dalek/tree/main/curve25519-dalek |
| curve25519-dalek-derive | 0.1.1 | MIT/Apache-2.0 | https://github.com/dalek-cryptography/curve25519-dalek |
| dashmap | 6.1.0 | MIT | https://github.com/xacrimon/dashmap |
| dasp | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_envelope | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_frame | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_interpolate | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_peak | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_ring_buffer | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_rms | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_sample | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/sample.git |
| dasp_signal | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_slice | 0.11.0 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| dasp_window | 0.11.1 | MIT OR Apache-2.0 | https://github.com/rustaudio/dasp.git |
| data-encoding | 2.9.0 | MIT | https://github.com/ia0/data-encoding |
| debug-helper | 0.3.13 | MIT | https://github.com/magiclen/debug-helper |
| default-net | 0.14.1 | MIT | https://github.com/shellrow/default-net |
| default_net | 0.1.0 | unspecified | git+https://github.com/rustdesk-org/default_net#78f8f70cd85151a3a2c4a3230d80d5272703c02e |
| der | 0.7.10 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/der |
| der-parser | 9.0.0 | MIT/Apache-2.0 | https://github.com/rusticata/der-parser.git |
| deranged | 0.3.11 | MIT OR Apache-2.0 | https://github.com/jhpratt/deranged |
| digest | 0.10.7 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| directories-next | 2.0.0 | MIT OR Apache-2.0 | https://github.com/xdg-rs/dirs/tree/master/directories |
| dirs | 2.0.2 | MIT OR Apache-2.0 | https://github.com/soc/dirs-rs |
| dirs | 5.0.1 | MIT OR Apache-2.0 | https://github.com/soc/dirs-rs |
| dirs-next | 2.0.0 | MIT OR Apache-2.0 | https://github.com/xdg-rs/dirs |
| dirs-sys | 0.3.7 | MIT OR Apache-2.0 | https://github.com/dirs-dev/dirs-sys-rs |
| dirs-sys | 0.4.1 | MIT OR Apache-2.0 | https://github.com/dirs-dev/dirs-sys-rs |
| dirs-sys-next | 0.1.2 | MIT OR Apache-2.0 | https://github.com/xdg-rs/dirs/tree/master/dirs-sys |
| dispatch | 0.2.0 | MIT | http://github.com/SSheldon/rust-dispatch |
| dispatch2 | 0.3.1 | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| displaydoc | 0.2.5 | MIT OR Apache-2.0 | https://github.com/yaahc/displaydoc |
| dlopen | 0.1.8 | MIT | https://github.com/szymonwieloch/rust-dlopen |
| dlopen_derive | 0.1.4 | MIT | https://github.com/rust-lang/crates.io-index |
| docopt | 1.1.1 | Unlicense/MIT | https://github.com/docopt/docopt.rs |
| downcast-rs | 1.2.1 | MIT/Apache-2.0 | https://github.com/marcianx/downcast-rs |
| dpi | 0.1.1 | Apache-2.0 | https://github.com/rust-windowing/winit |
| dtoa | 0.4.8 | MIT OR Apache-2.0 | https://github.com/dtolnay/dtoa |
| dunce | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 | https://gitlab.com/kornelski/dunce |
| ecdsa | 0.16.9 | Apache-2.0 OR MIT | https://github.com/RustCrypto/signatures/tree/master/ecdsa |
| ed25519 | 1.5.3 | Apache-2.0 OR MIT | https://github.com/RustCrypto/signatures/tree/master/ed25519 |
| either | 1.13.0 | MIT OR Apache-2.0 | https://github.com/rayon-rs/either |
| elliptic-curve | 0.13.8 | Apache-2.0 OR MIT | https://github.com/RustCrypto/traits/tree/master/elliptic-curve |
| enquote | 1.1.0 | Unlicense | https://github.com/reujab/enquote |
| enum-map | 2.7.3 | MIT OR Apache-2.0 | https://codeberg.org/xfix/enum-map |
| enum-map-derive | 0.17.0 | MIT OR Apache-2.0 | https://codeberg.org/xfix/enum-map |
| env_filter | 0.1.3 | MIT OR Apache-2.0 | https://github.com/rust-cli/env_logger |
| env_logger | 0.11.6 | MIT OR Apache-2.0 | https://github.com/rust-cli/env_logger |
| env_logger | 0.9.3 | MIT OR Apache-2.0 | https://github.com/env-logger-rs/env_logger/ |
| equivalent | 1.0.1 | Apache-2.0 OR MIT | https://github.com/cuviper/equivalent |
| errno | 0.3.14 | MIT OR Apache-2.0 | https://github.com/lambda-fairy/rust-errno |
| error-code | 3.2.0 | BSL-1.0 | https://github.com/DoumanAsh/error-code |
| exr | 1.72.0 | BSD-3-Clause | https://github.com/johannesvollmer/exrs |
| fastrand | 2.1.0 | Apache-2.0 OR MIT | https://github.com/smol-rs/fastrand |
| fdeflate | 0.3.4 | MIT OR Apache-2.0 | https://github.com/image-rs/fdeflate |
| ff | 0.13.1 | MIT/Apache-2.0 | https://github.com/zkcrypto/ff |
| filedescriptor | 0.8.2 | MIT | https://github.com/wez/wezterm |
| filetime | 0.2.23 | MIT/Apache-2.0 | https://github.com/alexcrichton/filetime |
| flate2 | 1.1.9 | MIT OR Apache-2.0 | https://github.com/rust-lang/flate2-rs |
| flexi_logger | 0.27.4 | MIT OR Apache-2.0 | https://github.com/emabee/flexi_logger |
| flume | 0.11.0 | Apache-2.0/MIT | https://github.com/zesterer/flume |
| fnv | 1.0.7 | Apache-2.0 / MIT | https://github.com/servo/rust-fnv |
| fon | 0.6.0 | Apache-2.0 OR MIT OR BSL-1.0 | https://github.com/ardaku/fon |
| fontdb | 0.23.0 | MIT | https://github.com/RazrFalcon/fontdb |
| foreign-types | 0.3.2 | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| foreign-types | 0.5.0 | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| foreign-types-macros | 0.2.3 | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| foreign-types-shared | 0.1.1 | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| foreign-types-shared | 0.3.1 | MIT/Apache-2.0 | https://github.com/sfackler/foreign-types |
| form_urlencoded | 1.2.1 | MIT OR Apache-2.0 | https://github.com/servo/rust-url |
| fruitbasket | 0.10.0 | Apache-2.0 | https://github.com/mrmekon/fruitbasket |
| fsevent | 2.1.2 | MIT | https://github.com/octplane/fsevent-rust |
| fsevent-sys | 4.1.0 | MIT | https://github.com/octplane/fsevent-rust/tree/master/fsevent-sys |
| futures | 0.3.30 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-channel | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-core | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-executor | 0.3.30 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-io | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-macro | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-sink | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-task | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-util | 0.3.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| generic-array | 0.14.7 | MIT | https://github.com/fizyk20/generic-array.git |
| getrandom | 0.2.15 | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| getrandom | 0.3.2 | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| ghash | 0.5.1 | Apache-2.0 OR MIT | https://github.com/RustCrypto/universal-hashes |
| gif | 0.13.1 | MIT/Apache-2.0 | https://github.com/image-rs/image-gif |
| gimli | 0.29.0 | MIT OR Apache-2.0 | https://github.com/gimli-rs/gimli |
| git2 | 0.16.1 | MIT OR Apache-2.0 | https://github.com/rust-lang/git2-rs |
| glob | 0.3.1 | MIT OR Apache-2.0 | https://github.com/rust-lang/glob |
| group | 0.13.0 | MIT/Apache-2.0 | https://github.com/zkcrypto/group |
| half | 2.4.1 | MIT OR Apache-2.0 | https://github.com/starkat99/half-rs |
| hashbrown | 0.14.5 | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| heck | 0.4.1 | MIT OR Apache-2.0 | https://github.com/withoutboats/heck |
| hex | 0.4.3 | MIT OR Apache-2.0 | https://github.com/KokaKiwi/rust-hex |
| hkdf | 0.12.4 | MIT OR Apache-2.0 | https://github.com/RustCrypto/KDFs/ |
| hmac | 0.12.1 | MIT OR Apache-2.0 | https://github.com/RustCrypto/MACs |
| home | 0.5.9 | MIT OR Apache-2.0 | https://github.com/rust-lang/cargo |
| hound | 3.5.1 | Apache-2.0 | https://github.com/ruuda/hound |
| html-escape | 0.2.13 | MIT | https://github.com/magiclen/html-escape |
| http | 1.3.1 | MIT OR Apache-2.0 | https://github.com/hyperium/http |
| http-body | 1.0.1 | MIT | https://github.com/hyperium/http-body |
| http-body-util | 0.1.3 | MIT | https://github.com/hyperium/http-body |
| httparse | 1.10.1 | MIT OR Apache-2.0 | https://github.com/seanmonstar/httparse |
| humantime | 2.1.0 | MIT/Apache-2.0 | https://github.com/tailhook/humantime |
| hyper | 1.7.0 | MIT | https://github.com/hyperium/hyper |
| hyper-rustls | 0.27.7 | Apache-2.0 OR ISC OR MIT | https://github.com/rustls/hyper-rustls |
| hyper-tls | 0.6.0 | MIT/Apache-2.0 | https://github.com/hyperium/hyper-tls |
| hyper-util | 0.1.17 | MIT | https://github.com/hyperium/hyper-util |
| iana-time-zone | 0.1.60 | MIT OR Apache-2.0 | https://github.com/strawlab/iana-time-zone |
| idna | 0.5.0 | MIT OR Apache-2.0 | https://github.com/servo/rust-url/ |
| image | 0.24.9 | MIT OR Apache-2.0 | https://github.com/image-rs/image |
| image | 0.25.1 | MIT OR Apache-2.0 | https://github.com/image-rs/image |
| impersonate_system | 0.1.0 | unspecified | git+https://github.com/rustdesk-org/impersonate-system#2f429010a5a10b1fe5eceb553c6672fd53d20167 |
| include_dir | 0.7.4 | MIT | https://github.com/Michael-F-Bryan/include_dir |
| include_dir_macros | 0.7.4 | MIT | https://github.com/Michael-F-Bryan/include_dir |
| indexmap | 2.2.6 | Apache-2.0 OR MIT | https://github.com/indexmap-rs/indexmap |
| inout | 0.1.3 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| instant | 0.1.13 | BSD-3-Clause | https://github.com/sebcrozet/instant |
| interceptor | 0.14.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/interceptor |
| ioctl-rs | 0.1.6 | MIT | https://github.com/dcuddeback/ioctl-rs |
| ipnet | 2.11.0 | MIT OR Apache-2.0 | https://github.com/krisprice/ipnet |
| iri-string | 0.7.8 | MIT OR Apache-2.0 | https://github.com/lo48576/iri-string |
| is-terminal | 0.4.16 | MIT | https://github.com/sunfishcode/is-terminal |
| is_debug | 1.0.1 | MIT AND Apache-2.0 | https://github.com/baoyachi/rust_is_debug |
| is_terminal_polyfill | 1.70.0 | MIT OR Apache-2.0 | https://github.com/polyfill-rs/is_terminal_polyfill |
| itertools | 0.12.1 | MIT OR Apache-2.0 | https://github.com/rust-itertools/itertools |
| itoa | 0.3.4 | MIT/Apache-2.0 | https://github.com/dtolnay/itoa |
| itoa | 1.0.11 | MIT OR Apache-2.0 | https://github.com/dtolnay/itoa |
| jobserver | 0.1.31 | MIT OR Apache-2.0 | https://github.com/rust-lang/jobserver-rs |
| jpeg-decoder | 0.3.1 | MIT OR Apache-2.0 | https://github.com/image-rs/jpeg-decoder |
| kcp-sys | 0.1.0 | MIT | https://github.com/EasyTier/kcp-sys |
| keepawake | 0.4.3 | MIT | https://github.com/segevfiner/keepawake-rs/ |
| kernel32-sys | 0.2.2 | MIT | https://github.com/retep998/winapi-rs |
| keyboard-types | 0.7.0 | MIT OR Apache-2.0 | https://github.com/pyfisch/keyboard-types |
| kurbo | 0.9.5 | MIT/Apache-2.0 | https://github.com/linebender/kurbo |
| lazy_static | 1.5.0 | MIT OR Apache-2.0 | https://github.com/rust-lang-nursery/lazy-static.rs |
| lazycell | 1.3.0 | MIT/Apache-2.0 | https://github.com/indiv0/lazycell |
| lebe | 0.5.2 | BSD-3-Clause | https://github.com/johannesvollmer/lebe |
| libc | 0.2.171 | MIT OR Apache-2.0 | https://github.com/rust-lang/libc |
| libgit2-sys | 0.14.2+1.5.1 | MIT OR Apache-2.0 | https://github.com/rust-lang/git2-rs |
| libloading | 0.8.4 | ISC | https://github.com/nagisa/rust_libloading/ |
| libm | 0.2.8 | MIT OR Apache-2.0 | https://github.com/rust-lang/libm |
| libsodium-sys | 0.2.7 | MIT OR Apache-2.0 | https://github.com/sodiumoxide/sodiumoxide.git |
| libz-sys | 1.1.18 | MIT OR Apache-2.0 | https://github.com/rust-lang/libz-sys |
| line-wrap | 0.2.0 | Apache-2.0 | https://bitbucket.org/marshallpierce/line-wrap-rs/src |
| lock_api | 0.4.12 | MIT OR Apache-2.0 | https://github.com/Amanieu/parking_lot |
| log | 0.4.22 | MIT OR Apache-2.0 | https://github.com/rust-lang/log |
| lru-slab | 0.1.2 | MIT OR Apache-2.0 OR Zlib | https://github.com/Ralith/lru-slab |
| mac_address | 1.1.7 | MIT OR Apache-2.0 | https://github.com/rep-nop/mac_address |
| mach2 | 0.4.2 | BSD-2-Clause OR MIT OR Apache-2.0 | https://github.com/JohnTitor/mach2 |
| machine-uid | 0.3.0 | MIT | https://github.com/Hanaasagi/machine-uid |
| magnum-opus | 0.4.0 | MIT/Apache-2.0 | https://github.com/DuckerMan/magnum-opus |
| malloc_buf | 0.0.6 | MIT | https://github.com/SSheldon/malloc_buf |
| matches | 0.1.10 | MIT | https://github.com/SimonSapin/rust-std-candidates |
| md-5 | 0.10.6 | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| md5 | 0.7.0 | Apache-2.0/MIT | https://github.com/stainless-steel/md5 |
| memalloc | 0.1.0 | MIT | https://github.com/reem/rust-memalloc.git |
| memchr | 2.7.4 | Unlicense OR MIT | https://github.com/BurntSushi/memchr |
| memmap2 | 0.9.8 | MIT OR Apache-2.0 | https://github.com/RazrFalcon/memmap2-rs |
| memoffset | 0.6.5 | MIT | https://github.com/Gilnaa/memoffset |
| memoffset | 0.7.1 | MIT | https://github.com/Gilnaa/memoffset |
| memoffset | 0.9.1 | MIT | https://github.com/Gilnaa/memoffset |
| metal | 0.18.0 | MIT OR Apache-2.0 | https://github.com/gfx-rs/metal-rs |
| minimal-lexical | 0.2.1 | MIT/Apache-2.0 | https://github.com/Alexhuszagh/minimal-lexical |
| miniz_oxide | 0.7.4 | MIT OR Zlib OR Apache-2.0 | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| miniz_oxide | 0.8.9 | MIT OR Zlib OR Apache-2.0 | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| mio | 1.0.3 | MIT | https://github.com/tokio-rs/mio |
| mozjpeg | 0.10.11 | IJG | https://github.com/ImageOptim/mozjpeg-rust |
| mozjpeg-sys | 2.2.2 | IJG AND Zlib AND BSD-3-Clause | https://github.com/kornelski/mozjpeg-sys.git |
| muda | 0.17.1 | Apache-2.0 OR MIT | https://github.com/amrbashir/muda |
| nanorand | 0.7.0 | Zlib | https://github.com/Absolucy/nanorand-rs |
| nasm-rs | 0.3.0 | MIT OR Apache-2.0 | https://github.com/medek/nasm-rs |
| native-tls | 0.2.12 | MIT OR Apache-2.0 | https://github.com/sfackler/rust-native-tls |
| native-windows-gui | 1.0.13 | MIT | https://github.com/gabdube/native-windows-gui |
| nix | 0.23.2 | MIT | https://github.com/nix-rust/nix |
| nix | 0.25.1 | MIT | https://github.com/nix-rust/nix |
| nix | 0.26.4 | MIT | https://github.com/nix-rust/nix |
| nix | 0.28.0 | MIT | https://github.com/nix-rust/nix |
| nokhwa | 0.10.7 | Apache-2.0 | https://github.com/l1npengtul/nokhwa |
| nokhwa-bindings-linux | 0.1.1 | Apache-2.0 | https://github.com/l1npengtul/nokhwa |
| nokhwa-bindings-macos | 0.2.2 | Apache-2.0 | https://github.com/l1npengtul/nokhwa |
| nokhwa-bindings-windows | 0.4.2 | Apache-2.0 | https://github.com/l1npengtul/nokhwa |
| nokhwa-core | 0.1.5 | Apache-2.0 | https://github.com/l1npengtul/nokhwa |
| nom | 7.1.3 | MIT | https://github.com/Geal/nom |
| ntapi | 0.4.1 | Apache-2.0 OR MIT | https://github.com/MSxDOS/ntapi |
| nu-ansi-term | 0.49.0 | MIT | https://github.com/nushell/nu-ansi-term |
| num-bigint | 0.4.6 | MIT OR Apache-2.0 | https://github.com/rust-num/num-bigint |
| num-conv | 0.1.0 | MIT OR Apache-2.0 | https://github.com/jhpratt/num-conv |
| num-integer | 0.1.46 | MIT OR Apache-2.0 | https://github.com/rust-num/num-integer |
| num-traits | 0.1.43 | MIT/Apache-2.0 | https://github.com/rust-num/num-traits |
| num-traits | 0.2.19 | MIT OR Apache-2.0 | https://github.com/rust-num/num-traits |
| num_cpus | 1.16.0 | MIT OR Apache-2.0 | https://github.com/seanmonstar/num_cpus |
| num_threads | 0.1.7 | MIT OR Apache-2.0 | https://github.com/jhpratt/num_threads |
| objc | 0.2.7 | MIT | http://github.com/SSheldon/rust-objc |
| objc-foundation | 0.1.1 | MIT | http://github.com/SSheldon/rust-objc-foundation |
| objc-sys | 0.3.5 | MIT | https://github.com/madsmtm/objc2 |
| objc2 | 0.5.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2 | 0.6.4 | MIT | https://github.com/madsmtm/objc2 |
| objc2-app-kit | 0.2.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2-app-kit | 0.3.2 | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| objc2-core-data | 0.2.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2-core-foundation | 0.3.2 | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| objc2-core-graphics | 0.3.2 | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| objc2-core-image | 0.2.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2-encode | 4.1.0 | MIT | https://github.com/madsmtm/objc2 |
| objc2-foundation | 0.2.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2-foundation | 0.3.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2-metal | 0.2.2 | MIT | https://github.com/madsmtm/objc2 |
| objc2-quartz-core | 0.2.2 | MIT | https://github.com/madsmtm/objc2 |
| objc_exception | 0.1.2 | MIT | http://github.com/SSheldon/rust-objc-exception |
| objc_id | 0.1.1 | MIT | http://github.com/SSheldon/rust-objc-id |
| object | 0.36.1 | Apache-2.0 OR MIT | https://github.com/gimli-rs/object |
| oid-registry | 0.7.1 | MIT OR Apache-2.0 | https://github.com/rusticata/oid-registry.git |
| once_cell | 1.19.0 | MIT OR Apache-2.0 | https://github.com/matklad/once_cell |
| opaque-debug | 0.3.1 | MIT OR Apache-2.0 | https://github.com/RustCrypto/utils |
| option-ext | 0.2.0 | MPL-2.0 | https://github.com/soc/option-ext.git |
| os-version | 0.2.0 | MIT/Apache-2.0 | https://github.com/kpcyrd/os-version |
| osascript | 0.3.0 | BSD-3-Clause | https://github.com/mitsuhiko/rust-osascript |
| p256 | 0.13.2 | Apache-2.0 OR MIT | https://github.com/RustCrypto/elliptic-curves/tree/master/p256 |
| p384 | 0.13.1 | Apache-2.0 OR MIT | https://github.com/RustCrypto/elliptic-curves/tree/master/p384 |
| parity-tokio-ipc | 0.7.3-6 | MIT/Apache-2.0 | https://github.com/nikvolf/parity-tokio-ipc |
| parking_lot | 0.12.3 | MIT OR Apache-2.0 | https://github.com/Amanieu/parking_lot |
| parking_lot_core | 0.9.10 | MIT OR Apache-2.0 | https://github.com/Amanieu/parking_lot |
| paste | 1.0.15 | MIT OR Apache-2.0 | https://github.com/dtolnay/paste |
| peeking_take_while | 0.1.2 | Apache-2.0/MIT | https://github.com/fitzgen/peeking_take_while |
| pem | 3.0.6 | MIT | https://github.com/jcreekmore/pem-rs.git |
| pem-rfc7468 | 0.7.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/pem-rfc7468 |
| percent-encoding | 2.3.1 | MIT OR Apache-2.0 | https://github.com/servo/rust-url/ |
| phf | 0.7.24 | MIT | https://github.com/sfackler/rust-phf |
| phf_codegen | 0.7.24 | MIT | https://github.com/sfackler/rust-phf |
| phf_generator | 0.7.24 | MIT | https://github.com/sfackler/rust-phf |
| phf_shared | 0.7.24 | MIT | https://github.com/sfackler/rust-phf |
| piet | 0.6.2 | MIT/Apache-2.0 | https://github.com/linebender/piet |
| piet-coregraphics | 0.6.2 | MIT/Apache-2.0 | https://github.com/linebender/piet |
| pin-project | 1.1.5 | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project |
| pin-project-internal | 1.1.5 | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project |
| pin-project-lite | 0.2.14 | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project-lite |
| pin-utils | 0.1.0 | MIT OR Apache-2.0 | https://github.com/rust-lang-nursery/pin-utils |
| pkcs8 | 0.10.2 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/pkcs8 |
| pkg-config | 0.3.30 | MIT OR Apache-2.0 | https://github.com/rust-lang/pkg-config-rs |
| plist | 1.6.1 | MIT | https://github.com/ebarnard/rust-plist/ |
| png | 0.17.13 | MIT OR Apache-2.0 | https://github.com/image-rs/image-png |
| png | 0.18.1 | MIT OR Apache-2.0 | https://github.com/image-rs/image-png |
| polyval | 0.6.2 | Apache-2.0 OR MIT | https://github.com/RustCrypto/universal-hashes |
| portable-atomic | 1.11.1 | Apache-2.0 OR MIT | https://github.com/taiki-e/portable-atomic |
| portable-pty | 0.8.1 | MIT | https://github.com/wez/wezterm |
| powerfmt | 0.2.0 | MIT OR Apache-2.0 | https://github.com/jhpratt/powerfmt |
| ppv-lite86 | 0.2.17 | MIT/Apache-2.0 | https://github.com/cryptocorrosion/cryptocorrosion |
| prettyplease | 0.2.20 | MIT OR Apache-2.0 | https://github.com/dtolnay/prettyplease |
| primeorder | 0.13.6 | Apache-2.0 OR MIT | https://github.com/RustCrypto/elliptic-curves/tree/master/primeorder |
| proc-macro2 | 0.4.30 | MIT/Apache-2.0 | https://github.com/alexcrichton/proc-macro2 |
| proc-macro2 | 1.0.93 | MIT OR Apache-2.0 | https://github.com/dtolnay/proc-macro2 |
| protobuf | 3.7.2 | MIT | https://github.com/stepancheg/rust-protobuf/ |
| protobuf-codegen | 3.7.2 | MIT | https://github.com/stepancheg/rust-protobuf/ |
| protobuf-parse | 3.7.2 | MIT | https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-parse/ |
| protobuf-support | 3.7.2 | MIT | https://github.com/stepancheg/rust-protobuf/ |
| qoi | 0.4.1 | MIT/Apache-2.0 | https://github.com/aldanor/qoi-rust |
| qrcode-generator | 4.1.9 | MIT | https://github.com/magiclen/qrcode-generator |
| qrcodegen | 1.8.0 | MIT | https://github.com/nayuki/QR-Code-generator |
| quest | 0.3.0 | MIT | https://github.com/quadrupleslap/quest |
| quick-xml | 0.30.0 | MIT | https://github.com/tafia/quick-xml |
| quick-xml | 0.31.0 | MIT | https://github.com/tafia/quick-xml |
| quinn | 0.11.9 | MIT OR Apache-2.0 | https://github.com/quinn-rs/quinn |
| quinn-proto | 0.11.15 | MIT OR Apache-2.0 | https://github.com/quinn-rs/quinn |
| quinn-udp | 0.5.14 | MIT OR Apache-2.0 | https://github.com/quinn-rs/quinn |
| quote | 0.6.13 | MIT OR Apache-2.0 | https://github.com/dtolnay/quote |
| quote | 1.0.36 | MIT OR Apache-2.0 | https://github.com/dtolnay/quote |
| rand | 0.6.5 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand | 0.8.5 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand | 0.9.2 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand_chacha | 0.1.1 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_chacha | 0.3.1 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand_chacha | 0.9.0 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand_core | 0.3.1 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_core | 0.4.2 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_core | 0.6.4 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand_core | 0.9.3 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand_hc | 0.1.0 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_isaac | 0.1.1 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_jitter | 0.1.4 | MIT OR Apache-2.0 | https://github.com/rust-random/rand |
| rand_os | 0.1.3 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_pcg | 0.1.2 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| rand_xorshift | 0.1.1 | MIT/Apache-2.0 | https://github.com/rust-random/rand |
| raw-window-handle | 0.6.2 | MIT OR Apache-2.0 OR Zlib | https://github.com/rust-windowing/raw-window-handle |
| rayon | 1.10.0 | MIT OR Apache-2.0 | https://github.com/rayon-rs/rayon |
| rayon-core | 1.12.1 | MIT OR Apache-2.0 | https://github.com/rayon-rs/rayon |
| rcgen | 0.13.2 | MIT OR Apache-2.0 | https://github.com/rustls/rcgen |
| rdev | 0.5.0-2 | MIT | https://github.com/Narsil/rdev |
| regex | 1.11.1 | MIT OR Apache-2.0 | https://github.com/rust-lang/regex |
| regex-automata | 0.4.9 | MIT OR Apache-2.0 | https://github.com/rust-lang/regex/tree/master/regex-automata |
| regex-syntax | 0.8.5 | MIT OR Apache-2.0 | https://github.com/rust-lang/regex/tree/master/regex-syntax |
| repng | 0.2.2 | MIT | https://github.com/quadrupleslap/repng |
| reqwest | 0.12.24 | MIT OR Apache-2.0 | https://github.com/seanmonstar/reqwest |
| rfc6979 | 0.4.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/signatures/tree/master/rfc6979 |
| rgb | 0.8.50 | MIT | https://github.com/kornelski/rust-rgb |
| ring | 0.17.14 | Apache-2.0 AND ISC | https://github.com/briansmith/ring |
| ringbuf | 0.3.3 | MIT/Apache-2.0 | https://github.com/agerasev/ringbuf.git |
| rpassword | 2.1.0 | Apache-2.0 | https://github.com/conradkdotcom/rpassword |
| rtcp | 0.13.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/rtcp |
| rtp | 0.13.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/rtp |
| rustc-demangle | 0.1.24 | MIT/Apache-2.0 | https://github.com/rust-lang/rustc-demangle |
| rustc-hash | 1.1.0 | Apache-2.0/MIT | https://github.com/rust-lang-nursery/rustc-hash |
| rustc-hash | 2.1.1 | Apache-2.0 OR MIT | https://github.com/rust-lang/rustc-hash |
| rustc_version | 0.4.0 | MIT/Apache-2.0 | https://github.com/Kimundi/rustc-version-rs |
| rusticata-macros | 4.1.0 | MIT/Apache-2.0 | https://github.com/rusticata/rusticata-macros.git |
| rustix | 0.38.34 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | https://github.com/bytecodealliance/rustix |
| rustls | 0.23.28 | Apache-2.0 OR ISC OR MIT | https://github.com/rustls/rustls |
| rustls-native-certs | 0.8.1 | Apache-2.0 OR ISC OR MIT | https://github.com/rustls/rustls-native-certs |
| rustls-pki-types | 1.11.0 | MIT OR Apache-2.0 | https://github.com/rustls/pki-types |
| rustls-platform-verifier | 0.6.2 | MIT OR Apache-2.0 | https://github.com/rustls/rustls-platform-verifier |
| rustls-webpki | 0.103.3 | ISC | https://github.com/rustls/webpki |
| rustversion | 1.0.17 | MIT OR Apache-2.0 | https://github.com/dtolnay/rustversion |
| ryu | 1.0.18 | Apache-2.0 OR BSL-1.0 | https://github.com/dtolnay/ryu |
| same-file | 1.0.6 | Unlicense/MIT | https://github.com/BurntSushi/same-file |
| schannel | 0.1.23 | MIT | https://github.com/steffengy/schannel-rs |
| sciter-rs | 0.5.57 | MIT | https://github.com/sciter-sdk/rust-sciter |
| scopeguard | 1.2.0 | MIT OR Apache-2.0 | https://github.com/bluss/scopeguard |
| sdp | 0.8.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/sdp |
| sec1 | 0.7.3 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/sec1 |
| security-framework | 2.10.0 | MIT OR Apache-2.0 | https://github.com/kornelski/rust-security-framework |
| security-framework | 3.5.1 | MIT OR Apache-2.0 | https://github.com/kornelski/rust-security-framework |
| security-framework-sys | 2.15.0 | MIT OR Apache-2.0 | https://github.com/kornelski/rust-security-framework |
| semver | 1.0.23 | MIT OR Apache-2.0 | https://github.com/dtolnay/semver |
| serde | 0.9.15 | MIT/Apache-2.0 | https://github.com/serde-rs/serde |
| serde | 1.0.228 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_core | 1.0.228 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_derive | 1.0.228 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_json | 0.9.10 | MIT/Apache-2.0 | https://github.com/serde-rs/json |
| serde_json | 1.0.118 | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| serde_repr | 0.1.19 | MIT OR Apache-2.0 | https://github.com/dtolnay/serde-repr |
| serde_spanned | 0.6.6 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| serde_urlencoded | 0.7.1 | MIT/Apache-2.0 | https://github.com/nox/serde_urlencoded |
| serial | 0.4.0 | MIT | https://github.com/dcuddeback/serial-rs |
| serial-core | 0.4.0 | MIT | https://github.com/dcuddeback/serial-rs |
| serial-unix | 0.4.0 | MIT | https://github.com/dcuddeback/serial-rs |
| serial-windows | 0.4.0 | MIT | https://github.com/dcuddeback/serial-rs |
| sha1 | 0.10.6 | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| sha2 | 0.10.8 | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| shadow-rs | 0.21.0 | MIT AND Apache-2.0 | https://github.com/baoyachi/shadow-rs |
| shared_library | 0.1.9 | Apache-2.0/MIT | https://github.com/tomaka/shared_library/ |
| shared_memory | 0.12.4 | MIT OR Apache-2.0 | https://github.com/elast0ny/shared_memory-rs |
| shell-words | 1.1.0 | MIT/Apache-2.0 | https://github.com/tmiasko/shell-words |
| shlex | 1.3.0 | MIT OR Apache-2.0 | https://github.com/comex/rust-shlex |
| shutdown_hooks | 0.1.0 | MIT | https://github.com/Techern/shutdown_hooks_rs |
| signal-hook-registry | 1.4.2 | Apache-2.0/MIT | https://github.com/vorner/signal-hook |
| signature | 1.6.4 | Apache-2.0 OR MIT | https://github.com/RustCrypto/traits/tree/master/signature |
| signature | 2.2.0 | Apache-2.0 OR MIT | https://github.com/RustCrypto/traits/tree/master/signature |
| simd-adler32 | 0.3.7 | MIT | https://github.com/mcountryman/simd-adler32 |
| siphasher | 0.2.3 | MIT/Apache-2.0 | https://github.com/jedisct1/rust-siphash |
| slab | 0.4.9 | MIT | https://github.com/tokio-rs/slab |
| slotmap | 1.0.7 | Zlib | https://github.com/orlp/slotmap |
| smallvec | 1.15.1 | MIT OR Apache-2.0 | https://github.com/servo/rust-smallvec |
| smol_str | 0.2.2 | MIT OR Apache-2.0 | https://github.com/rust-analyzer/smol_str |
| socket2 | 0.3.19 | MIT/Apache-2.0 | https://github.com/alexcrichton/socket2-rs |
| socket2 | 0.5.10 | MIT OR Apache-2.0 | https://github.com/rust-lang/socket2 |
| sodiumoxide | 0.2.7 | MIT OR Apache-2.0 | https://github.com/sodiumoxide/sodiumoxide |
| softbuffer | 0.4.5 | MIT OR Apache-2.0 | https://github.com/rust-windowing/softbuffer |
| spin | 0.9.8 | MIT | https://github.com/mvdnes/spin-rs.git |
| spki | 0.7.3 | Apache-2.0 OR MIT | https://github.com/RustCrypto/formats/tree/master/spki |
| strict-num | 0.1.1 | MIT | https://github.com/RazrFalcon/strict-num |
| strsim | 0.10.0 | MIT | https://github.com/dguo/strsim-rs |
| strsim | 0.11.1 | MIT | https://github.com/rapidfuzz/strsim-rs |
| strsim | 0.8.0 | MIT | https://github.com/dguo/strsim-rs |
| strum | 0.24.1 | MIT | https://github.com/Peternator7/strum |
| strum_macros | 0.24.3 | MIT | https://github.com/Peternator7/strum |
| stun | 0.8.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/stun |
| stun_codec | 0.3.5 | MIT | https://github.com/sile/stun_codec |
| stunclient | 0.4.1 | MIT/Apache-2.0 | https://github.com/vi/rust-stunclient |
| substring | 1.4.5 | MIT OR Apache-2.0 | https://github.com/Anders429/substring |
| subtle | 2.6.1 | BSD-3-Clause | https://github.com/dalek-cryptography/subtle |
| syn | 0.15.44 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| syn | 1.0.109 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| syn | 2.0.98 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| sync_wrapper | 1.0.2 | Apache-2.0 | https://github.com/Actyx/sync_wrapper |
| synstructure | 0.13.2 | MIT | https://github.com/mystor/synstructure |
| sys-locale | 0.3.1 | MIT OR Apache-2.0 | https://github.com/1Password/sys-locale |
| sysinfo | 0.29.10 | MIT | https://github.com/GuillaumeGomez/sysinfo |
| system-configuration | 0.5.1 | MIT OR Apache-2.0 | https://github.com/mullvad/system-configuration-rs |
| system-configuration-sys | 0.5.0 | MIT OR Apache-2.0 | https://github.com/mullvad/system-configuration-rs |
| system_shutdown | 4.0.1 | MIT/Apache-2.0 | https://github.com/risoflora/system_shutdown |
| tao | 0.25.0 | Apache-2.0 | https://github.com/tauri-apps/tao |
| target_build_utils | 0.3.1 | ISC/Apache-2.0 | https://github.com/nagisa/target_build_utils.rs |
| tauri-winrt-notification | 0.1.3 | MIT OR Apache-2.0 | https://github.com/tauri-apps/winrt-notification |
| tempfile | 3.10.1 | MIT OR Apache-2.0 | https://github.com/Stebalien/tempfile |
| termcolor | 1.4.1 | Unlicense OR MIT | https://github.com/BurntSushi/termcolor |
| termios | 0.2.2 | MIT | https://github.com/dcuddeback/termios-rs |
| termios | 0.3.3 | MIT | https://github.com/dcuddeback/termios-rs |
| textwrap | 0.11.0 | MIT | https://github.com/mgeisler/textwrap |
| tfc | 0.7.0 | MIT OR Apache-2.0 | https://github.com/indianakernick/The-Fat-Controller |
| thiserror | 1.0.61 | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| thiserror | 2.0.17 | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| thiserror-impl | 1.0.61 | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| thiserror-impl | 2.0.17 | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| tiff | 0.9.1 | MIT | https://github.com/image-rs/image-tiff |
| time | 0.1.45 | MIT/Apache-2.0 | https://github.com/time-rs/time |
| time | 0.3.36 | MIT OR Apache-2.0 | https://github.com/time-rs/time |
| time-core | 0.1.2 | MIT OR Apache-2.0 | https://github.com/time-rs/time |
| time-macros | 0.2.18 | MIT OR Apache-2.0 | https://github.com/time-rs/time |
| tiny-skia | 0.11.4 | BSD-3-Clause | https://github.com/RazrFalcon/tiny-skia |
| tiny-skia-path | 0.11.4 | BSD-3-Clause | https://github.com/RazrFalcon/tiny-skia/tree/master/path |
| tinyvec | 1.6.1 | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/tinyvec |
| tinyvec_macros | 0.1.1 | MIT OR Apache-2.0 OR Zlib | https://github.com/Soveu/tinyvec_macros |
| tokio | 1.44.2 | MIT | https://github.com/tokio-rs/tokio |
| tokio-macros | 2.5.0 | MIT | https://github.com/tokio-rs/tokio |
| tokio-native-tls | 0.3.1 | MIT | https://github.com/tokio-rs/tls |
| tokio-rustls | 0.26.0 | MIT/Apache-2.0 | https://github.com/rustls/tokio-rustls |
| tokio-socks | 0.5.2-3 | MIT | https://github.com/sticnarf/tokio-socks |
| tokio-tungstenite | 0.29.0 | MIT | https://github.com/snapview/tokio-tungstenite |
| tokio-util | 0.7.15 | MIT | https://github.com/tokio-rs/tokio |
| toml | 0.5.11 | MIT/Apache-2.0 | https://github.com/toml-rs/toml |
| toml | 0.7.8 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| toml_datetime | 0.6.3 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| toml_edit | 0.19.15 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| totp-rs | 5.5.1 | MIT | https://github.com/constantoine/totp-rs |
| tower | 0.5.2 | MIT | https://github.com/tower-rs/tower |
| tower-http | 0.6.6 | MIT | https://github.com/tower-rs/tower-http |
| tower-layer | 0.3.3 | MIT | https://github.com/tower-rs/tower |
| tower-service | 0.3.3 | MIT | https://github.com/tower-rs/tower |
| tracing | 0.1.41 | MIT | https://github.com/tokio-rs/tracing |
| tracing-attributes | 0.1.29 | MIT | https://github.com/tokio-rs/tracing |
| tracing-core | 0.1.34 | MIT | https://github.com/tokio-rs/tracing |
| trackable | 0.2.24 | MIT | https://github.com/sile/trackable |
| trackable | 1.3.0 | MIT | https://github.com/sile/trackable |
| trackable_derive | 1.0.0 | MIT | https://github.com/sile/trackable_derive |
| tray-icon | 0.21.3 | MIT OR Apache-2.0 | https://github.com/tauri-apps/tray-icon |
| try-lock | 0.2.5 | MIT | https://github.com/seanmonstar/try-lock |
| ttf-parser | 0.25.1 | MIT OR Apache-2.0 | https://github.com/harfbuzz/ttf-parser |
| tungstenite | 0.29.0 | MIT OR Apache-2.0 | https://github.com/snapview/tungstenite-rs |
| turn | 0.10.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/turn |
| typenum | 1.17.0 | MIT OR Apache-2.0 | https://github.com/paholg/typenum |
| tz-rs | 0.6.14 | MIT OR Apache-2.0 | https://github.com/x-hgg-x/tz-rs |
| tzdb | 0.5.10 | Apache-2.0 | https://github.com/Kijewski/tzdb |
| tzdb | 0.6.1 | Apache-2.0 | https://github.com/Kijewski/tzdb |
| tzdb_data | 0.1.2 | MIT-0 | https://github.com/Kijewski/tzdb |
| unic-bidi | 0.9.0 | MIT/Apache-2.0 | https://github.com/open-i18n/rust-unic/ |
| unic-char-property | 0.9.0 | MIT/Apache-2.0 | https://github.com/open-i18n/rust-unic/ |
| unic-char-range | 0.9.0 | MIT/Apache-2.0 | https://github.com/open-i18n/rust-unic/ |
| unic-common | 0.9.0 | MIT/Apache-2.0 | https://github.com/open-i18n/rust-unic/ |
| unic-ucd-bidi | 0.9.0 | MIT/Apache-2.0 | https://github.com/open-i18n/rust-unic/ |
| unic-ucd-version | 0.9.0 | MIT/Apache-2.0 | https://github.com/open-i18n/rust-unic/ |
| unicode-bidi | 0.3.15 | MIT OR Apache-2.0 | https://github.com/servo/unicode-bidi |
| unicode-ident | 1.0.12 | (MIT OR Apache-2.0) AND Unicode-DFS-2016 | https://github.com/dtolnay/unicode-ident |
| unicode-normalization | 0.1.23 | MIT/Apache-2.0 | https://github.com/unicode-rs/unicode-normalization |
| unicode-segmentation | 1.11.0 | MIT/Apache-2.0 | https://github.com/unicode-rs/unicode-segmentation |
| unicode-width | 0.1.13 | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-width |
| unicode-xid | 0.1.0 | MIT/Apache-2.0 | https://github.com/unicode-rs/unicode-xid |
| unicode-xid | 0.2.4 | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-xid |
| universal-hash | 0.5.1 | MIT OR Apache-2.0 | https://github.com/RustCrypto/traits |
| untrusted | 0.9.0 | ISC | https://github.com/briansmith/untrusted |
| url | 2.5.2 | MIT OR Apache-2.0 | https://github.com/servo/rust-url |
| urlencoding | 2.1.3 | MIT | https://github.com/kornelski/rust_urlencoding |
| utf8-width | 0.1.7 | MIT | https://github.com/magiclen/utf8-width |
| utf8parse | 0.2.2 | Apache-2.0 OR MIT | https://github.com/alacritty/vte |
| uuid | 1.16.0 | Apache-2.0 OR MIT | https://github.com/uuid-rs/uuid |
| vcpkg | 0.2.15 | MIT/Apache-2.0 | https://github.com/mcgoo/vcpkg-rs |
| vec_map | 0.8.2 | MIT/Apache-2.0 | https://github.com/contain-rs/vec-map |
| version_check | 0.9.4 | MIT/Apache-2.0 | https://github.com/SergioBenitez/version_check |
| waitgroup | 0.1.2 | Apache-2.0 | https://github.com/laizy/waitgroup-rs |
| walkdir | 2.5.0 | Unlicense/MIT | https://github.com/BurntSushi/walkdir |
| wallpaper | 3.2.0 | Unlicense | https://github.com/reujab/wallpaper.rs |
| want | 0.3.1 | MIT | https://github.com/seanmonstar/want |
| webm | 1.1.0 | MPL-2.0 | https://github.com/DiamondLovesYou/rust-webm.git |
| webm-sys | 1.0.4 | MPL-2.0 | https://github.com/DiamondLovesYou/rust-webm.git |
| webpki-roots | 0.26.11 | CDLA-Permissive-2.0 | https://github.com/rustls/webpki-roots |
| webpki-roots | 1.0.9 | CDLA-Permissive-2.0 | https://github.com/rustls/webpki-roots |
| webrtc | 0.13.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc |
| webrtc-data | 0.11.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/data |
| webrtc-dtls | 0.12.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/dtls |
| webrtc-ice | 0.13.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/ice |
| webrtc-mdns | 0.9.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/mdns |
| webrtc-media | 0.10.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/media |
| webrtc-sctp | 0.12.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/sctp |
| webrtc-srtp | 0.15.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/srtp |
| webrtc-util | 0.11.0 | MIT OR Apache-2.0 | https://github.com/webrtc-rs/webrtc/tree/master/util |
| weezl | 0.1.8 | MIT OR Apache-2.0 | https://github.com/image-rs/lzw |
| which | 4.4.2 | MIT | https://github.com/harryfei/which-rs.git |
| whoami | 1.6.0 | Apache-2.0 OR BSL-1.0 OR MIT | https://github.com/ardaku/whoami |
| widestring | 1.1.0 | MIT OR Apache-2.0 | https://github.com/starkat99/widestring-rs |
| win-sys | 0.3.1 | MIT OR Apache-2.0 | https://github.com/elast0ny/win-sys |
| winapi | 0.2.8 | MIT | https://github.com/retep998/winapi-rs |
| winapi | 0.3.9 | MIT/Apache-2.0 | https://github.com/retep998/winapi-rs |
| winapi-build | 0.1.1 | MIT | https://github.com/retep998/winapi-rs |
| winapi-util | 0.1.8 | Unlicense OR MIT | https://github.com/BurntSushi/winapi-util |
| windows | 0.32.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.34.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.43.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.44.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.48.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.51.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.52.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.54.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows | 0.61.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-collections | 0.2.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-core | 0.51.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-core | 0.52.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-core | 0.54.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-core | 0.61.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-future | 0.2.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-implement | 0.52.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-implement | 0.60.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-interface | 0.52.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-interface | 0.59.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-link | 0.1.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-link | 0.2.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-numerics | 0.2.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-result | 0.1.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-result | 0.3.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-service | 0.6.0 | MIT OR Apache-2.0 | https://github.com/mullvad/windows-service-rs |
| windows-strings | 0.3.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-strings | 0.4.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-sys | 0.45.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-sys | 0.48.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-sys | 0.52.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-sys | 0.60.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-targets | 0.42.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-targets | 0.48.5 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-targets | 0.52.6 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-targets | 0.53.5 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-version | 0.1.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-win | 3.0.0 | BSL-1.0 | https://github.com/DoumanAsh/windows-win-rs |
| windows_x86_64_msvc | 0.32.0 | MIT OR Apache-2.0 | https://github.com/rust-lang/crates.io-index |
| windows_x86_64_msvc | 0.34.0 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows_x86_64_msvc | 0.42.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows_x86_64_msvc | 0.48.5 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows_x86_64_msvc | 0.52.6 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows_x86_64_msvc | 0.53.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| winnow | 0.5.40 | MIT | https://github.com/winnow-rs/winnow |
| winreg | 0.10.1 | MIT | https://github.com/gentoo90/winreg-rs |
| winreg | 0.11.0 | MIT | https://github.com/gentoo90/winreg-rs |
| winres | 0.1.12 | MIT | https://github.com/mxre/winres |
| wol-rs | 1.0.1 | MIT | https://github.com/fengyc/wol-rs |
| x25519-dalek | 2.0.1 | BSD-3-Clause | https://github.com/dalek-cryptography/curve25519-dalek/tree/main/x25519-dalek |
| x509-parser | 0.16.0 | MIT OR Apache-2.0 | https://github.com/rusticata/x509-parser.git |
| xattr | 1.4.0 | MIT/Apache-2.0 | https://github.com/Stebalien/xattr |
| xxhash-rust | 0.8.18 | BSL-1.0 | https://github.com/DoumanAsh/xxhash-rust |
| yasna | 0.5.2 | MIT OR Apache-2.0 | https://github.com/qnighy/yasna.rs |
| zerocopy | 0.7.34 | BSD-2-Clause OR Apache-2.0 OR MIT | https://github.com/google/zerocopy |
| zerocopy-derive | 0.7.34 | BSD-2-Clause OR Apache-2.0 OR MIT | https://github.com/google/zerocopy |
| zeroize | 1.8.1 | Apache-2.0 OR MIT | https://github.com/RustCrypto/utils/tree/master/zeroize |
| zeroize_derive | 1.4.2 | Apache-2.0 OR MIT | https://github.com/RustCrypto/utils/tree/master/zeroize/derive |
| zstd | 0.13.1 | MIT | https://github.com/gyscos/zstd-rs |
| zstd-safe | 7.1.0 | MIT/Apache-2.0 | https://github.com/gyscos/zstd-rs |
| zstd-sys | 2.0.11+zstd.1.5.6 | MIT/Apache-2.0 | https://github.com/gyscos/zstd-rs |
| zune-inflate | 0.2.54 | MIT OR Apache-2.0 OR Zlib | https://github.com/etemesi254/zune-image/tree/main/zune-inflate |
