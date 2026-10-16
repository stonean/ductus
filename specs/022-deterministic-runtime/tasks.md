# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 145. Prune 022's spent tasks, close it, and release 0.59.7

No runtime behavior changed in this round — doc comments, a script and a schema label, and `runtime/README.md` — so the 0.59.7 changelog section records the split into 066 and 067, the constitution's canonical-sources row now naming 067's data model, the corrected scenario-link bullet, the doc-comment and label corrections, and the crate readme's rewrite.

- [x] Prune 022's spent task sections with `/ductus:prune` (tasks.md is ephemeral; git keeps them)
- [x] Review and analyze 022, read its data model in full, and run the completion gate
- [x] Ask the operator once to approve 066, 067 and 022 to done and release 0.59.7, saying it became two new specs because one would have left both data models within a few KB of the limit
- [ ] Cut 0.59.7 per the AGENTS.md release entries: bump version, runtime/Cargo.toml and the changelog heading; one build without --locked; full local gate and `scripts/audit/run-all.sh` after committing; tag; `git push --atomic origin main ductus-v0.59.7`; `git ls-remote`; read every workflow run including verify-published and verify-installer; expect 13 assets

- **Done when**: 022, 066 and 067 are done, 022's tasks.md holds no spent sections, and ductus-v0.59.7 is published with every workflow run green.
