# 007 — Review Fixture Plan

## Technical Decisions

A single-file scope keeps the golden deterministic: the plan's Affected Files
names one in-repo source file, so `compute-review-scope` resolves a stable,
non-empty scope. The harness lands in a commit no spec claims and this
directory in its own, so beside that file a first review reads only this
directory.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `src/reviewed.rs` | Edit | The single in-scope source file the passes review. |
