---
section: "Follow-on scenarios"
---

# Migration-chain-reference-integrity

## Context

This spec's Motivation names the failure it exists to prevent: *"Nothing prevents
a maintainer from deleting a template, command, or filename convention without
writing an adopter migration. Adopters silently break on the next pipeline
command."* The registry closed that. It did not close the subtler version, where
the migration **does** exist and a *later* migration invalidates what an
*earlier* one wrote.

Each entry is authored against the layout as it stood at its own
`introduced_in`, and is correct there. Nothing validates the **composition** —
and an adopter far enough behind runs several in one batch, so the composition
is what they actually execute.

Two instances surfaced on 2026-08-16, both during the real adopter bootstrap
[048](../../048-govern-acquired-runtime/spec.md) AC10 called for, and neither
visible to any check this repository runs against itself:

- **`constitution-relocate` (0.24.0) step 4** rewrites `CLAUDE.md`, `AGENTS.md`
  and `README.md` to point at `.govern/constitution.md`. **`ductus-rename`
  (0.28.0) step 2** then moves that file to `.ductus/constitution.md` and, as
  authored, said nothing about those three files. Following both literally left
  a dangling `@import`. Found by the maintainer running the batch by hand.
- **`govern-dir-consolidate` (0.22.0)** moved the shipped generators from
  `scripts/` to `.govern/scripts/` and `ductus-rename` moved them again, while
  the adopter-owned `.githooks/pre-commit` that invokes them was re-pointed by
  neither — leaving a hook calling a script whose targets no longer existed.

A third, found the same day and the same way, is the branding half rather than
the path half: `README.md` still reads *"the `govern` framework"* and links
`github.com/stonean/govern`. That link **redirects**, so nothing dangles and no
reachability check would flag it — but `AGENTS.md` records that the redirect
holds only while the retired name stays unused, so an adopter's README is
carrying a reference with an expiry nobody is tracking. It is in scope here
because the form is **shipped**: `framework/templates/project/project-readme.md`
seeds it, so a migration can target the known form the way
`constitution-relocate` already targets README's constitution links, warning
rather than rewriting when the text was hand-altered.

Both share one shape. The referrer is an **adopter-owned** file — strategy
`create`, so the manifest never overwrites it, and unpinned, so the
pinned-invoker warnings never fire. A migration is the only mechanism that can
follow a move into one, and each migration knows only its own move.

The failure is silent in the way this project pays most for: nothing errors. A
dangling `@import` yields a constitution that is simply not loaded, and a hook
calling a missing script fails at commit time, far from the run that broke it.

## Behavior

**A migration that moves a path re-points every adopter-owned referrer of it,
including references a previous migration wrote.** This is stated as a rule the
registry's procedure files are authored against, so a new entry is written with
the composition in view rather than only its own hop.

**The batch verifies itself when it finishes.** After the pending migrations
have run, `/{project}` checks that no adopter-owned file left behind by the
batch references a framework-owned path that does not exist. The check is
**derived** — it reads the files and the filesystem — rather than declared by
the migration author, because a per-entry "referrers" list would be exactly the
input `AGENTS.md`'s second Design Principle rules out: correctness would depend
on remembering to fill it in, and the cases where it is forgotten are the cases
where it mattered.

**Its durable home is `/{project}:analyze`, under §Project-level consistency.**
A check that runs only during `/{project}` runs only while an adopter is
already mid-bootstrap; an orphaned reference discovered a week later has no
surface to report it. That section already exists for exactly this subject —
checks that *"span the project's installed command set and constitution rather
than the target feature"* and *"catch drift in the framework files `ductus`
ships"* — it is read-only and advisory, it ships to adopters in the Shared
Files manifest, and it runs on every invocation regardless of which feature is
targeted. One rule, one primitive, two call sites: `analyze` is where an
adopter meets it, and the batch-end call is where it runs closest to the cause.

**Attribution is available in one call site and not the other, and the result
says which.** `framework/migrations.toml` — the registry carrying each entry's
`target_paths` — is **not** in the Shared Files manifest; it lives in staging
during a `/{project}` run and is absent from an adopter checkout. So the
batch-end call can name the entry whose move most likely orphaned a reference,
while the `analyze` call has only `.ductus/config.toml`'s
`[migrations].last_applied` — a watermark naming the newest applied entry, not
a map from paths to the migrations that moved them. The primitive therefore
reports **which mode it ran in**: attributed against the registry, or
watermark-only. Rendering the two identically would be `QUAL-CLAIM-001` in the
check itself — a finding that reads as an attribution nobody computed.

**A dangling reference is reported, not repaired.** The batch names the file,
the missing path, and the migration whose move most likely orphaned it. It does
not guess a replacement: the adopter may have hand-edited the reference, and a
rewrite that guessed wrong would be worse than a report that is precise. The run
does not halt — the migrations that did apply are correct and re-running is
safe.

**The check runs on every batch, not only on chains.** A single migration can
orphan a reference just as a chain can; scoping this to multi-entry batches
would make it a check that runs least often in the case it was written for.

**Stale grants are the same class and are *not* covered by `/{project}:configure`.**
A migration that renames a tool surface leaves the old permission entries
behind — the adopter still carries `Bash(command -v gvrn)`, `Bash(gvrn --version)`,
`govern-*` temp Read globs and two `cp …govern.md.upstream` grants after the
batch. `ductus-rename` scopes itself to the `mcp__gvrn__` entries, which is
correct for what it claims. But nothing else prunes them: `configure.md` is
explicit that it adds canonical entries and removes only **exact-match
duplicates**, and never rewrites non-duplicate entries another command added —
so these survive every future run. They are dead rather than dangerous, and the
report-not-repair rule above applies: name them, let the operator decide.

## Edge Cases

- **A converged adopter**: the batch applies nothing, so there is nothing to
  verify and the check emits nothing — not a clean bill of health for files it
  never examined.
- **A reference the adopter hand-edited to something valid**: resolves, so it is
  not reported. The check tests reachability, not conformance to a shipped form.
- **A reference into a path that is legitimately absent** — an optional artifact
  a project never scaffolded: reported, because the check cannot tell it from a
  break. The report names the file and lets the operator judge; a suppression
  list would be the declared input this scenario avoids.
- **A pinned referrer**: still reported. Pinning opts out of framework *writes*,
  not out of being told the file is broken.
- **The check cannot read a file it was going to examine**: reported as
  unexamined rather than passed, per `QUAL-CLAIM-001` — the same distinction the
  rest of this framework's checks draw.

## Open Questions

*None — see Resolved Questions.*

## Resolved Questions

- **Where should the check live — the bootstrap procedure or an
  `/{project}:audit` family?** **Neither, as posed: `/{project}:analyze` under
  §Project-level consistency, with the bootstrap as a second call site.**
  Resolved 2026-08-17 by the operator.

  The question offered two options and both were wrong for the same reason.
  `/{project}:audit` is **maintainer-only** — its own frontmatter says so, it is
  absent from the Shared Files manifest, and its body states adopters never
  invoke it — so an audit family could only ever check *this* repository, never
  the adopter whose config the two motivating instances were found in. And the
  bootstrap alone confines the check to the minutes an adopter spends running
  `/{project}`, which is the one window in which they are least likely to be
  looking.

  `/{project}:analyze` is neither of those. It ships to adopters, it already
  carries a §Project-level consistency section scoped to exactly this subject —
  framework-owned paths referenced from files outside the target feature — and
  that section is already read-only and advisory with an established
  advisory→blocking promotion path this check inherits. No new command, no new
  surface.

  The bootstrap call site is kept rather than replaced, because the registry it
  holds is what makes attribution possible at all (see the Behavior section):
  the same primitive runs richer there and degrades explicitly, never silently,
  when `analyze` calls it without a registry to attribute against.
