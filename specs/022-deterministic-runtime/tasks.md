---
next-task: 35
---

# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 23. Installed files are not group-writable

- [x] Implement the behavior described in `scenarios/installed-files-are-not-group-writable.md`
- [x] `extract-archive` masks each entry's mode to `rwxr-xr-x` (`PERMISSION_MASK` in `runtime/src/primitives/extract_archive.rs`), on the tar and zip paths, with the module doc saying why
- [x] Tests show a `0664` entry landing as `0644` and a `0775` one as `0755` on both paths, and each fails when the mask keeps group write (`scripts/mutation/installed-file-modes.py`)
- [x] `apply-manifest` re-applies the extracted mode to an `update`-strategy file whose bytes already match when only its mode differs, leaving bytes and modification time alone, with a test and a mutation in `scripts/mutation/installed-file-modes.py` (owner decision 2026-10-27: reach files already installed)
- [ ] A destination whose mode the runner may not change keeps its mode and is named in `apply-manifest`'s `modes-kept`, and the walk goes on rather than stopping mid-manifest; `framework/bootstrap/ductus.md` step 4 names it in the completion message (found by the independent re-read of `47e68233`). Written 2026-10-28 and its macOS-only test passes; `python3 scripts/mutation/run.py scripts/mutation/installed-file-modes.py` ran whole on macOS at `77fff147` on 2026-10-28, 8 of 8 killed (M7 and M8 are this item's). Still owed: a real adopter bootstrap that shows the completion message naming a `modes-kept` file, run in a scratch adopter whose `.ductus/config.toml` sets `[runtime] path` to this tree's release build, with one destination made immutable (`chflags uchg`) so the runner may not change its mode; no release carries this runtime, and none is cut before every spec is `done` (owner, 2026-11-11)

- **Done when**: an archive's `0664` and `0775` entries extract as `0644` and `0755`, the executable bit survives, an already-installed file with matching bytes takes the masked mode on the next run, and the mutation run kills every mutation in `scripts/mutation/installed-file-modes.py`

## 32. Bound dashboard's unknown config keys and discover-rule-files' notices (BE-PAYLOAD-001, from 019)

- [x] `dashboard`'s `config.unknown-keys` and `discover-rule-files`' `notices` go through `bounded_lists::place`: each list's `-count` inline in both forms, and past `[artifacts] read-size-bytes` the lists in the file `lists-file` names. The owner deferred 019's analysis finding here on 2026-11-14 (019's `analysis.md` stored decision): both lists grow with the unrecognized keys in the project's config, 019's plan (§Unknown keys are reported by name, never dropped) adds them, and both primitives are 022's (019's plan, §The runtime half is owned by 022 and 063). `check-artifact-size`'s notices were bounded by 063's task 17
- [x] `DASHBOARD`, `DISCOVER_RULE_FILES` and 022's data model commit to the bound, and every caller that reads either list reads it from `lists-file` when `form` is `file`: the `/status` view, `/review` step 2's notice lines, and the exec walker
- [x] A test proves the inline form under the read size and the file form past it, each failing under its mutant, and `framework-checks.yml` step (u) passes

- **Done when**: Both lists return inline, or in a lists-file past the read size, proven by tests their mutants turn red; the whole `cargo test` passes; and 022's next review reads the change.

## 33. create-scenario refuses an existing slug atomically (BE-RACE-003, from 023)

- [ ] `create-scenario` creates its file exclusively, with `create_new` or a no-clobber persist of its tempfile, so of two concurrent calls with one slug exactly one writes and the other gets `scenario already exists: {feature-path}/scenarios/{slug}.md`. Today `runtime/src/primitives/create_scenario.rs` checks `dest.exists()` and then calls `write_atomic`, a tempfile and a rename that replaces any existing file, so both calls pass the check and the second silently overwrites the first's scenario. The owner deferred 023's BE-RACE-003 (MUST) finding here on 2026-11-14; 023's task 25 cites this task
- [ ] 022's `scenarios/ask-consolidation.md` edge case "`create-scenario` with a slug that already exists" states the exclusive-create mechanism
- [ ] A test fails under the old exists-then-rename: a file appearing between the check and the write is refused and left as it was, and the test turns red under its mutant

- **Done when**: create-scenario never replaces an existing scenario file, proven by a test its mutant turns red; the whole `cargo test` passes; and 022's next review reads the change.

## 34. The session files open through canonical containment (BE-INPUT-004, from 062)

- [ ] Every open of a session file goes through `validate_contained` against the project root, or refuses a symbolic link leaving the project: this process's own target (`own_path`, `sessions_dir(repo).join("{key}.toml")`, from an identity sanitized from `DUCTUS_SESSION` or `CLAUDE_CODE_SESSION_ID`), the shared default `.ductus/session.toml`, and the other sessions' files under `.ductus/sessions/`. Today none of `runtime/src/session.rs`, `write_session.rs`, `resolve_session.rs` or `retarget_sessions.rs` calls `validate_contained`, so a symbolic link there is followed out of the project. The owner deferred 062's BE-INPUT-004 (MUST) finding here on 2026-11-14, since the session core's path containment is 022's; 062's task 29 cites this task
- [ ] 022's `plan.md` §Path arguments names the session files among the sites that open a path: its lexical-screen bullet covers `write-session`'s and `retarget-sessions`' `path` argument, which they record and never open, and not the session files they read and write
- [ ] An outward-link test: a `.ductus/sessions/` entry, and the shared default, each a symbolic link to a file outside the project, is refused or left unread rather than followed, and the test turns red under its mutant

- **Done when**: No session file is read or written through a link leaving the project, proven by an outward-link test its mutant turns red; the whole `cargo test` passes; and 022's next review reads the change.
