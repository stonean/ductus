---
section: "Artifact checks"
---

# Status-gated-families-are-not-silent

## Context

`criterion-path-existence` and `body-path-existence` examine a spec only at `done`. Below `done` each reports no finding and no skipped target, so the analysis that satisfies the pre-`done` gate — taken while the spec is `in-progress` — never runs them, and its `unexamined: 0` reads as complete coverage. Measured 2026-10-25 on 0.59.11 and HEAD f24c55c6: a `done` spec with 4 body-path findings gave 0 findings and an empty `skipped` once its status was set to `in-progress`. The scoping to `done` is deliberate (scenarios `criterion-path-existence-family` and `body-path-existence-family`); what is missing is any trace of it in the run's result or record.

## Behavior

- At `in-progress` and at `done`, both families examine the spec with the same grammar, exemptions and skip reasons, so the analysis the pre-`done` gate reads has run them.
- At any other status — `draft`, `clarified`, `planned`, or a value outside the set — each family records one skipped target with reason `status-gated`, its path the spec's `spec.md`, so the result and the `analysis.md` record it feeds say the family did not examine the spec.
- `status-gated` is in the *excluded by construction* class: it recurs on every such run, so it never sets `restricted`, and a stored decision is pruned as it is today.
- A run at `done` is unchanged.

## Edge Cases

- A spec whose criteria and body carry no backticked path: at `in-progress` or `done` the families examine it and report nothing; at any other status each still records its `status-gated` skip.
- The pre-`done` gate's own ordering: the analysis it reads is taken at `in-progress`, where both families now run, so it reports what they found, and the analyze reference's flip-to-`done` measurement is no longer needed.
- A criterion at `in-progress` naming a file the spec has yet to create is an advisory `criterion-path-existence` finding until the file lands. A body path not yet created is `never-existed`, a skip, not a finding.
- Repeat runs over unchanged inputs are byte-identical: `status-gated` is one record per family, never one per candidate.
- A criterion naming a path the repository's own ignore rules ignore, such as `.ductus/session.toml`, which resolves in a working checkout and not in a fresh clone or worktree, is recorded as a `gitignored` skip, excluded by construction, never a finding, so the same spec's result no longer differs by tree. The rules are the ones `git check-ignore` reads (`git2`'s `is_path_ignored`), so a rule kept only inside a directory absent from the tree is not seen; `/.ductus/results/` is therefore listed in the shipped gitignore template, whose managed block this repository's `.gitignore` carries.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **How is the status gate made visible, and which option satisfies "a check that cannot run MUST never be indistinguishable from a check that passed"?** Both families run at `in-progress` as at `done`, and at any other status each records a `status-gated` skip in the excluded-by-construction class (owner, 2026-11-11, option (d)). Keeping the bound stated only in the doc comment and the analyze reference fails that filter. A skip alone satisfies it at every status, but leaves every closing analysis with the two families unmeasured, a recorded caveat where §design-principles requires a measurement, and keeps the flip-to-`done` procedure, which works only when someone remembers it. Running at `in-progress` alone stays silent below it. Measured at 794af825 on 0.59.11, with the 30 in-progress specs flipped to `done` in a scratch worktree: running the families there adds 6 body-path findings in 3 specs, each a path its prose describes as history, the 11 worktree-only criterion findings above, and 429 excluded-by-construction skips. This reverses, below `in-progress`, the `AGENTS.md` rule that a family with nothing to examine records no skip, and the edge cases of `check-artifacts-skipped-targets` and `criterion-path-existence-family` that say the path families record nothing below `done`; the constitution's statement that the criteria check examines `done` specs only, and the analyze reference's flip-to-`done` measurement, change with it. The owner confirmed (2026-11-11) the two details this answer added: a status outside the set records `status-gated` too, and the skip's path is the spec's `spec.md`, one record per family.
- **How does `criterion-path-existence` classify a path that does not resolve because git ignores it?** As a `gitignored` skip, excluded by construction, by the repository's own ignore rules through `git2`, and `/.ductus/results/` joins the shipped gitignore template so the rule exists in every tree (owner, 2026-11-11, option (b) and the template line). Measured: on the 45 `done` specs, 3 findings in 2 specs (035, 062) appeared only in a fresh worktree, all naming `.ductus/session.toml` or `.ductus/sessions/`, and none in this checkout; running the family at `in-progress` adds the 11 above, 14 in 10 specs. Reading the shipped template instead, under the existing `ships-to-adopter` reason, was rejected because it answers only in this repository, while the ignore rules answer in an adopter's tree too. `.ductus/results/` was ignored only by a `.gitignore` inside it, which a fresh tree lacks; the template line is redundant with that file, and every adopter's managed block gains it on its next `/ductus` run.
