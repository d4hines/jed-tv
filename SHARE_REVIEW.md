# Sharing review — second pass, 2026-11-01

**Recommendation:** publish the team's original implementation, interface
findings, research notes, backup/recovery tools, and local image patchers.
Keep licensed third-party source with its notices. Do not treat the words
"root", "firmware", "decompiled", or a vendor function name as automatic
reasons to withhold our own work. Hold actual credentials, copied proprietary
payloads, imports without redistribution permission, and the narrow
factory-token replay mechanism for separate resolution.

This is a U.S.-focused practical assessment, not a legal opinion about every
file. It assumes lawful possession of the owner-operated TVs and no NDA or
other negotiated restriction on the team's research. It does not establish
that distribution rules in every jurisdiction are identical.

## What this copy contains

Source: fetched `internal/main` at `5a0fcfd` (full ID in the manifest).
An incremental update imports 44 changed original files since `85528fe`,
including the Linux confinement model and proofs, City Flight performance
work, and G13 signed-app deployment fixes and research. A second, from
`120f0b6`, adds the GPL/LGPL texts and City Flight notices, the new root
README, and the removal of `README.txt` and `src/README.md`. Existing
exclusions, content redactions/edits, and license notices are preserved.
This copy retains **2,098 of 2,103** original files, including all
**585 Lean files** and **338 original research files**, with the original
AGENTS.md archived as `research/archive/INTERNAL_AGENT_NOTES.txt`.
The current AGENTS.md remains
short repository guidance. Historical commands and hardware state are not
instructions to perform new hardware operations.

Restored material includes Magisk/boot-image recipes, root verification,
kexec tooling, OTA-signature checking, backup and readback utilities, USB
recovery preparation, operational findings, and the team's City Flight build
wrappers. These are source tools and notes, not distributions of the vendor
images they inspect or modify. Recovery-label handling and other device
access mechanisms remain fact-specific; their inclusion is a reasoned
selection, not a claim of blanket exemption for every use.

Two OTA-query clients now require `TCL_OTA_APP_KEY` from the caller; the
embedded vendor signing constant was removed. The factory-root research note
keeps its findings but omits the executable token-extraction, helper-deployment,
replay, and cleanup recipe. The Lean wordmark is unchanged, with a trademark
symbol and wording identifying the programming language and independent TV
project. The root README keeps the team's text and appends this copy's
provenance and licensing notes. Other retained original files are
byte-identical.

## Remaining exclusions — 5 files

| Material | Assessment | What would permit inclusion |
| --- | --- | --- |
| Three factory-token replay helpers | Narrow legal-review item: they reuse a signed stock application credential and matching process identity to register for privileged vendor operations. This is more specific than ordinary root access or a generic backup utility. No conclusion that it is necessarily unlawful. | Focused review of whether this controls access to protected works and whether 17 USC 1201(f) or another defense permits distribution; authorizations may also resolve it. |
| Prebuilt QEMU kernel | Linux is redistributable; this is a GPL packaging task, not suspicious proprietary firmware. | Supply its complete corresponding source and build material through a compliant distribution arrangement. Config, source URL, manifest, and build script remain. A source URL alone is not being represented as completed compliance. |
| YouTube launcher icon | Brand-use review, not a claim that all YouTube references are prohibited. The existing client is independent, and a logo can imply affiliation. | Check the exact use against current brand terms or use original neutral art. |

The three held helper paths are `research/hello/FactoryBootProbe.java`,
`research/probes/FactoryAuthorizedIdentityProbe.java`, and
`research/probes/FactoryReadback.java`. Every excluded path and its reason is
listed in `sharing/source-manifest.json`.

Ignored assets were never imported: vendor firmware, APKs, proprietary
libraries/modules, complete decompilations, device backups, production keys,
and commercial game data. Do not add these just because we can read them from
our hardware. Publishing our extraction/patching tools is a separate decision
from publishing their inputs or outputs.

## Retained third-party material and release questions

- **City Flight:** restored after the owner confirmed that its author has
  authorized public redistribution. All imported code is included, and the
  team's renderer screenshot is restored. `PUBLICATION.txt` records the
  permission; `JSBSIM_NOTICES.txt` records upstream attributions. The JSBSim
  library has LGPL terms, while its C172 model carries an explicit no-sale
  notice. Free public source sharing is the scope here; do not infer rights
  to sell the model or treat the entire import as permissively licensed.

- **FFmpeg, OPL, Doomgeneric, Axiom, fonts, and public MediaTek material:** keep
  notices and their respective licenses. Translation into Lean/Rust does not
  remove obligations. Source distribution is much easier to package than a
  linked binary release; GPL/LGPL requirements still matter for both.
- **Lean Doom upstream:** retained with the internal provenance declaration of
  GPL-2.0-or-later derivation. The public upstream root and README still lack
  an explicit license for its author's original additions. Get the upstream
  author's confirmation before treating those contributions as cleared for
  public redistribution. The base engine's GPL does not automatically supply
  permission for every new contribution. This issue predates this export.
- **Doom wordmark:** restored with existing attribution; its exact Commons
  source labels it public-domain text/logo artwork. Trademark rights remain;
  identify the port without suggesting endorsement.
- **Lean wordmark:** restored with the original geometry, added trademark
  symbol, and explicit language/independent-project attribution, following the
  published policy for accurately identifying software written using Lean.
- **Team-owned code:** the owner explicitly confirmed the dande-router imports
  are owned by the team and authorized sharing. No new blanket license has
  been assigned. Publishing your own code and granting everyone a reuse
  license are separate decisions; choose reuse terms before calling the
  complete repository open source. Preserve existing third-party terms.
- **YouTube clients and codec implementations:** retaining original protocol
  and decoder code is not a finding about service-contract compliance or
  codec patent licensing for a shipped product. Those questions do not justify
  deleting unrelated source and research wholesale.

## Why this boundary is less restrictive

The Copyright Office distinguishes protected program expression from
functional aspects such as algorithms, logic, and system design. Original
interface descriptions and implementations therefore need a different
analysis from copying vendor source or firmware.
[Copyright Office Circular 61](https://www.copyright.gov/circs/circ61.pdf).

The current smart-TV exemption covers circumvention for interoperability of
lawfully obtained applications, excluding unauthorized access to other
copyrighted works. That closely matches the stated aim of running our own
software, although it is not a file-by-file legal determination.
[37 CFR 201.40(b)(10)](https://www.ecfr.gov/current/title-37/chapter-II/subchapter-A/part-201/section-201.40).

That exemption concerns the act of circumvention. It does not automatically
excuse distributing a circumvention tool. Section 1201(f) separately provides
conditional permissions for interoperability research and sharing necessary
information/means. This is why the credential-replay mechanism merits focused
review while generic file tools and factual notes need not all be withheld.
[17 USC 1201(a), (f)](https://www.copyright.gov/title17/92chap12.html);
[EFF reverse-engineering FAQ](https://www.eff.org/issues/coders/reverse-engineering-faq).

Open-source redistribution depends on the actual license, preserved notices,
and compliance, not on whether code was translated or modified.
[GNU license guidance](https://www.gnu.org/licenses/gpl-howto.en.html).
Artwork evidence: [Doom file provenance](https://commons.wikimedia.org/wiki/File:Doom_logo.png),
[Lean trademark policy](https://lean-lang.org/trademark-policy/),
and [YouTube brand resources](https://brand.youtube/).
JSBSim sources: [library license](https://github.com/JSBSim-Team/jsbsim/blob/v1.3.3/COPYING)
and [C172 model notice](https://github.com/JSBSim-Team/jsbsim/blob/v1.3.3/aircraft/c172p/c172p.xml).

## Verification and delivery

The manifest records each original path, source hash, destination if moved,
output hash, and the five content edits described above. Credential-pattern
checks and a scan for the known removed vendor constant were repeated.
The two changed query clients were checked for rejection before network access
when no key is supplied; their Python syntax and the changed SVG were checked.
No TV operation, complete build, or runtime test is claimed.

The selected source is one root commit. The share bundle and clean clone
contain only that commit. The original working checkout's `.git` retains
private internal objects and review scratch files: distribute the clean clone
or bundle, not that `.git` directory, and do not mirror-push it. Nothing was
pushed by this review. Existing application bundles still need the omitted
YouTube icon and/or kernel; the historical FFmpeg
assembly parity check still expects internal Git history.
