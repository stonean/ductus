---
section: "Generators and Hooks"
---

# The-adopter-ci-template-can-run

## Context

017's §CI safety net says adopters run the generators in CI through `framework/templates/ci/adopter-generators.yml`, and AC24 is ticked on that basis. Until this scenario the template could not run in an adopter's CI. From `f4c3bbd3` (2026-08-19, first released in ductus-v0.30.0), when the frontmatter generators became runtime primitives, its steps called `.ductus/bin/ductus`. That is the per-project pointer `/ductus` materializes, which `framework/bootstrap/ductus.md` §Pointer materialization says is gitignored, so it "never arrives with a clone", and the template had no step that acquired a runtime. On a fresh CI checkout its first step found no such file and failed. Before that commit the template called `.ductus/scripts/gen-spec-deps.sh`, which the Shared Files manifest wrote into the adopter's tree, so the change that promoted the generators removed what the template ran without replacing it.

Two related facts, measured 2026-10-20. No Shared Files manifest row installs the template and nothing in `README.md` or `docs/` points at it; the four places that said otherwise were corrected in `d235e722`. And an adopter's tree carries no runtime version pin: `/ductus` reads the pin live from this repository's `version` file at the ref its run resolves.

An independent BE-DEPS-006 assessment of 022 task 3's pinning found it (2026-10-20).

## Behavior

- A workflow an adopter copies from the template runs on a fresh checkout of their repository: its first step after the checkout installs the runtime the other steps call.
- That step downloads the release binary, so the adopter needs no Rust or cargo. It names the release by `DUCTUS_VERSION` and the x86_64 Linux archive's sha256 by `DUCTUS_SHA256`, checks the download against the hash before anything runs, fails when the binary reports another version, and puts the binary on `PATH` from outside the work tree, so the sync check never sees it.
- Both values are pinned to an immutable reference (`BE-DEPS-006`) and read by the weekly pins check, like every other CI input in this repository: it fails when `DUCTUS_VERSION` is behind this repository's latest release, when `DUCTUS_SHA256` is not the sha256 that release published, and when a version has no hash beside it. A release cannot carry its own archive's hash, so the template always names a release already published, and the bump follows each release.

## Edge Cases

- A project in a subdirectory of its repository keeps working under the template's commented `working-directory` default (spec 059, AC10).
- The template's runtime and the runtime an adopter's `/ductus` acquired can be different versions, since the adopter's copy is taken once and `/ductus` follows the latest release. Where derived frontmatter differs between the two, the template's sync check would fail a correct commit.
- Reading the version from this repository's latest release at run time would select what runs by a mutable reference, which `BE-DEPS-006` does not allow, so the template does not do it.

## Open Questions

*None — resolved below.*

## Resolved Questions

- **How does the template get a runtime?** Resolved 2026-10-20 by the owner: (b), download the release binary. The owner's reason: option (a) would require the adopter to have Rust and cargo installed, and the release binaries are built to make installation easier and to remove that requirement. The options as put: (a) Install it from crates.io: a SHA-pinned `dtolnay/rust-toolchain` step, then `cargo install --locked ductus --version <exact>`, the form the release workflow already uses for its tools and that the pins check already compares with crates.io; a cold install took 91 s on a 16-core machine on 2026-10-20 and will take several times that on a standard runner, unmeasured, unless cached. (b) Download the release binary at a pinned version and check it against a sha256 written into the template; faster per run, but a release cannot carry its own archive's hash, so the template would pin the release before it or need a commit after each release, and the pins check would need a new form to read it. (c) Withdraw the template: delete it and correct what describes it, which reopens the five `done` specs whose spec or plan names it (026, 043, 049, 057, 059), edits 020's, which is in progress, and edits `framework/commands/review.md`, and leaves adopters with no CI gate. (d) Keep the template and say in its header that the adopter adds a step that puts a runtime at `.ductus/bin/ductus`; the template still fails as copied.
