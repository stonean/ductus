---
spec: 048-govern-acquired-runtime
last-run: 2026-11-22T04:36:06Z
reviewed-against: 2b306a6923d989ff025050f4c00b5059077b6b2d
diff-base: d77a904dc8ad4e09cbefffd6f07854e4b9d44c41
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 3
scope: 11
unread:
  - framework/bootstrap/ductus.md
  - framework/bootstrap/govern.md
  - install.sh
  - runtime/CHANGELOG.md
  - scripts/audit/README.md
  - scripts/audit/permission-wildcard-position.sh
  - framework/constitution.md
  - framework/reference/clarify.md
excluded: 22
contract-breaks: 0
skipped-passes: []
fix-rounds: 0
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: bdf3bb0f65012793f70ad57a26ac55c823ceaf42106452501094c8db8a4e04d0
  scenarios/fetch-archive-reads-its-proxy-once.md: a14dcf0651874295a292eb8a0f56e009f27cc4ded857a7b55283755c1bebef1a
  scenarios/pin-is-readable-when-acquisition-needs-it.md: fb1368d6a85c6291628b23755e70841e55e1919235e7cbda6393f9e8402ae518
  scenarios/pointer-follows-the-store.md: 350974b3c619868c66f1c5459fbe94d7d0a3c88a750bbf5fcb4cdef963709e5f
  scenarios/release-halves-publish-together.md: 0448ad2cb2df94e5d6b97ce4660345e26619af68d5f05da29d8b8dd8a011306f
  scenarios/retired-namespace-tools-are-off-limits.md: 5ef3496d8bdb180f65970682d1070cd217acd391127cfbf05fa8ffd7032be637
  scenarios/state-a-version-checks-the-pin.md: af9f474c87d09ad7ddb9f1433e04fc42574fe5f4900375268e2db3aaadeff389
  scenarios/state-b-continues-in-session.md: eaf650d5444e8d060e46a97668779bfcf648b086b77bb6ba881afca0759f947a
criteria-judged: 24
criteria-met: 24
criteria-verdicts:
  - label: AC1
    digest: 51825eedf014f8deb0ff0d83bd2297fd1893231fe2611483982446e6a72734c1
  - label: AC2
    digest: 4aee012e0ec0cce84b42c5cc7f58c8213ff958465ce0a16a769dd96122f10174
  - label: AC3
    digest: 0b1239463a253872e95c73a1bc0f631b3d0363593530581f2ee40bc3687854f9
  - label: AC4
    digest: 9fb46eca93b1a5a2b7cdea9fe9030ee356ea1d8cfe3d46217aa8dd22ed7e6035
  - label: AC5
    digest: e3fa81a1c8ac8b5986c799b5d097f865a5c4101c2f340cbb3a6b220e79ba44bb
  - label: AC6
    digest: 69e3e8b60c25be2a2b843362f77da88c64caeaadd0bb6f408b335bc9a38247aa
  - label: AC7
    digest: 277b2e6fe7aed901af385f538b2211d9383c2f4a44a08bf35033a3210330d87d
  - label: AC13
    digest: 3a71fff97749e1a25e620861833130b61e5e614e623cca35621392e238914889
  - label: AC15
    digest: bbd81b47d1922f3c32e33a55393a1d6136a490cf6348137b851369aca0aaa56b
  - label: AC8
    digest: 5dcf70baedd36b0a69719e6d4b3e04ba0b3e550765fc687d401182172f990382
  - label: AC9
    digest: 7bb74ff68efc376ab915150e4c5e04612a07ddc165ddbce99f8498134dcd03b6
  - label: AC10
    digest: aeb725a8763ae68fe2c2196eb56bb18ff1f51dcb04e5ccde8ebc73fc57499b1b
  - label: AC11
    digest: 7f30d73f2ab1a4f4e37891d04aa56b0ce207f1c90e3dbceda85ba220906102e3
  - label: AC16
    digest: d11418ef59404152f03483eb77d5512d776d276656c9608211c1fe2f544e95c0
  - label: AC18
    digest: 8cbb884c927c3e17b24b8dcaf3fe745a2267b809e5a7425b4e11b36f4288df06
  - label: AC19
    digest: ea91a2d9df38705a09468e934a07a464f5c84f013e8973fabaeea210f2e46c65
  - label: AC20
    digest: 052cfc68c331d531ec59352caec61b54bc7fc4ce24af737f2b62d1d0ef89657b
  - label: AC21
    digest: 061b5069407da21acfca55cfbf36fe258302eadd419c7670ee263123e324a4f3
  - label: AC22
    digest: 176727510b1477102605ce670de0e67e707d9844d9c1943875dec3f5feed3229
  - label: AC23
    digest: 885d72219a2dfbcae71e59ea020e9cfcedc114d7b0a162fd2040400859d5ca60
  - label: AC17
    digest: 6eb336bdc53add59590803a7fc78d8cbac65361ea9f0a5d5aa0c270f39cfe1bd
  - label: AC12
    digest: c61f57e704fc826719b0ab4e53e94bf8ec27dfec58ddba2a5d0e3224944e9a98
  - label: AC24
    digest: 0b05377f4b194de86bcc540dbeae02fa5c32192bda0b4dfbd66f970c650c715b
  - label: AC25
    digest: c33f509adff291eecc5db882582136ae53a23583c020930173d179d7bce0a88f
blocking: false
dispositions:
  fixed: 0
  deferred: 0
  waived: 16
  undispositioned: 0
waivers:
  - rule: BE-TIMEOUT-001
    file: framework/bootstrap/hooks/ductus-pre-commit
    reason: "Owner, 2026-11-12 (048 tasks.md task 30) and 2026-11-13 (this review): the hook's derive-dependencies, derive-references, check-corpus-links and check-artifact-size calls stay unbounded, since a fixed bound could kill a large repository's derive, which is slow and not hung; the guard's four probes are bounded by its watchdog."
    waived-at: 2026-11-13T14:31:31Z
    waived-by: andy@stone.dev
decisions:
  - key: "contract: prune-plan's apply writes plan.md before its reopen check can fail, against the plan's 'each error writes nothing' — `runtime/src/primitives/prune_plan.rs:264`"
    outcome: deferred
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "contract: prune-tasks' all + apply walk rewrites tasks.md before its status read can fail — `runtime/src/primitives/prune_tasks.rs:177`"
    outcome: deferred
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: prune-plan rebuilds the stored-decision key inline beside write_analysis::finding_key — `runtime/src/primitives/prune_plan.rs:305`"
    outcome: deferred
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "contract: 041's spec, plan, data-model, scenarios, prune.md, analyze.md and the constitution's new text disagree on the reopen's timing, AC19's trigger, what prune touches, contributor knowledge's destinations and the design-record list's home — `specs/041-task-pruning/spec.md`"
    outcome: deferred
    target: specs/041-task-pruning/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 052's review.md calls its 7 unread files unchanged since the full review; all 7 changed — `specs/052-spec-supersession-and-consolidation/review.md:32`"
    outcome: deferred
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "claim: 052's spec says prune destroys tasks.md; since 041 it also removes plan sections — `specs/052-spec-supersession-and-consolidation/spec.md:65`"
    outcome: deferred
    target: specs/052-spec-supersession-and-consolidation/tasks.md
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: common/mod.rs's stated reason for concurrent_sessions.rs's own spawn helper is inexact — `runtime/tests/common/mod.rs:22`"
    outcome: waived
    reason: that file removes the same variables and then sets each child's identity; converging it on ductus_command is a refactor with no behavior at stake
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: a redundant `||` in a crlf assertion — `runtime/tests/crlf_preservation.rs:210`"
    outcome: waived
    reason: cosmetic, and predates the window
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: the crlf prune-tasks cases do not assert that the rewrite happened — `runtime/tests/crlf_preservation.rs:196`"
    outcome: waived
    reason: their inputs do trigger the rewrite, so a CRLF regression fails them; predates the window
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "test: the walker prune test's doc says the preview reports a missing plan without asserting it — `runtime/tests/walker.rs:829`"
    outcome: waived
    reason: prune_plan's a_missing_plan_is_reported_not_an_error pins that; the walker test's subject is the walk continuing
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18`"
    outcome: waived
    reason: predates the window; consolidating test helpers is a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "dead code: dispatch_primitive's gate-confirm arm is unreachable in production — `runtime/src/interpreter/mod.rs:1083`"
    outcome: waived
    reason: the registry-pin test requires every registry primitive to dispatch; removing the arm means exempting gate-confirm from that pin, a design change outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "reuse: the exec session report joins feature and scenario by hand beside SessionTarget::display — `runtime/src/interpreter/mod.rs:875`"
    outcome: waived
    reason: a SessionPeer is not a SessionTarget, and a shared helper for two three-line joins would be premature
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "naming: check_stuck mixes project-root and work-tree paths, and its _rel names mean work-tree paths — `runtime/src/primitives/check_stuck.rs:48`"
    outcome: waived
    reason: both are correct as written and pinned by the subdirectory test; a read_at_commit sibling is a refactor outside 048
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "errors: InvalidSlug and InvalidPath print a rejected value raw, a newline included — `runtime/src/primitives/mod.rs:332`"
    outcome: waived
    reason: the value reaches callers inside a JSON error envelope, which escapes it
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "errors: a consolidation given only path or scenario is refused naming feature — `runtime/src/primitives/retarget_sessions.rs:146`"
    outcome: waived
    reason: the refusal is correct and names an argument the call needs; which one it names first is cosmetic
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 058's review.md cites plan.md line 274 for 275 and says no 022 file changed — `specs/058-findings-route-at-discovery/review.md:83`"
    outcome: waived
    reason: a review record is pinned to its reviewed-against sha and is regenerated by 058's next review
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "record: 058's plan calls its write-analysis step 'now step 19' — `specs/058-findings-route-at-discovery/plan.md:347`"
    outcome: waived
    reason: it records 058's own renumbering at the time, correct as history
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "inbox: an item's fix list predates 041's prune — `specs/inbox.md:31`"
    outcome: waived
    reason: /groom re-reads each item when it routes it; the inbox is not a live artifact
    decided-at: 2026-09-30T16:32:54Z
    decided-by: andy@stone.dev
  - key: "claim: 067's Overview says no runtime behavior changes, against its 18 runtime commits — `specs/067-analyze-and-audit-checks/plan.md`"
    outcome: deferred
    target: specs/067-analyze-and-audit-checks/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "migration: an entry added after another but sorting before it at the same introduced_in is never run for an adopter whose last_applied names the later-sorting one — `scripts/audit/migration-coverage.sh`"
    outcome: deferred
    target: specs/027-bootstrap-migration-registry/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "errors: rewrite-spec-links checks the path escape before the name grammar, so a `to` failing both is refused as InvalidPath — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: contrived input; refused before any write either way, and the error names the more serious defect, the escape
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "concurrency: the read-all-then-write phase widens the window in which another session's edit between read and write is overwritten — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: waived
    reason: "contrived timing: reading the 656 spec-root markdown files takes about 0.02 s; BE-TXN-002 concerns a service's persisted state, and §concurrent-features says sessions in one tree are not isolated from each other's edits"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "test: check_artifact_size.rs's two permission tests return silently as root (predates the window, 5b575782) — `runtime/src/primitives/check_artifact_size.rs`"
    outcome: deferred
    target: specs/063-oversized-artifacts-warn-with-a-fix/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "ci: lint-release-ordering.sh accepts a tag-check step with `if: false` or `continue-on-error: true` and a publish with `if: always()`, and refuses a check inside publish — `scripts/lint-release-ordering.sh`"
    outcome: waived
    reason: "contrived: each accepted form is a deliberate disabling edit, not the reasonable-looking edit the lint exists for, and a condition it cannot evaluate would still pass an extended lint"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: check-release-tag.sh strips all whitespace from `version`, so a two-line file collapses — `scripts/check-release-tag.sh`"
    outcome: waived
    reason: the collapsed value refuses every real tag, and Family 20 names the extra line in the same audit job publish needs, so nothing publishes and the cause is reported
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "test: the Family 19 parity test returns ok unrun outside a git checkout — `runtime/tests/mechanical_sweep_parity.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: since ced8c7ca a linked spec root is listed in `skipped` as not followed while its files are rewritten (walkdir reports the followed root entry as a link), so examined-and-nothing-pointed-here can never be reported — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: 051 AC22 counts two cases the result names and misses the spec-root symbolic link ced8c7ca added to `skipped` — `specs/051-branch-scoped-spec-numbering/spec.md`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "doc: the reason a spec-root link is not rewritten, that the write would turn it into a copy, holds for a file link only, not a directory link — `runtime/src/primitives/rewrite_spec_links.rs`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "doc: the fold reference's stop clause after the new symlink sentence reads as being about a symbolic link — `framework/reference/fold.md`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: the docs say a symbolic link to a markdown file is listed, while the code tests the link's own name for `.md` — `specs/051-branch-scoped-spec-numbering/scenarios/the-spec-root-is-read-before-the-first-write.md`"
    outcome: deferred
    target: specs/051-branch-scoped-spec-numbering/tasks.md
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "bug: check-orphaned-references reads a trailing-star glob as a missing path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: predates the window; a loud false finding, not a miss; no instance exists; the fix is not mechanical because bold-wrapped paths rely on the trim
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: check-orphaned-references would block on a FIFO at a referrer path — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: contrived input
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: is_path_char is ASCII-only — `runtime/src/primitives/check_orphaned_references.rs`"
    outcome: waived
    reason: the names the check matches are ASCII by grammar; predates the window
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "robustness: extract-archive leaves a zip entry with no Unix mode at the umask default — `runtime/src/primitives/extract_archive.rs`"
    outcome: waived
    reason: "extends 050's stored discard: no archive ductus extracts is a zip (048 AC23)"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "test: installed-file-modes.py's M7 and M8 survive off macOS while EXPECTED_SURVIVORS is empty — `scripts/mutation/installed-file-modes.py`"
    outcome: waived
    reason: disclosed in the file; the lists are run on macOS, and run.py has no per-platform model
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: 008's single-host-appliance-rule-fit scenario argued from a since-removed Rationale — `specs/008-security-rules/scenarios/single-host-appliance-rule-fit.md`"
    outcome: waived
    reason: "already corrected at HEAD: 554d0ee3 rewrote the scenario, which now states the rules carry no Rationale and argues from none"
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: "claim: 021 AC7(d) and spec.md:94 still say a graceful-fallback pattern — `specs/021-runtime-boundary/spec.md`"
    outcome: waived
    reason: the pointer is the command's fallback declaration, as 021's plan says; nothing became untrue
    decided-at: 2026-10-29T23:33:22Z
    decided-by: andy@stone.dev
  - key: other — a non-UTF-8 proxy variable reads as unset, so that scheme's fetches go direct — `runtime/src/primitives/fetch_archive.rs`
    outcome: waived
    reason: "Owner, 2026-11-12 (048 tasks.md task 30): contrived, the fix reaching every proxy read."
    decided-at: 2026-11-13T14:31:31Z
    decided-by: andy@stone.dev
  - key: convention — the bootstrap interpolates {tempdir} unquoted, so a TMPDIR holding whitespace splits the operands — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: "Owner, 2026-11-12 (048 tasks.md task 30): the plan states that a TMPDIR holding whitespace is unsupported."
    decided-at: 2026-11-13T14:31:31Z
    decided-by: andy@stone.dev
  - key: bug — a hung probe or a failed mktemp halts the hook with 'It has no <primitive> subcommand', naming the wrong cause — `framework/bootstrap/hooks/ductus-pre-commit`
    outcome: waived
    reason: "By design: the hook reads a probe its watchdog ended as a failed probe, as its comment says, and halts either way naming the binary and its --version; a message per cause is a new branch and test, not a mechanical fix."
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — State A's pointer check adds a restart notice for any re-make, a missing or dangling pointer included — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: The extra restart is the conservative direction and never leaves a stale server running; telling the cases apart would rest on how the live server was spawned, which the procedure cannot see.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: "other — nothing fails if the pointer procedure regresses: its one automated artifact is the measurement script, which never fails on an outcome — `scripts/measure-replace-under-server.sh`"
    outcome: waived
    reason: Pointer materialization is host-executed procedure prose, which no unit test can run; its composition test is the real adopter bootstrap the release program owes, and runtime.yml's measurement for Windows.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: convention — the measurement harness passes certutil a cygpath -w path where the procedure passes the Git Bash path — `scripts/measure-replace-under-server.sh`
    outcome: waived
    reason: Git Bash converts a POSIX path argument for a native program itself, so cygpath -w is the conversion the procedure's spelling already gets; the case difference is fixed in the procedure.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — the pointer's recorded rationale lost the Claude ${VAR} alternative when the Auggie, Antigravity and OpenCode sweep deleted its Resolved Question — `specs/048-govern-acquired-runtime/spec.md`
    outcome: waived
    reason: The pointer stays the one path Claude's .mcp.json, Pi's bridge, the hook and State B's CLI calls all name; whether Claude's ${VAR} expansion should replace it there is a design change outside this review.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: convention — HostRow::CommandOrSkill and the Command/skill path row label still name skills no remaining agent has — `runtime/src/primitives/host_config.rs`
    outcome: waived
    reason: "Naming only: the variant carries both remaining agents' command paths, and a rename touches the runtime and its tests for no behavior."
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — no end-to-end test resolves a claude-style commands/{project} command under a non-default host block since the exec fixtures became Pi-shaped — `runtime/tests/exec_subprocess.rs`
    outcome: waived
    reason: host_config.rs's unit tests hold the candidate order that end-to-end case relied on.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — Family 2, manifest-parity.sh, is now a single-file check — `scripts/audit/manifest-parity.sh`
    outcome: waived
    reason: It says so in its header and in its stderr count, so it reads as narrowed, never as a clean check of more.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — three migration procedures and runtime-store-path's target_paths were edited in place rather than by an ordered new entry — `framework/migrations.toml`
    outcome: waived
    reason: The edits removed only the dropped agents' paths, which no supported adopter has, so nothing an adopter who already ran them needs is lost.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — the constitution is 49,969 bytes of its 50,000-byte ceiling — `framework/constitution.md`
    outcome: waived
    reason: It is within the ceiling scripts/lint-governing-size.sh holds, and the next addition retires a rule instead, as spec 072 intends.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: convention — DUCTUS_FETCH_ALLOW_INSECURE_HOSTS reads a non-UTF-8 value as an empty allowlist — `runtime/src/primitives/fetch_archive.rs`
    outcome: waived
    reason: It fails closed, exempting no host, the class of the proxy read the owner waived on 2026-11-12.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: security — DUCTUS_FETCH_ALLOW_INSECURE_HOSTS lets an operator reach a named host over plain http — `runtime/src/primitives/fetch_archive.rs`
    outcome: waived
    reason: An opt-in escape hatch an operator sets, documented in docs/runtime.md and 022's scenarios, that predates this window.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: convention — the release workflow's retry honors Retry-After only as seconds and keeps every redirect hop's headers — `.github/workflows/runtime-release.yml`
    outcome: waived
    reason: BE-RETRY-003's hint is advisory and the comment states the seconds-only form; tail reads the last header written, the final hop's when it carries one.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — Screen::new's Proxy::all filter changes no fetch's outcome, since run refuses a fetch whose proxy parse failed — `runtime/src/primitives/fetch_archive.rs`
    outcome: waived
    reason: A unit test, only_named_proxies_reqwest_accepts_are_exempt, builds a Screen from a rejected proxy and pins the filter, which keeps the screen right whatever order run checks in.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — CFG-ENV-008 and CFG-ENV-001 are met by owner decisions recorded in the proxy-once scenario, not by the code's shape — `specs/048-govern-acquired-runtime/scenarios/fetch-archive-reads-its-proxy-once.md`
    outcome: waived
    reason: Those decisions are the scenario's Resolved Questions, which the code implements; nothing is outstanding.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — whether Pi's shell tool applies the {probe-timeout} the bootstrap sets on every --version probe is unverified — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: The bootstrap names the bound and how a host applies it, as BE-TIMEOUT-001 asks of a design; how one host's shell tool honors a timeout is that host's behavior, which the bootstrap cannot change.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: other — a new --waive's files include those of low-confidence findings of its rule, which need no waiver, so this run's first write held back the fetch-archive DNS finding to keep the owner's hook waiver to the hook — `runtime/src/primitives/write_review.rs`
    outcome: waived
    reason: Pipeline machinery outside 048, observed while reviewing framework work; this record is right, since its hook waiver was stored by a write holding no other BE-TIMEOUT-001 finding and the final write passes it as applied.
    decided-at: 2026-11-13T14:31:31Z
    decided-by: rule
  - key: security — the runtime archive's `.sha256` sidecar is fetched from the same release as the archive, so Branch 2 step 5 catches corruption and a swapped archive only while the sidecar is untouched; no signature or provenance attestation is checked — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: the spec's Resolved Questions make the sidecar digest the acquisition gate, and SEC-DEPS-001 asks for a checksum the build verifies, which the step does; an independent signature would be a new requirement outside this review
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: security — the self-update check overwrites the installed command from `{tempdir}/ductus.md.upstream` and the framework archive installs commands and `.githooks/ductus-pre-commit`, all fetched over verified TLS with no digest and selected by `main` or a release tag, and `data-model.md` says `install.sh` is piped to `sh` with no sidecar — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: documented design (061's source resolution; GitHub's source tarballs carry no sidecar; `data-model.md` records a post-release byte comparison for `install.sh`), and SEC-DEPS-001 governs a project's own build inputs, not the adoption procedure
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: security — the Claude seed keeps `.exe` twins (`Bash(.ductus/bin/ductus.exe fetch-archive *)` and eleven more subcommands, `Bash(chmod +x ~/.ductus/bin/ductus.exe)`) that run a repo-relative path no step creates, compares or re-makes on Linux or macOS, so a tracked `.ductus/bin/ductus.exe` would run unprompted if an agent were steered to call it — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: owner decision of 2026-11-19 recorded in 048's Resolved Question on Windows support keeps the twins for a `[runtime] path` Windows user and for `runtime-probe-parity.sh`
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: security — MCP wiring leaves an existing `ductus` entry byte-unchanged whatever its `command` names, so a `.mcp.json` entry pointing at another binary is reported as already registered — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: AC5 requires additive, no-clobber merging, the `runtime-store-path` migration rewrites the bare pre-048 command, and the host's own approval of a project MCP server gates what it spawns
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "security: `publish` sets `CARGO_REGISTRY_TOKEN` in the environment of `cargo publish --locked`, whose verification build runs every dependency's build script and proc macro with the token in its environment; the lockfile and `cargo-audit` gate bound the exposure but nothing scopes it. — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: no loaded rule reaches a CI secret's exposure to build code (`BE-DATA-003` asks only for runtime sourcing, which the step-scoped `env` meets); the dependency set is pinned by `Cargo.lock`, gated by `cargo-audit` before `publish`, and the pipeline's own machinery is not what this review's spec changed (AGENTS.md waives such observations).
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "security: the release's `.sha256` sidecars sit beside the archives in the same release and carry no signature or build-provenance attestation, so acquisition's digest check detects corruption but not replacement of an asset by whoever can write the release. — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: constitution §runtime-boundary's acquisition invariant and 048's acquisition contract specify digest verification against the sidecar, no loaded rule requires publisher signing of a release asset (`BE-DEPS-004` governs registry dependencies), and adding attestation is a design change outside this window.
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "other — the bootstrap keeps 12 `.exe` seed twins and the store probe's `.exe` forms though no workflow publishes or fills a Windows store; kept by the owner's 2026-11-19 decision recorded in `spec.md`'s Resolved Question \"Is Windows a supported platform?\" and held by `runtime-probe-parity.sh` — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "owner decision 2026-11-19, recorded in 048's Resolved Question \"Is Windows a supported platform?\": the `.exe` twins stay for a `[runtime] path` user and `runtime-probe-parity.sh` requires the probe's twin"
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "perf: `runtime-release.yml` compiles `cargo-zigbuild` and `cargo-cyclonedx` with `cargo install` on every tag run, while its `cargo-audit` job takes a prebuilt binary for the stated reason that a cargo install dominates wall-clock — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: both jobs run only on a tag push and sit inside their timeout-minutes bounds (35 and 10); they are not a hot path, and the pins are exact (`--version`, hash-checked ziglang)
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "perf: the `lint` job and the ubuntu leg of `build-and-test` in `runtime.yml` share one `actions/cache` key (`runner.os` plus the `Cargo.lock` hash), so whichever finishes first saves `runtime/target` and the other's save is skipped — `.github/workflows/runtime.yml`"
    outcome: waived
    reason: benign, the cache is only an accelerator and every step passes on a cold cache; splitting keys would double stored target directories for no correctness gain
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "other: `runtime-acquisition.yml`, which `verify-published` calls straight after `release-assets`, fetches with no retry, although `verify-installer`'s comment documents that a just-uploaded asset can answer 404 for a brief window — `.github/workflows/runtime-acquisition.yml`"
    outcome: waived
    reason: both jobs gate nothing, the last five release runs (`ductus-v0.59.7` to `ductus-v0.59.11`) passed, and the documented recovery is a manual re-run or `workflow_dispatch`; adding a retry loop here is a design change owned by 048's release scenario
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "other: the same verify, list, extract, install and `--version` probe sequence is written out in `runtime-release.yml` `acquire`, `runtime-acquisition.yml` and (as a packaging probe) `runtime.yml`, so a change to Branch 2 of the bootstrap must be applied in each — `.github/workflows/runtime-acquisition.yml`"
    outcome: waived
    reason: the acquisition workflow's header states it is the executable copy of the bootstrap sequence, `acquire` reads staged artifacts where the other fetches the published URL, and `verify-published` already reuses the workflow by `workflow_call` rather than restating it
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "convention: the shipped hook falls back to a bare `ductus` on PATH after `.ductus/bin/ductus`, while 048's Resolved Question rejects PATH as any fallback; it only widens where a hand-installed binary is found and no document tells anyone to install onto PATH, so AC12 is not touched — `framework/bootstrap/hooks/ductus-pre-commit`"
    outcome: waived
    reason: the hook is out of this run's scope, its comment says the PATH lookup is for a hand-installed binary only, and the Resolved Question governs what /ductus acquires and registers, not a hook's lookup
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "convention: nine `done` files state a `markdown-only path` in the present tense (`specs/013-text-first-artifacts/scenarios/criterion-identifiers.md`, `specs/023-govern-refinement/scenarios/extend-existing-scenario-task.md`, `specs/040-configurable-specs-dir/spec.md` and its scenarios `command-prose-resolves-spec-root.md` and `spec-root-rule-stated-once.md`, `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md`, `specs/046-scenario-open-question-visibility/spec.md`, `specs/056-bootstrap-archive-boundary-split/spec.md`, `specs/060-exec-analyze-assesses-each-loaded-rule/scenarios/an-assessment-sees-the-loaded-rule-files.md`); a stricter reading of AC24's last sentence would count them — `specs/040-configurable-specs-dir/spec.md`"
    outcome: waived
    reason: "AC24 sweeps statements that the runtime is optional, which none of the nine makes: each describes the markdown-only path spec 079 retired, whose own sweep owns that wording, and the rewording would edit other specs' done scenarios, a change of their contracts rather than a mechanical fix"
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: other — no Windows run has had a host write State B's --*-json files into {tempdir}, which the CLI must find inside a staging directory — `framework/bootstrap/ductus.md`
    outcome: waived
    reason: "No published or tested platform is Windows since AC25 (owner decisions 2026-11-17 and 2026-11-18, 048's Resolved Question \"Is Windows a supported platform?\"), and the owner decided not to run a real Windows agent session; the CLI's staging-directory check itself is driven on the published platforms. The deferral lapsed with that session's task."
    decided-at: 2026-11-19T13:57:32Z
    decided-by: rule
  - key: "convention: 048's AC22 annotation says the rest of the acquisition sequence prompts for nothing on Claude, which a reader may take to cover the Windows [runtime] path digest — `specs/048-govern-acquired-runtime/spec.md`"
    outcome: waived
    reason: 048's own review reads its spec in this window, and the claim concerns the acquisition sequence, which no Windows host runs since 048's AC25
    decided-at: 2026-11-22T04:36:06Z
    decided-by: rule
  - key: "convention: Permission Setup's closing rationale against a prompt on every bootstrap stands beside the Windows configured binary's per-run digest prompt — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "the agent's recommendation, adopted by the operator, 2026-11-21: that prompt over a trailing wildcard or no entry, and the same paragraph states it"
    decided-at: 2026-11-22T04:36:06Z
    decided-by: rule
  - key: "other: the pinned certutil -hashfile .ductus/bin/ductus.exe SHA256, a Git Bash relative path, is unmeasured on Windows, where the measurement harness passes a cygpath -w path — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the procedure passed Git Bash paths to certutil before this change too, and no Windows runner exists since 048's AC25 (owner, 2026-11-19)
    decided-at: 2026-11-22T04:36:06Z
    decided-by: rule
  - key: "bug: mid_wildcard does not flag a pattern whose first token is a wildcard, such as Bash(* ls) — `scripts/audit/permission-wildcard-position.sh`"
    outcome: waived
    reason: the predicate predates this change and serves both subjects, and no allow entry either set ships has that shape
    decided-at: 2026-11-22T04:36:06Z
    decided-by: rule
  - key: "other: Family 29's seed parse is a third copy of the Agent Registry row read that Families 32c and 14 also carry — `scripts/audit/permission-wildcard-position.sh`"
    outcome: waived
    reason: each family is a standalone script by the audit contract and lib.sh holds bash helpers only, so a shared reader is a new surface rather than a mechanical fix; a broken parse reports findings, never a pass
    decided-at: 2026-11-22T04:36:06Z
    decided-by: rule
  - key: "convention: tasks.md task 32's prose names runtime.yml's windows-unelevated job, which task 33 removed, and says write-analysis refuses while the task is open, though every box is ticked — `specs/048-govern-acquired-runtime/tasks.md`"
    outcome: waived
    reason: tasks.md is ephemeral and tasks 32 to 34 are spent; no gate, criterion or later reader acts on a ticked task's prose
    decided-at: 2026-11-22T04:36:06Z
    decided-by: rule
---

# Review — 048-govern-acquired-runtime

## Summary

A later review of the window 023's review read in depth: the same commit, c9657c88, reopened both specs, and 048's window holds nine entries; the record's scope of 11 adds framework/constitution.md and framework/reference/clarify.md, another session's uncommitted edits made during this run, which it did not read and lists unread. Six of them (framework/bootstrap/ductus.md, framework/bootstrap/govern.md, install.sh, runtime/CHANGELOG.md, scripts/audit/README.md, scripts/audit/permission-wildcard-position.sh) are byte-identical to what 023's review (specs/023-govern-refinement/review.md, committed in d2e834fe) read over that commit, so this run did not re-read them whole: ductus.md was read in part, the c9657c88 hunks with ## Permission Setup, the runtime-detection sections through #### Pointer materialization, and the Agent Registry claude row, as 048's deliverable; all six are listed unread. The five passes read 048's own three entries whole (spec.md, tasks.md and the pointer-follows-the-store scenario, its claims checked against the tree) against the five rule files review loads, each read whole: no rule violation. All 24 criteria judged one by one and met. The ten stored decisions keyed to an in-scope file were each checked against that file and still stand, so each disposes of its observation; the five 023 decisions on 048's scope are carried with their text and disposition unchanged, and one new observation is waived by the rule.
 This run wrote no fix. brief-re-read from the run's fix-round base returned only framework/constitution.md and framework/reference/clarify.md, another session's uncommitted edits made after that base, so no round of this run's needed a re-read and none is recorded.

0 fix round(s): 0 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

24 of 24 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: framework/bootstrap/ductus.md Runtime acquisition Branch 2 steps 3-6 (triple table with four rows, sidecar fetch, digest check before install, tempfile+mv -f, chmod +x); no Windows row so a Windows host halts, as AC1's narrowing says; the acquire job in .github/workflows/runtime-release.yml executes the same sequence on four native runners.
- AC2 — met: framework/templates/project/gitignore holds /.ductus/bin/ (the block merge-managed-block installs, Instructions step 5); a scratch git repo seeded with that template and a file at .ductus/bin/ductus reports only .gitignore untracked under git status.
- AC3 — met: ductus.md Branch 2 step 5 verifies before step 6's install and Failure says nothing is written to the store or pointer and the halt's reason names expected (sidecar) and computed digests; the release acquire job's sha256sum -c step likewise fails before install. Procedure prose only, no bootstrap test drives a mismatch.
- AC4 — met: ductus.md Failure: a network failure or unpublished asset aborts with a halt text naming {store-path} and the releases/tag/ductus-v{pin} URL and saying to place the binary by hand and re-run; the unpublished-platform branch of step 3 routes to the same halt.
- AC5 — met: ductus.md MCP wiring writes .mcp.json with command .ductus/bin/ductus, args [mcp], through the five additive cases (missing, no ductus key, existing no-op, no mcpServers, invalid JSON untouched); this repo's tracked .mcp.json carries the repo-relative pointer and no absolute path.
- AC6 — met: ductus.md Branch 2 step 2: a store probe reporting {pin} means no download and the binary byte-unchanged, continuing to the pointer; scripts/measure-replace-under-server.sh store-control/store-running rows are the only store writes, and none runs on a current store.
- AC7 — met: ductus.md Branch 2 step 6 spells cp to {store-path}.tmp.$$ && mv -f over the store and the Closing restart runtime line reports 'Installed {version}'; I ran scripts/measure-replace-under-server.sh against runtime/target/release/ductus with HOME in the scratchpad on macOS: store-running exit 0, server alive after and 2s later, store holds new; runtime.yml's ubuntu/macos build-and-test job runs it, Windows dropped per AC25 narrowing.
- AC13 — met: ductus.md Pointer materialization steps 2-6 compare SHA-256 digests, re-make a missing/dangling/differing pointer by ln -sf then cp fallback (rm of a symlink first, move-aside for a busy pointer) and re-compare, needing no elevation; the measure script run here shows stale copy differs, missing/dangling read differs, current equal, ln -sf repair under a running server equal. The seed's certutil entry is now the exact .ductus/bin/ductus.exe form, consistent with the scenario pointer-follows-the-store; runtime-probe-parity, permission-wildcard-position and permission-entry-shape audits pass.
- AC15 — met: version, runtime/Cargo.toml and the newest ## [0.61.0] CHANGELOG heading all read 0.61.0 and tag ductus-v0.61.0 exists; scripts/audit/version-agreement.sh (Family 20) exits 0; scripts/check-release-tag.sh ductus-v0.60.0 exits 1 naming the mismatch, test-check-release-tag.sh and lint-release-ordering.sh pass, and runtime-release.yml's audit job runs the check with publish needing audit.
- AC8 — met: framework/migrations.toml registers runtime-store-path (0.28.0) with procedure framework/migrations/runtime-store-path.md: step 1 exits silently when .mcp.json is absent, has no ductus server or already names a path, step 2 rewrites command to .ductus/bin/ductus, so a re-run is a no-op.
- AC9 — met: ductus.md Detection mechanism probes ~/.ductus/bin/ductus --version (ignoring PATH) and the Claude seed in the Agent Registry grants exactly Bash(~/.ductus/bin/ductus --version) and its .exe twin; scripts/audit/runtime-probe-parity.sh exits 0.
- AC10 — met: ductus.md State B steps 1-5 acquire, materialize the pointer, wire MCP and add `mcp__ductus__*` permission in the pre-flight pass, then 'State B does not stop here' and continues through {pointer-path} <primitive>, ending in ductus-procedure.md Closing restart; scenario state-b-continues-in-session records it.
- AC11 — met: Every claim holds: no markdown-only-pipeline.yml exists, git history shows it renamed to framework-checks.yml, runtime-release.yml's acquire job (needs audit/cargo-audit/build, gates publish) fetches staged assets and executes the installed binary on four native runners, and runtime-acquisition.yml has workflow_dispatch plus workflow_call and fetches the published assets.
- AC16 — met: framework/constitution.md §runtime-boundary principle 3 reads 'Required, and acquired by the pipeline', the Opt-in invariant is replaced by #### Acquisition invariant, and §text-first-artifacts says only the artifacts are usable standalone; a scan of framework/, docs/, README.md, scripts/, .github/ for runtime-optional wording finds only retirement comments (framework-checks.yml, runtime-acquisition.yml headers).
- AC18 — met: framework/bootstrap/ductus.md Runtime acquisition Branch 1: step 2 no download and nothing written to the store, step 3 version mismatch emits one warning and continues, step 5 and Pointer materialization compare/link against the configured binary; State A repeats the warn-not-halt outcome.
- AC19 — met: .ductus/config.toml sets [runtime] path = "runtime/target/release/ductus"; that file exists (0.61.0) and .ductus/bin/ductus is a symlink ../../runtime/target/release/ductus reporting ductus 0.61.0.
- AC20 — met: .github/workflows/runtime-release.yml: build matrix fail-fast: false with the four published targets, acquire (needs build, fail-fast: false, one leg per target) and publish needs [audit, cargo-audit, build, acquire, sbom], release-assets asserts REQUIRED_TARGETS (four) archives plus sidecars before upload; actionlint on .github/workflows exits 0.
- AC21 — met: Superseded text holds: README.md has no cross-compilation claim and mentions Windows only as a host supplying its own binary through [runtime] path (no published Windows binary named), and docs/runtime.md lists exactly the four published targets and says no Windows binary is published.
- AC22 — met: Annotation checked against the claude settings_template in framework/bootstrap/ductus.md Agent Registry and §Permission Setup (post c9657c88): seeded are `mkdir -p ~/.ductus/bin/` and `mkdir -p .ductus/bin`, `shasum *` and `sha256sum *`, `chmod +x ~/.ductus/bin/ductus`, `~/.ductus/bin/ductus --version` and the Read tempdir rules; no cp/mv/ln/rm/curl/tar entry (cp+mv install and pointer re-make, curl, tar prompt exactly as the annotation and §Permission Setup say); permission-wildcard-position.sh, permission-entry-shape.sh and runtime-probe-parity.sh exit 0.
- AC23 — met: For the four published targets: runtime-release.yml packages 'tar -czf ductus-{target}.tar.gz' and probes/acquires with tar -xzf only, and no unzip/.zip/Expand-Archive appears under framework/, .github/, install.sh or docs/README (only extract-archive's in-process zip support in runtime/src, outside any permission surface).
- AC17 — met: No 'With no ductus runtime registered' (or any no-runtime fallback instruction) remains in `framework/commands/*.md`, framework/bootstrap/ductus.md or ductus-procedure.md (grep of no runtime/not registered/runtime unavailable forms); the reference sections remain under framework/reference/ (their heading is no longer 'Markdown-only', renamed by specs 071/079).
- AC12 — met: Pattern scan of framework/ and README.md for PATH, /usr/local/bin, install -m, cargo install, bare "command": "ductus" finds only prose saying PATH is not consulted (framework/bootstrap/ductus.md Detection mechanism, README.md The runtime) and the migration that rewrites a bare command to .ductus/bin/ductus (framework/migrations/runtime-store-path.md); .mcp.json and framework/bootstrap/ductus.md MCP wiring register .ductus/bin/ductus.
- AC24 — met: specs/021-runtime-boundary/spec.md carries a signpost naming 048's amendment of principle 3, criterion 3 and the CI invariant, with AC1/AC3 annotated; specs/029-bootstrap-runtime-autowire/spec.md carries a post-048 signpost, a Post-completion note and superseded annotations on AC1/4/5/8/10/11/12/14/15/17 (both are done again after their reopen); a scan of the other specs/*/spec.md for optional/opt-in/markdown-only runtime wording finds annotated or unrelated uses only (e.g. 030 AC10 and 022 AC9/AC16 carry 'Superseded by 048').
- AC25 — met: grep -i windows|msvc|.exe|.zip over .github/workflows finds only runtime.yml's comment that no Windows CI job exists; every runs-on/runner in runtime.yml, runtime-release.yml and runtime-acquisition.yml is ubuntu or macos, REQUIRED_TARGETS and both matrices name four targets, and the Runtime acquisition triple table in framework/bootstrap/ductus.md has four rows with the 'a Windows host included' halt text.

## Observations

- security — the runtime archive's `.sha256` sidecar is fetched from the same release as the archive, so Branch 2 step 5 catches corruption and a swapped archive only while the sidecar is untouched; no signature or provenance attestation is checked — `framework/bootstrap/ductus.md` — **waived**: the spec's Resolved Questions make the sidecar digest the acquisition gate, and SEC-DEPS-001 asks for a checksum the build verifies, which the step does; an independent signature would be a new requirement outside this review (decided by the rule)
- security — the self-update check overwrites the installed command from `{tempdir}/ductus.md.upstream` and the framework archive installs commands and `.githooks/ductus-pre-commit`, all fetched over verified TLS with no digest and selected by `main` or a release tag, and `data-model.md` says `install.sh` is piped to `sh` with no sidecar — `framework/bootstrap/ductus.md` — **waived**: documented design (061's source resolution; GitHub's source tarballs carry no sidecar; `data-model.md` records a post-release byte comparison for `install.sh`), and SEC-DEPS-001 governs a project's own build inputs, not the adoption procedure (decided by the rule)
- security — the Claude seed keeps `.exe` twins (`Bash(.ductus/bin/ductus.exe fetch-archive *)` and eleven more subcommands, `Bash(chmod +x ~/.ductus/bin/ductus.exe)`) that run a repo-relative path no step creates, compares or re-makes on Linux or macOS, so a tracked `.ductus/bin/ductus.exe` would run unprompted if an agent were steered to call it — `framework/bootstrap/ductus.md` — **waived**: owner decision of 2026-11-19 recorded in 048's Resolved Question on Windows support keeps the twins for a `[runtime] path` Windows user and for `runtime-probe-parity.sh` (decided by the rule)
- security — MCP wiring leaves an existing `ductus` entry byte-unchanged whatever its `command` names, so a `.mcp.json` entry pointing at another binary is reported as already registered — `framework/bootstrap/ductus.md` — **waived**: AC5 requires additive, no-clobber merging, the `runtime-store-path` migration rewrites the bare pre-048 command, and the host's own approval of a project MCP server gates what it spawns (decided by the rule)
- convention — the bootstrap interpolates {tempdir} unquoted, so a TMPDIR holding whitespace splits the operands — `framework/bootstrap/ductus.md` — **waived**: Owner, 2026-11-12 (048 tasks.md task 30): the plan states that a TMPDIR holding whitespace is unsupported.
- other — State A's pointer check adds a restart notice for any re-make, a missing or dangling pointer included — `framework/bootstrap/ductus.md` — **waived**: The extra restart is the conservative direction and never leaves a stale server running; telling the cases apart would rest on how the live server was spawned, which the procedure cannot see. (decided by the rule)
- other — the pointer's recorded rationale lost the Claude ${VAR} alternative when the Auggie, Antigravity and OpenCode sweep deleted its Resolved Question — `specs/048-govern-acquired-runtime/spec.md` — **waived**: The pointer stays the one path Claude's .mcp.json, Pi's bridge, the hook and State B's CLI calls all name; whether Claude's ${VAR} expansion should replace it there is a design change outside this review. (decided by the rule)
- other — whether Pi's shell tool applies the {probe-timeout} the bootstrap sets on every --version probe is unverified — `framework/bootstrap/ductus.md` — **waived**: The bootstrap names the bound and how a host applies it, as BE-TIMEOUT-001 asks of a design; how one host's shell tool honors a timeout is that host's behavior, which the bootstrap cannot change. (decided by the rule)
- other — the bootstrap keeps 12 `.exe` seed twins and the store probe's `.exe` forms though no workflow publishes or fills a Windows store; kept by the owner's 2026-11-19 decision recorded in `spec.md`'s Resolved Question "Is Windows a supported platform?" and held by `runtime-probe-parity.sh` — `framework/bootstrap/ductus.md` — **waived**: owner decision 2026-11-19, recorded in 048's Resolved Question "Is Windows a supported platform?": the `.exe` twins stay for a `[runtime] path` user and `runtime-probe-parity.sh` requires the probe's twin (decided by the rule)
- other — no Windows run has had a host write State B's --*-json files into {tempdir}, which the CLI must find inside a staging directory — `framework/bootstrap/ductus.md` — **waived**: No published or tested platform is Windows since AC25 (owner decisions 2026-11-17 and 2026-11-18, 048's Resolved Question "Is Windows a supported platform?"), and the owner decided not to run a real Windows agent session; the CLI's staging-directory check itself is driven on the published platforms. The deferral lapsed with that session's task. (decided by the rule)
- convention: 048's AC22 annotation says the rest of the acquisition sequence prompts for nothing on Claude, which a reader may take to cover the Windows [runtime] path digest — `specs/048-govern-acquired-runtime/spec.md` — **waived**: 048's own review reads its spec in this window, and the claim concerns the acquisition sequence, which no Windows host runs since 048's AC25 (decided by the rule)
- convention: Permission Setup's closing rationale against a prompt on every bootstrap stands beside the Windows configured binary's per-run digest prompt — `framework/bootstrap/ductus.md` — **waived**: the agent's recommendation, adopted by the operator, 2026-11-21: that prompt over a trailing wildcard or no entry, and the same paragraph states it (decided by the rule)
- other: the pinned certutil -hashfile .ductus/bin/ductus.exe SHA256, a Git Bash relative path, is unmeasured on Windows, where the measurement harness passes a cygpath -w path — `framework/bootstrap/ductus.md` — **waived**: the procedure passed Git Bash paths to certutil before this change too, and no Windows runner exists since 048's AC25 (owner, 2026-11-19) (decided by the rule)
- bug: mid_wildcard does not flag a pattern whose first token is a wildcard, such as Bash(* ls) — `scripts/audit/permission-wildcard-position.sh` — **waived**: the predicate predates this change and serves both subjects, and no allow entry either set ships has that shape (decided by the rule)
- other: Family 29's seed parse is a third copy of the Agent Registry row read that Families 32c and 14 also carry — `scripts/audit/permission-wildcard-position.sh` — **waived**: each family is a standalone script by the audit contract and lib.sh holds bash helpers only, so a shared reader is a new surface rather than a mechanical fix; a broken parse reports findings, never a pass (decided by the rule)
- convention: tasks.md task 32's prose names runtime.yml's windows-unelevated job, which task 33 removed, and says write-analysis refuses while the task is open, though every box is ticked — `specs/048-govern-acquired-runtime/tasks.md` — **waived**: tasks.md is ephemeral and tasks 32 to 34 are spent; no gate, criterion or later reader acts on a ticked task's prose (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/review.md`
- `docs/analyze.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/review.md`
- `runtime/src/primitives/merge_permissions.rs`
- `runtime/src/schema/descriptions.rs`
- `specs/013-text-first-artifacts/plan.md`
- `specs/016-cross-cutting-rules/spec.md`
- `specs/023-govern-refinement/analysis.md`
- `specs/023-govern-refinement/plan.md`
- `specs/023-govern-refinement/review.md`
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md`
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md`
- `specs/023-govern-refinement/spec.md`
- `specs/023-govern-refinement/tasks.md`
- `specs/050-constitution/scenarios/a-decision-is-attributed-to-whoever-made-it.md`
- `specs/050-constitution/scenarios/a-primitive-is-probed-in-a-local-clone.md`
- `specs/050-constitution/spec.md`
- `specs/050-constitution/tasks.md`

## Unexamined governance

*None.*
