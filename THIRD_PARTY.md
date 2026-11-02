# Provenance and existing licenses

This document records evidence in the imported tree. It does not relicense
third-party work or assert that all ownership questions have been resolved.
The source manifest records the exact internal snapshot. Original source files
and their notices are preserved byte for byte except where the manifest says
otherwise.

| Component | Evidence and treatment |
| --- | --- |
| Zr runtime, assembly DSL, host tooling, and conformance work from dande-router | `src/lean/zr/README.md` and `tools/bootloader/vendor/UPSTREAM.txt` record source revisions. On 2026-11-01 the owner explicitly confirmed: "Include it; we own it and authorize sharing." This supersedes the earlier uncertainty about permission to share these team-owned imports; it does not select a public software license. |
| Lean Doom and Doomgeneric/Chocolate Doom derivatives | `src/doomtv/lean/UPSTREAM.txt` records the Lean import and GPL-2.0-or-later derivation. `src/third_party/doomgeneric/UPSTREAM` and its existing `LICENSE` are retained. Full GPL v2 text is also in `LICENSES/GPL-2.0.txt`. The Lean upstream's missing standalone license remains a provenance question; its derivative status does not by itself prove permission for every original upstream contribution. |
| Nuked OPL3 and Chocolate Doom music ports | The headers of `Doom/Audio/Opl.lean`, `OplTables.lean`, `OplMusic.lean`, and `OplMusicTables.lean` name sources and authors. Preserve their LGPL-2.1-or-later/GPL-2.0-or-later terms; both license texts are included. |
| FFmpeg-derived Rust decoder, assembly, Lean H.264 and assembly DSL translations | `src/ffh264/Cargo.toml`, source headers, `src/lean/yt/STATUS.md`, and `src/lean/zr/ZrAsm/Ffmpeg/` identify LGPL-2.1-or-later derivation. Full LGPL v2.1 text is in `LICENSES/LGPL-2.1.txt`. These translations are third-party derivatives, not exclusively original team work. |
| Axiom: First Light | `src/axiomtv/lean/LICENSE.axiom` retains Peter Gostev's MIT license. The source/build metadata identifies `petergpt/lean-3d-game-axiom`. |
| DejaVu fonts and generated UI glyph data | `src/rpgtv/fonts/dejavu-2.37/LICENSE` and `SOURCE.txt` are retained alongside both font files. They also supply the notice for `src/lean/yt/Yt/Ui/FontData.lean`; its old comment references the font's former location. |
| MediaTek FCIE register/transaction material | `src/bootloader/Unsafe/LICENSE.MediaTek` identifies the public TCL source revision and chooses its BSD-3-Clause alternative. Preserve that notice with Bootstrap.lean. |
| City Flight / JsbExact | Included after the owner confirmed author permission for public redistribution on 2026-11-01. See `src/third_party/cityflight/PUBLICATION.txt` and `JSBSIM_NOTICES.txt`. JSBSim library LGPL terms and the upstream C172 model no-sale notice remain separate obligations; no blanket permissive license is asserted. |
| City Flight tile | `src/flighttv/src/main.rs` identifies `demo/flight-tile.png` as the output of our renderer without the HUD. Restored as a team-rendered procedural scene. |
| Doom wordmark | Restored from the recorded Commons source, which identifies it as public-domain text/logo artwork. Existing source attribution is in `demo/tiles/README.txt`. Trademark rights remain; this project makes no claim of endorsement. |
| Lean boot graphic | The original wordmark geometry is retained. The public copy adds a trademark symbol and wording identifying use of the Lean programming language and the independent TV project. See the current Lean trademark policy linked in SHARE_REVIEW.md. |

Dependency fetch/build recipes remain source tooling. They do not transfer
ownership of downloaded dependencies, game data, firmware, or generated
binaries. In particular, commercial Doom game data is not covered by the Doom
engine license. Existing Freedoom recipes should retain Freedoom's notices
when distributing its data.

The GPL text was copied unchanged from the snapshot's Doomgeneric license.
The LGPL text was obtained unchanged from FFmpeg's `n8.0/COPYING.LGPLv2.1`
(https://raw.githubusercontent.com/FFmpeg/FFmpeg/n8.0/COPYING.LGPLv2.1).
This supplies license text, not a change to the decoder's recorded version.

Git author attribution in the source history names Daniel Hines, EduardoRFS,
and Jack Hall. Squashing does not remove their authorship or third-party
copyright interests. This list describes recorded Git authors, not a legal
ownership determination.
