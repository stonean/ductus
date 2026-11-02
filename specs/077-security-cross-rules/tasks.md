# 077 — Security cross rules Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Register the `SEC` prefix (AC2)

- [x] `scripts/lint-rule-ids.sh` allows `SEC`: its `id_re`, its source comment naming `specs/077-security-cross-rules/data-model.md`, and its malformed-heading message
- [x] `framework/reference/analyze.md`'s "Which ID format governs a file", item 1, names 077's data-model as `SEC`'s format source
- [x] 008's `data-model.md` names `security-cross.md`'s `SEC` prefix beside `CFG` and `QUAL`
- [x] Prove the lint change both ways on a scratch copy: a `### SEC-LOG-001` heading passes, and a `### XS-LOG-001` heading still fails

- **Done when**: `bash scripts/lint-rule-ids.sh` passes on the tree, the scratch-copy probe shows `SEC` accepted and an unregistered prefix refused, and both documents name 077's data-model.

## 2. Create `security-cross.md` with `SEC-LOG-001` (AC1, AC3)

- [x] Fetch the OWASP Logging Cheat Sheet, and quote from it only what the fetched copy holds
- [x] Write the file's header (scope, `SEC-{CATEGORY}-{NNN}`, the categories `LOG`, `DEPS` and `AUTHN`, and a pointer to constitution §rules), and `SEC-LOG-001` as the plan's "`SEC-LOG-001` states its own value set" decision specifies
- [x] `SEC-LOG-001`'s value list matches `BE-LOG-002`'s Statement word for word

- **Done when**: `SEC-LOG-001` has a Statement, a Verification with its three owners under their load conditions, and a fetched Source, and `bash scripts/lint-rule-ids.sh` and `npx markdownlint-cli2` pass on the file.

## 3. Move `BE-DEPS-006` to `SEC-DEPS-001` (AC4, AC5)

- [x] Re-fetch `BE-DEPS-006`'s Sources and correct any quote that no longer matches
- [x] Write `SEC-DEPS-001` with `BE-DEPS-006`'s Statement and Verification. Its owners take load conditions: `BE-DEPS-002` takes `security-backend.md`, and `FE-DEPS-004` takes `security-frontend.md`
- [x] Label `BE-DEPS-006` `**DEPRECATED in 0.59.12:**`, naming `SEC-DEPS-001` and removal in `0.61.0`, and make its Verification one sentence naming `SEC-DEPS-001` as the owner of every case, with no condition

- **Done when**: `SEC-DEPS-001` carries `BE-DEPS-006`'s obligations, and `BE-DEPS-006` is a labelled pointer whose Verification reports nothing of its own.

## 4. Move `BE-AUTHN-009`, `-013` and `-020` to `SEC-AUTHN-001` to `-003` (AC4, AC5)

Handoff, 2026-11-02: tasks 1 to 3 are committed, with task 7's manifest row and a first `## [Unreleased]` entry in `runtime/CHANGELOG.md` (`SEC-LOG-001`, and `BE-DEPS-006` to `SEC-DEPS-001`) landed early, so the pushed tree never deprecates a rule in favour of one that does not install. This task extends that entry with the three `AUTHN` moves. The form of a deprecated rule is `BE-DEPS-006`'s in `framework/rules/security-backend.md`: the `**DEPRECATED in 0.59.12:**` paragraph under the heading, the Statement and Source kept, and a Verification reading "This rule is deprecated. **Not flagged:** every case it governs, which `security-cross.md` `SEC-DEPS-001` owns." A successor's text is its predecessor's Statement and Verification verbatim; only the IDs change, and each owner on a surface file gains a load condition. Prove the label with `check-rule-ids`, which reported `BE-DEPS-006` `deprecated: true`. The tree is shared with another session, which had `runtime/` work staged: commit as AGENTS.md's pre-commit-hook gotcha says, and run `scripts/gen-claude-commands.sh` first, since `framework/reference/analyze.md` is generated into `.ductus/reference/`.

- [ ] Re-fetch each rule's Sources (RFC 9700, RFC 9126, RFC 8252, RFC 7636, OpenID Connect Core 1.0, and the constant-time comparison sources) and correct any stale quote
- [ ] Write the three successors with their predecessors' text. `SEC-AUTHN-001`'s owner `BE-API-010` (d) and `SEC-AUTHN-002`'s owner `BE-API-009` take "where `security-backend.md` is also loaded". The two rules' references to each other name `SEC-AUTHN-002` and `SEC-AUTHN-003`, with no condition
- [ ] Label the three predecessors as task 3 labelled `BE-DEPS-006`, each a pointer to its successor

- **Done when**: each successor carries its predecessor's obligations, each predecessor is a labelled pointer, and `every_shipped_rule_is_assessable` passes.

## 5. Re-point the yields and Statements (AC6)

- [ ] `security-backend.md`: the yields in `BE-API-009`, `BE-API-010`, `BE-API-011` and `BE-API-013`, and `BE-API-010` (d)'s and `BE-API-011`'s Statements, name the successors
- [ ] `security-frontend.md`: `FE-AUTHN-001`'s and `FE-STORAGE-003`'s yields name the successors and drop "where `security-backend.md` is also loaded"
- [ ] `configuration-cross.md`: `CFG-ENV-006` names a non-configuration sensitive value in a log line as `BE-LOG-002`'s where `security-backend.md` is also loaded and otherwise `SEC-LOG-001`'s. Its trace-context pointer to `BE-LOG-008` stays, since a sensitive configuration value there is 008 task 43's open item, as the plan's "Re-pointing" decision says

- **Done when**: no rule file outside a deprecated rule's own label and pointer names one of the four deprecated IDs, and every yield to a successor reaches it, read on both sides.

## 6. Sweep the live citations (AC7)

- [ ] Re-point the citations the plan's "Sweep" decision lists. Each is a mechanical substitution
- [ ] Run `actionlint` on the two changed workflows
- [ ] Grep the live-artifact set in `AGENTS.md` for each deprecated ID, and classify each hit: re-pointed, a decision or measurement record kept as written, or a changelog or audit record

- **Done when**: every hit is re-pointed or classified, and no present-tense claim of a deprecated ID as a current requirement remains outside the deprecated rules themselves.

## 7. Ship the file (AC8, AC9, AC11)

- [x] `framework/bootstrap/ductus.md`: the manifest row `framework/rules/security-cross.md` → `specs/rules/security-cross.md`
- [ ] `docs/rules.md`'s "What ships" row and `README.md`'s security bullet
- [ ] `runtime/CHANGELOG.md`'s `## [Unreleased]` section: the four ID changes, their successors, the deprecation and removal versions, the waivers they expire, and the pinned-`security-backend.md` double report
- [ ] Confirm by reading the surface filter (`framework/bootstrap/ductus.md`, "Rule-file surface filter") that frontend-only, backend-only and cross-only `[rules] surfaces` settings each keep the row

- **Done when**: the manifest, both documents and the changelog name the file, and the audit families that check the manifest pass.

## 8. Re-read until clean (AC10)

- [ ] Parallel fresh agents, one per cluster (logging, dependencies, authentication, records), each read-only and probing all four project shapes under 008's severity bar
- [ ] Apply each fix that implements a decision. Record a refinement as an owner's call, and a pre-existing gap with its owner's call. Repeat until a round has no MUST or SHOULD

- **Done when**: a round reports no MUST or SHOULD, and every finding has a disposition.

## 9. Discharge the cross-spec impact

- [ ] Block-quoted signposts in 008's and 017's `spec.md`, each linking this spec. A block quote induces no `dependencies:` edge, so 008's does not close a cycle
- [ ] Tick 008's task 32 follow-on item, task 40's first, fourth and fifth items, and task 42's item, with `mark-task`
- [ ] Run `derive-dependencies` and read its cycle and drift reports

- **Done when**: `check-review-gate` on 077 reports both impacts discharged, `derive-dependencies` reports no cycle, and 008's four items are ticked.

## 10. Local gate and commit

- [ ] On a fresh clone of HEAD plus the patch: `cargo build --release --locked`, `cargo test --release --locked --no-fail-fast`, and every step of `framework-checks.yml`, `generators.yml` and `runtime.yml`
- [ ] Commit only this work's paths, then run `scripts/audit/run-all.sh` and `git show --stat`

- **Done when**: every gate step exits 0 with its result read, and the commit holds exactly the planned paths.
