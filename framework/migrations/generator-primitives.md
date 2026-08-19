# Migration — generator scripts become runtime primitives

The two frontmatter derivations shipped as bash scripts scaffolded into every
adopter's `.ductus/scripts/`, invoked by the pre-commit hook on every commit:
`gen-spec-deps.sh` derived `dependencies:` from body links to sibling specs,
and `gen-cross-service-refs.sh` derived `references:` from cross-service URLs.
Both parsed YAML frontmatter and markdown structure in `awk`.

They are now the `derive-dependencies` and `derive-references` runtime
primitives (spec 022, `adopter-generator-promotion`). The scripts are removed,
along with the `lib/specs-root.sh` helper they shared — the runtime resolves
`[paths] specs-root` itself.

Nothing about the derivation changed. The primitives were held byte-identical
to the scripts across a corpus of fixtures covering every exclusion rule, the
frontmatter splice, cycle detection, and both root-matching tiers before the
scripts were retired; those fixtures survive as the runtime's golden tests.

## Why the scripts could not stay

[§runtime-boundary](../constitution.md#runtime-boundary) principle 3 names
shell pipelines that parse frontmatter or markdown structure as **not** a
sanctioned substitute for the runtime primitives. The scripts predated the
runtime being required — a generator that fires on every commit could not
depend on a binary that might be absent. Spec 048 made the runtime required
and acquired, which retired that constraint.

## Steps

1. **Remove the retired scripts.** For each of `.ductus/scripts/gen-spec-deps.sh`,
   `.ductus/scripts/gen-cross-service-refs.sh`, and
   `.ductus/scripts/lib/specs-root.sh` that exists, delete it. Remove
   `.ductus/scripts/lib/` if it is left empty. Leave any other file under
   `.ductus/scripts/` alone — it is the adopter's own.

2. **Pinned scripts are still removed, and the pin is cleared.** Pinning opts
   out of *updates*, not out of a removal: a pinned copy of a retired generator
   is a script whose invoker no longer calls it, and leaving it behind is how an
   adopter ends up debugging dead code. Delete it, and drop its entry from
   `.ductus/config.toml` `[pinned] files` so the pin does not outlive its
   target. Report each one: `removed pinned {path} — the generator is now the
   {primitive} primitive; the pin has been cleared.`

3. **Pinned-invoker warning.** The hook that invoked the generators is normally
   rewritten by the scaffolding pass. An adopter who has **pinned**
   `.githooks/ductus-pre-commit` keeps a copy that calls the now-deleted
   scripts, so every commit would fail on a missing file. This migration does
   not rewrite pinned files; instead, when `.githooks/ductus-pre-commit` is
   listed in `[pinned] files` and still references `.ductus/scripts/gen-`, emit:
   `warning: pinned .githooks/ductus-pre-commit still calls the removed
   generators; replace those lines with 'ductus derive-dependencies --write
   --staged' and 'ductus derive-references --write --staged'.`

4. **CI.** An adopter using the shipped `adopter-generators.yml` template gets
   the updated copy from the scaffolding pass. One that has customized it may
   still invoke `.ductus/scripts/gen-spec-deps.sh`; the orphaned-reference check
   reports it on the next `/ductus:analyze`.

## What the adopter sees afterwards

The pre-commit hook derives both indexes through the runtime. A commit made
with the runtime unreachable now **halts** rather than silently skipping the
derivation — the indexes are captured by the commit, so a skip would land stale
values. `git commit --no-verify` remains the deliberate bypass.
