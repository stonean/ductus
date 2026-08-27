---
section: "Flags"
---

# Review-flag-parsing-is-specified

## Context

An adopter reported that `--since` "doesn't show as an option". It is
documented in the Flags table of both this spec and
`framework/commands/review.md`, and `compute-review-scope` accepts a `since`
argument — so the plumbing exists at both ends. What is missing is everything
between them.

Two concrete gaps, both checkable:

**The `argument-hint` is incomplete.** `framework/commands/review.md` declares
`argument-hint: "[--all] [--fix] [feature]"` while its Flags table documents
eight entries. `--since`, `--waive`, `--security`, `--simplicity`, and
`--quality` appear in no hint. `argument-hint` is the surface a host renders
when offering the command, so a flag absent from it is a flag the operator is
never shown — which is precisely the reported symptom.

**There is no `$ARGUMENTS` parse step.** `review.md` never tells the model what
to do with its arguments. Sibling commands do: `/ductus:analyze` opens with
"Parse `$ARGUMENTS` for flags and an optional feature identifier", and
`/ductus:implement` specifies `--auto` handling down to its position-
independence and its non-persistence to the session file. `review.md` mentions
`--since` only in passing inside step 1's prose, as "or a `--since` override",
which leaves whether the flag is honoured to the model's discretion on the day.

The adopter's own framing is the right one: this is the whole table, not
`--since`. A flag that is documented, plumbed, and unparsed is worse than an
undocumented one, because the table asserts a capability the command does not
reliably have — the same claim-exceeds-behavior shape `QUAL-CLAIM-001` names,
expressed in a command body rather than a result payload.

`/ductus:analyze`'s command-frontmatter check catches the converse case — a
body that documents `$ARGUMENTS` without an `argument-hint` — so a command with
a hint but no parse step, and a hint that disagrees with its own Flags table,
both pass it today.

## Behavior

`framework/commands/review.md` gains an explicit `$ARGUMENTS` parse step, in
the shape `/ductus:analyze` and `/ductus:implement` already establish: the
arguments are parsed for flags in any position, the residue is treated as the
optional feature override, and each flag's effect is stated where it is parsed
rather than inferred from prose elsewhere in the body.

The parse step covers every flag the Flags table documents — the dimension
selectors, `--all`, `--fix`, `--since=<ref>`, and the `--waive` /
`--reason` pair — so table and behavior describe the same command.

`argument-hint` is brought into agreement with the Flags table, so every
documented flag is surfaced.

The two are kept in agreement by a check rather than by author diligence.
Without one, this scenario fixes one command on one day and the next flag added
to the table reopens the gap — the author-diligence dependency the framework
forbids.

The check is the `check-command-flags` runtime primitive, surfaced as `/audit`
Family 30 (`scripts/audit/command-flag-hint-parity.sh`). Its result shape is
recorded in [022's `data-model.md`](../../022-deterministic-runtime/data-model.md),
the canonical registry of primitive result shapes.

**Not `/ductus:analyze`, and the reason is the fix's location.** That command's
command-frontmatter family looks like the obvious home and is the wrong one
twice over. It reads the *installed copies* under a host's commands directory,
which are generated — an adopter told their `review.md` hint disagrees with its
own Flags table cannot act on it, because the repair is a ductus release. And
that family is scoped frontmatter-only in three places, deliberately, to stay
bounded on every adopter run; a Flags table lives in the body. The divergence
originates in `framework/commands/` in this repo, so it is caught there, before
it ships.

**The parsing is a primitive, not shell.** [§runtime-boundary](../../../framework/constitution.md#runtime-boundary)
principle 3 names `awk`/`sed` pipelines over frontmatter and markdown structure
as not a sanctioned substitute for the runtime, and the runtime already owns
tested frontmatter-fence and fenced-block scanners that a shell version would
re-implement worse — this suite has been bitten by exactly that, with the
portability trap that left Family 7 dead on macOS. The `/audit` contract in
`scripts/audit/README.md` is nonetheless a shell one (source `lib.sh`, render
through `emit`, stay directly invocable), so the family script is a thin entry
point over the primitive: entry point in shell, logic in the runtime.

An unrecognized flag is reported to the operator rather than silently dropped
or absorbed into the feature override, since silently treating `--sinse=HEAD~5`
as a feature name is how a typo becomes a review of the wrong scope.

## Edge Cases

- **`--since` with no value** (`--since` rather than `--since=<ref>`) is an
  operator error, reported as one; it does not fall through to the default diff
  base, because a silent fallback here reviews a different window than the one
  asked for.
- **`--since` composed with `--all`** applies the same override to every
  targeted feature. That is the literal reading, and it is what an operator
  auditing a release window wants.
- **`--since` naming an unreachable ref** is `compute-review-scope`'s domain
  outcome to surface, not the parse step's to validate — the parser's job ends
  at extracting the value.
- **Flag order and position do not matter,** matching `/ductus:implement`'s
  `--auto` rule, so `--fix --all foo` and `foo --all --fix` are the same
  invocation.
- **No flag is persisted to the session file.** All of these are
  per-invocation, the same decision `/ductus:implement` recorded for `--auto`.
- **The other pipeline commands are assessed, not assumed.** `review.md` is the
  command the adopter hit, but the hint-versus-table divergence is a corpus
  question; each command with a Flags table is checked against the same
  question and corrected only where the answer is yes — the uniform edit
  017's `generator-sync-claim-honesty` warns against is not applied here
  either.

  **Outcome of that assessment (2026-08-27).** `review.md` is the only command
  in the corpus with a Flags *table*, and it was the only one divergent — five
  flags absent from its hint, six findings once `--waive`'s paired `--reason`
  is counted. `implement.md` is the one other command with a `Flags` heading;
  it documents `--auto` in prose with no table and its hint names it, so it is
  correct and is not a subject of a table-shaped check. Every other command
  file declares an `argument-hint` with no flags table behind it. So the
  corpus needed exactly one correction, and the check that now guards it
  reports 15 examined with 1 carrying a table — the count that keeps a clean
  exit from reading as a broader claim than it is.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
