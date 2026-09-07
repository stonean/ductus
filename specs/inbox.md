# Inbox

<!-- Rules:
     - Do not frontfill bugs that are not being actively worked on.
     - A bug or omission inside the scope of the spec currently in progress does NOT belong
       here — it becomes a task on that spec's tasks.md. The inbox is for findings with no
       home yet; an in-progress spec is already the home, so an item logged here is routed
       straight back to it (constitution §brownfield-inbox, scope decides the destination).
     - Write specs for areas being actively touched — let adoption spread naturally.
     - As specs are written, items migrate from here into spec updates or new scenarios.
     - Chores (project maintenance with no feature home — lint/formatting cleanup,
       dependency cleanup, repo hygiene) also live here; /groom recognizes them and leaves
       them in place. They clear when done, not by migrating to a spec.
     - The brownfield backlog drains toward empty as adoption completes; incidental
       capture is ongoing, so the file persists while work keeps surfacing issues.
     - Status notes do NOT belong here. Every item must be routable by /groom to one of
       its five routes (rule, spec, scenario, chore, discard); a "where things stand"
       or "what to do next" note matches none of them, so it would be walked and
       re-discarded on every pass forever. Pipeline state is derived — read it from
       /status, tasks.md, and git — not narrated into the backlog.

     Format each item as a checkbox list entry with a brief description and any relevant
     context. Three forms are in use:

     1. Manual entry (via /log) — the simple form below:
        `- [ ] {Brief description of the issue and any relevant context}`

     2. Auto-captured finding (an agent recorded this automatically while working a task,
        per §brownfield-inbox Automatic issue capture). Lead with a category so /groom can
        route it, and include a source pointer:
        `- [ ] {category}: {summary} — {file:line or area} (captured during {NNN-feature})`
        Categories: security, leak (memory/resource), convention, bug, perf, other.
        Security issues and leaks are the highest-priority captures.

     3. Audit finding written by /ductus — stricter form (see
        specs/008-security-rules/spec.md): `- [ ] {Rule ID}: {artifact} does not address — {summary}`.

     When an item is migrated, remove it from this list. -->

- [ ] perf: `analyze_freshness` runs twice on every passing gate — once via `stale_analyze_block`, once via `passing_notices` — each doing a `Repository::discover`, a revparse, and a full tree diff. Hoisting one call in `run_with_lint` and threading the result would halve it. Maps to no loaded rule: the performance rules are all DB/HTTP/cache/pool-shaped. — `runtime/src/primitives/check_review_gate.rs:209` (captured during review of 047-analyze-findings-durability)
- [ ] convention: the take(3) / "(+N more)" path-list rendering is now triplicated in one module (`unexaminable_contracts_guidance`, `stale_review_block`, `stale_analyze_block`). Maps to no loaded rule — `CFG-CONST-001` governs constants shared across modules, not duplicated rendering logic within one. — `runtime/src/primitives/check_review_gate.rs:348` (captured during review of 047-analyze-findings-durability)
- [ ] bug: `reviewed-against` has the same reference-point mismatch 047 is fixing for `analyzed-against`. `/{project}:review` reviews the working tree (its scope is the union of the plan's Affected Files and files modified since the diff base) but records a **commit** sha, so `stale_review_block` can diff a commit the review never examined and report a durable contract as changed when the review had in fact read that exact content. The review half noticed the symptom and chose report-not-block — `unexaminable_contracts_guidance` is precisely that acknowledgement — without fixing the cause. It bites less often than the analyze half did (only `scenarios/*.md` and `data-model.md` are in scope, and those churn far less than `tasks.md`), which is why it has survived. The fix is 047's: record a digest of what was read and compare content, keeping the sha as provenance for the rename exemption. Own spec; do not fold into 047.
- [ ] perf: `compute-review-scope` reports `captured-issues: []` while three items sit uncommitted in `specs/inbox.md` — it diffs the inbox across `diff-base..HEAD`, so items captured this session are invisible to the report that exists to surface them. Same committed-tree horizon 047 just removed from the analyze record, in a third place. — `runtime/src/primitives/compute_review_scope.rs` (captured during review of 047-analyze-findings-durability)
- [ ] convention: the take(3) / "(+N more)" path-list rendering is still triplicated in check_review_gate.rs (`unexaminable_contracts_guidance`, `stale_review_block`, `stale_analyze_block`). Maps to no loaded rule — CFG-CONST-001 governs constants shared across modules, not duplicated rendering within one. — `runtime/src/primitives/check_review_gate.rs:348` (captured during review of 047-analyze-findings-durability)
- [ ] perf: `compute-review-scope` reports `captured-issues: []` while items sit uncommitted in `specs/inbox.md` — it diffs the inbox across `diff-base..HEAD`, so issues captured this session are invisible to the report that exists to surface them. Same committed-tree horizon 047 just removed from the analyze record, in a third place. — `runtime/src/primitives/compute_review_scope.rs` (captured during review of 047-analyze-findings-durability)
- [ ] convention: the take(3) / "(+N more)" path-list rendering is triplicated in check_review_gate.rs (`unexaminable_contracts_guidance`, `stale_review_block`, `stale_analyze_block`). Maps to no loaded rule — CFG-CONST-001 governs constants shared across modules, not duplicated rendering within one. — `runtime/src/primitives/check_review_gate.rs:348` (captured during review of 047-analyze-findings-durability)
