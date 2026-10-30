# 056 — Bootstrap archive-boundary split Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Create the archive half

- [x] Create `framework/bootstrap/ductus-procedure.md` with frontmatter carrying a `description:`, an H1, and a short preamble saying what the file is, that it is read from the extracted archive and never installed, and that §Instructions in `ductus.md` is the numbered walker.
- [x] Append the five blocks **verbatim** in original file order (A §Pre-run Migrations, B §Frontmatter Migration, C §Security Audit (brownfield), D §Hook Installation, E §What This Command Does NOT Do → §Directory Creation).
- [x] Confirm the copy is byte-exact against the source ranges before any prose repair — diff the extracted bytes rather than eyeballing.

- **Done when**: `framework/bootstrap/ductus-procedure.md` exists and its nine sections are byte-identical to their source ranges in `ductus.md` at `dfc4a3fc`.

## 2. Excise the five blocks and leave one pointer

- [x] Delete blocks E, D, C, B, A from `framework/bootstrap/ductus.md` in that order — bottom-up, so each earlier block's line numbers stay valid while the edit is in flight.
- [x] Add one `## The archive half` section immediately after §File Fetching naming `{tempdir}/ductus-main/framework/bootstrap/ductus-procedure.md`, what it holds, and when the run reads it.
- [x] Repair, as a **separately identifiable** edit, only the prose the move falsifies: §Pre-flight abort's skipped-section enumeration (which names sections now in the other file) and any "below"/"above" directional wording whose target left the file.
- [x] Re-read every surviving `**Section**` and `§Section` reference in `ductus.md` and confirm it resolves within `ductus.md` or is explicitly qualified as living in the archive half.

- **Done when**: `ductus.md` holds none of the nine moved sections, carries exactly one pointer to the archive half, and no surviving cross-reference in it names a section that is neither present nor qualified.

## 3. Allowlist the archive half as reference prose

- [x] Add `framework/bootstrap/ductus-procedure.md` to `runtime/legacy-prose-commands.txt`.
- [x] Correct that file's header, which asserts every entry is a `framework/commands/*.md` file — a claim the new entry falsifies.
- [x] Run `bash scripts/lint-procedure-parseability.sh` and confirm it passes, then confirm by probe that removing the entry makes it **fail** — a lint that passes for the wrong reason is worth less than no lint.

- **Done when**: the parseability lint passes with the entry and fails without it, proven by running it both ways.

## 4. Re-copy the retired bootstrap alias

- [x] `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md` in the same commit as task 2.
- [x] Run `bash scripts/audit/transitional-bootstrap-parity.sh` and confirm Family 21 passes.

- **Done when**: `cmp framework/bootstrap/ductus.md framework/bootstrap/govern.md` reports no difference and Family 21 exits 0.

## 5. Re-earn the consumer classification by running it

- [x] Run each of the seven audit families named in the spec's §What the split does not reach (`installer-command-parity`, `sweep-target-manifest-parity`, `manifest-destination-links`, `self-url-resolution`, `installer-registry-parity`, `host-namespace-parity`, `runtime-probe-parity`) and confirm each reports a **non-empty** extraction rather than merely exiting 0.
- [x] Probe `adopter_destinations` in both directions: confirm it derives the same destination set from the post-split `ductus.md` as from the pre-split file, and confirm it returns empty when the manifest tables are absent — so the passing result is known to distinguish the two.

- **Done when**: every consumer resolves a non-empty subject after the split, and the `adopter_destinations` probe has demonstrated both the populated and the empty outcome.

## 6. Run the whole local gate, after committing

- [x] Commit tasks 1–4 as one change (`git show --stat` **and** `git status --short`), then run `bash scripts/audit/run-all.sh` — Families 19 and 20 read committed history, so a pre-commit run is not evidence for them.
- [x] Prove `run-all.sh` fails before trusting its silence: write a wrong value into the repo-root `version`, confirm Family 20 exits 1, restore with `git checkout -- version`.
- [x] Run `npx markdownlint-cli2`; the six `lint-*.sh` scripts; `scripts/tests/*.sh`; `shellcheck -S warning` over the tracked shell set minus `runtime/tests/fixtures/`; the three generators plus `derive-dependencies` and `derive-references` reporting no drift.
- [x] Under `runtime/`, with an absolute `cd` on each invocation: `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, `cargo test --release --locked`. Redirect rather than pipe, read `$?` on the line that ran the command, and count `^test result:` lines against the binary count.

- **Done when**: every gate command has been run with its exit status read directly, `run-all.sh` has been demonstrated to fail on a seeded defect, and all pass.

## 7. Record the post-split measurement

- [x] Measure `framework/bootstrap/ductus.md` after the split and write the figure into `plan.md` §Post-split measurement, against the 146,953-byte pre-split file.
- [x] State the delta between the achieved reduction and the 38,616 B the spec predicted, and account for any difference (the pointer section adds bytes back).

- **Done when**: `plan.md` §Post-split measurement carries a figure taken after the split, with the prediction delta accounted for.

## 8. Close out

- [x] Verify each acceptance criterion against the tree by hand, including the ones `check-artifacts` reports as `skipped`.
- [x] Re-run `check-artifacts` **after** the status flip and walk the `skipped` array entry by entry, expecting this pass's own annotations to add entries.
- [x] Retire the inbox item this spec discharges, matching it by a distinctive substring of its own text rather than by line number.

- **Done when**: every criterion is verified against the tree, the `skipped` array has been walked entry by entry, and the inbox item is removed.

## 9. The walker places every archive-half section

- [ ] Implement the behavior described in `scenarios/the-walker-places-every-archive-half-section.md`

- **Done when**: the scenario's described behavior is correctly implemented and verified by a real adopter bootstrap walked in the stated order
