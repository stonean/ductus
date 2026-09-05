---
section: "Follow-on scenarios"
---

# Apply-manifest — the substitution-key contract, and a count that was computed and thrown away

## Context

`apply-manifest` builds the token it searches for by wrapping each substitution key in braces — `format!("{{{key}}}")`. The key is therefore the *inside* of the placeholder: `project`, not `{project}`.

Nothing said so. The [apply-manifest](./apply-manifest.md) scenario that introduced the primitive describes `substitutions: BTreeMap<String, String>` without stating which form the keys take, the schema's own doc comment called it a "`{key}` → value substitution map" — which reads as *keys are braced* — and the installer's §Placeholder Substitution writes every placeholder braced while `keep-literals: ["project", "cli-config-dir"]`, two sections earlier, writes the same names bare. Both spellings are correct for what they are naming: one names tokens *in files*, the other names *map keys*. Read together, with no contract stated, they are a coin flip.

An adopter called it with braced keys. Every key built `{{project}}`, which matches nothing. Every entry still ran its strategy, every file was still written, and the call returned:

```json
{"created":1,"updated":25,"unchanged":11}
```

That is what a correct run returns. There was no error, no warning, and no field whose value differed between the run that worked and the run that did not. What actually shipped was 370 literal placeholders across the constitution, all 17 commands, 5 rule files and 2 templates — every `/{project}:review` reaching the adopter as `/{project}:review`.

**The result had the diagnostic and discarded it.** `read_and_substitute` called `apply_substitutions`, which returns `(String, u32)`, and bound the count to `_count`. One number — zero replacements across thirty-seven files — separates the two runs, and it was computed on every entry and dropped on the floor. That is `QUAL-CLAIM-001` exactly: a result reporting a clean state that cannot distinguish *examined and found nothing* from *never examined at all*. The rule is one this project ships to adopters in `framework/rules/quality-cross.md`, whose own `**Source:**` field cites four instances in ductus's own tooling. This is the fifth, in the primitive that installs the file.

## Behavior

**A malformed key is rejected, not tolerated.** A substitution key that is empty or contains `{` or `}` is a [`PrimitiveError::InvalidSubstitutionKey`]. The check runs before the traversal validation and before any filesystem operation, so a bad map halts the walk with **zero writes** rather than leaving a half-substituted tree for an operator to reconcile.

The rejection is exact rather than stylistic, and that is what lets it be a hard error instead of a warning. A key containing a brace can never match a well-formed placeholder — the primitive would have to produce `{{project}}`, which is not a token any template carries. It rejects only keys *incapable* of matching, never keys that merely look unusual: `One-line project description.` carries spaces and a period, is a real installer placeholder, and passes.

**The count is surfaced.** Three fields, and the shape of all three is chosen so that no reading of them is silently wrong:

- `substitutions-applied` on each entry — `Option<u32>`. `Some(0)` means the file was read, decoded as UTF-8, and matched no placeholder, which is correct for a file that carries none. `None` means the question was never asked: the entry was pinned, skipped-exists, source-missing, used `skip-if-conflict` (which never substitutes), or its bytes are not UTF-8. Serialized with `skip_serializing_if`, so `None` is *absent* rather than `0`.
- `substitutions-applied` on the result — the total across every entry that ran.
- `entries-substituted` on the result — the denominator without which the total cannot be read. Zero replacements across twenty substituted entries is a defect; zero across zero is a manifest of pinned and skipped files behaving correctly. One number cannot say which, which is the same mistake in miniature as the one that hid the original bug.

An invalid key is now impossible, but an *incomplete* map — a key the caller simply forgot — still is not, and no validation can catch it. That is precisely why the count ships alongside the rejection rather than instead of it: the rejection closes the class that can be closed, and the count makes the class that cannot be closed visible in one glance.

## Edge Cases

- **A binary source.** Passed through unchanged, as before, and reports `None`. It was never examined for placeholders, so it has no count rather than a count of zero.
- **`keep-literals` masking every key for an entry.** The entry still ran substitution against an empty effective map, so `Some(0)` is correct and honest: it was examined and, by instruction, nothing applied.
- **The `ductus` self-install.** Passes an empty substitutions map by design, so validation has nothing to reject and every entry reports `Some(0)`. The installer's step 9 already explains why the map is empty rather than masked; that reasoning is unchanged.
- **A key with regex or format metacharacters.** Not special. Substitution is plain string replacement over a `BTreeMap` in lexicographic order, non-recursive, and none of that changes.

## Resolved Questions

- **Why reject rather than warn, or normalize by stripping the braces?** Normalizing would guess. `{project}` almost certainly means the key `project`, but a caller who wrote it believed something false about the contract, and silently repairing the call leaves that belief in place to be re-expressed somewhere no repair exists. A warning fares worse: this failure already produced a clean-looking result, and the operator who missed 370 literal placeholders across 37 files is not the operator who will catch a warning line in a long bootstrap transcript. §design-principles rejects designs that depend on human diligence, and a warning here is exactly one.
- **Why not accept both forms, since the intent is unambiguous?** Because then both forms are the contract, `keep-literals` has to accept both too, and every future caller has to be told the two are equivalent — which is more surface than the one sentence that was missing. One form, stated where callers read, and enforced where they call.
- **Is adding fields to the result a breaking change for hosts?** No. Both aggregate fields are additive, and the per-entry field is `Option` with `skip_serializing_if`, so an existing consumer sees the same JSON it saw before for every entry that did not substitute and one extra key for those that did. The installer's §Post-Scaffolding Output now surfaces both numbers, since a diagnostic nobody prints is one nobody reads.
