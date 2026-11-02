# Selected source repository

Read README.md, SHARE_REVIEW.md, THIRD_PARTY.md, and sharing/source-manifest.json
before changing the selection. Historical research notes describe past device
state, not current hardware state or authorization to operate hardware.

Preserve existing attribution and license notices. Do not import internal Git
history, vendor firmware, decompiled vendor sources, factory authentication
credentials, held token-replay helpers, or omitted third-party imports into this tree.
Do not assign a new repository-wide license without an owner decision.

Keep TV runtime code and its adjacent Lean specifications/proofs in src/;
host tooling belongs in tools/, checks in test/, Nix code in nix/, and research
in research/. Rust manifests, build.rs, and in-crate tests remain with crates.
Lean package buildDir belongs under assets/build/lake/; generated objects,
binaries, logs, and local device material belong under ignored assets/.

Do not rewrite published history or operate a TV as part of a source-only task.
