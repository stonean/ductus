---
section: "Generators and Hooks"
---

# The-adopter-ci-template-can-run

## Context

017's §CI safety net says adopters run the generators in CI through `framework/templates/ci/adopter-generators.yml`, and AC24 is ticked on that basis. The template cannot run in an adopter's CI. Since `f4c3bbd3` (2026-08-19, first released in ductus-v0.30.0), when the frontmatter generators became runtime primitives, its steps call `.ductus/bin/ductus`. That is the per-project pointer `/ductus` materializes, which `framework/bootstrap/ductus.md` §Pointer materialization says is gitignored, so it "never arrives with a clone", and the template has no step that acquires a runtime. On a fresh CI checkout its first step finds no such file and fails. Before that commit the template called `.ductus/scripts/gen-spec-deps.sh`, which the Shared Files manifest wrote into the adopter's tree, so the change that promoted the generators removed what the template ran without replacing it.

Two related facts, measured 2026-10-20. No Shared Files manifest row installs the template and nothing in `README.md` or `docs/` points at it; the four places that said otherwise were corrected in `d235e722`. And an adopter's tree carries no runtime version pin: `/ductus` reads the pin live from this repository's `version` file at the ref its run resolves.

An independent BE-DEPS-006 assessment of 022 task 3's pinning found it (2026-10-20).

## Behavior

- A workflow an adopter copies from the template runs on a fresh checkout of their repository, or the template stops claiming it does. Which of those, and how, is the open question below.
- Whatever the template fetches to run the generators is pinned to an immutable reference (`BE-DEPS-006`) and is read by the weekly pins check, like every other CI input in this repository.

## Edge Cases

- A project in a subdirectory of its repository keeps working under the template's commented `working-directory` default (spec 059, AC10).
- The template's runtime and the runtime an adopter's `/ductus` acquired can be different versions, since the adopter's copy is taken once and `/ductus` follows the latest release. Where derived frontmatter differs between the two, the template's sync check would fail a correct commit.
- Reading the version from this repository's latest release at run time would select what runs by a mutable reference, which `BE-DEPS-006` does not allow, so no option below does it.

## Open Questions

- How does the template get a runtime? (a) Install it from crates.io: a SHA-pinned `dtolnay/rust-toolchain` step, then `cargo install --locked ductus --version <exact>`, the form the release workflow already uses for its tools and that the pins check already compares with crates.io; a cold install took 91 s on a 16-core machine on 2026-10-20 and will take several times that on a standard runner, unmeasured, unless cached. (b) Download the release binary at a pinned version and check it against a sha256 written into the template; faster per run, but a release cannot carry its own archive's hash, so the template would pin the release before it or need a commit after each release, and the pins check would need a new form to read it. (c) Withdraw the template: delete it and correct what describes it, which reopens the five `done` specs whose spec or plan names it (026, 043, 049, 057, 059), edits 020's, which is in progress, and edits `framework/commands/review.md`, and leaves adopters with no CI gate. (d) Keep the template and say in its header that the adopter adds a step that puts a runtime at `.ductus/bin/ductus`; the template still fails as copied.

## Resolved Questions

*None yet.*
