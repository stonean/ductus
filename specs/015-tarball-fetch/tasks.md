---
title: "015-tarball-fetch — tasks"
next-task: 8
---

# 015 — Tarball Fetch Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 7. Trim the framework archive to the files a run reads (AC14)

- [ ] `.gitattributes` sets `export-ignore` as the plan's **The archive carries only what a run reads** says
- [ ] Family 42, `scripts/audit/archive-contents.sh`, derives the read set and fails in both directions; registered in `scripts/audit/run-all.sh`, `scripts/audit/README.md` and `framework/reference/audit.md`
- [ ] `scripts/tests/test-archive-contents.sh` fails the family on an omitted read file and on an extra archived file, run by `framework-checks.yml` step (ac)
- [ ] `cargo package --list` in `runtime/` is unchanged by the attribute
- [ ] Re-measure the archive figures in the spec's Cleanup and Tradeoffs and the plan's Fetch and extract sequencing and Trade-offs
- [ ] After pushing, the tarball `codeload.github.com` serves for `refs/heads/main` lists exactly the read set

- **Done when**: `git archive HEAD` lists exactly the read set, Family 42 passes on the tree and fails on a fixture in each direction, and the codeload tarball of `main` lists the same files
