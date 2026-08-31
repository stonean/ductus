---
status: done
dependencies: [050-constitution, 052-spec-supersession-and-consolidation]
review:
  last-run: 2026-08-31T00:30:00Z
  reviewed-against: 018f8be23ebc2ccd339263d0075a77b7bab534c2
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  blocking: false
next-criterion: 27
---

# 054 — Remove supersession

Remove supersession from the framework: the `/{project}:supersede` command, the `--supersedes` flag on `/{project}:specify`, the `supersedes:` frontmatter key, the annotation conventions in the constitution, the two runtime primitives and the `classifyClaims` extension point behind them, and the `supersession-reciprocity` check family. The two answers that remain are the two that keep the corpus true: **update the earlier spec in place** through the existing `done → in-progress` back-edge, or **`/{project}:consolidate`** it into the spec that covers it.

## Motivation

Supersession answered a real question — how a reader tells a live decision from a countered one — and answered it by **keeping the countered spec on disk**, annotated, as the record of what shipped. That was the wrong trade for this corpus.

An annotated spec is still a spec. It is still indexed, still linked, still harvested into `dependencies:`, and still read by an agent that opens it looking for current behavior. A banner at the top is a single line of prose asking every future reader — human and agent — to notice it and to carry it down through several hundred lines of body that describes something which no longer exists. The framework's own [constitution](../050-constitution/spec.md) states that a spec body is a living document describing current state; supersession created a class of spec that is exempt from that, and every exempt spec is a set of references pointing at behavior nobody can act on. Multiply that across a corpus and the drift the framework exists to prevent is the drift it manufactures.

The two operations that keep the corpus honest already existed before supersession did. **Editing the earlier spec in place** is what the `done → in-progress` back-edge is for, and it leaves one true description where there were two. **Consolidation** removes a spec that was never a separate concern and re-points every inbound pointer first, so nothing is stranded. Supersession was the third option, and it is the only one of the three that ends with a document asserting things that are false.

The cost of carrying it was real and had barely begun to be paid: a command, a flag on a second command, a hand-authored frontmatter key with its own validator, a constitution section defining three annotation granularities and the blockquote rule that keeps them out of the dependency graph, two runtime primitives across five registration sites each, an extension point with a four-outcome classification vocabulary, a check family whose own documentation has to state at length what its silence does *not* mean, and a shared already-present predicate binding the annotation writer to `check-artifacts`.

The machinery shipped in [052 — Spec supersession and consolidation](../052-spec-supersession-and-consolidation/spec.md) — the declaration and its annotation, plus the reconciliation pass that followed — and has not been used. **No spec in the corpus carries a `supersedes:` key.** The one sunset banner in the corpus — on `005-workflows`, written when `043-workflows-sunset` removed the feature — was hand-authored before any of this existed, which is itself the evidence that the prose convention never needed a command, a key, a primitive, or a check to produce it. Removing the machinery now costs no migration and no corpus rewrite; removing it after adoption would cost both.

`/{project}:consolidate` shipped in the same spec and **stays**. It is the operation that removes the dead reference rather than annotating it.

## What replaces it

Nothing new is built. Two existing operations absorb the cases supersession claimed:

| Case | Answer |
| --- | --- |
| A spec shipped, and a later spec countered part of it | Reopen it `done → in-progress` and **edit the countered section** so the body describes what is true now. The lifecycle back-edge already covers this. |
| A spec shipped, and a later spec countered all of it | Reopen and edit it down to what survives, or **consolidate** it into the spec that covers it. |
| A spec never shipped and overlapped a sibling | **Consolidate** — unchanged from today. |

The distinction `/{project}:consolidate` currently draws between "did it ship?" (annotate) and "did it never ship?" (consolidate) collapses: the question a reader now asks is whether the earlier spec still describes something true, not whether it once did. Its guidance needs rewriting to that question rather than to `/{project}:supersede`.

## Removal surface — framework

- **`framework/commands/supersede.md`** — the whole file, and its generated copy under the host command directory. It is also the **canonical home of Declaration semantics**, which `specify.md` and `consolidate.md` defer to rather than restate; those deferrals must not be left pointing at nothing.
- **`framework/commands/specify.md`** — `[--supersedes <feature>]` in the `argument-hint:` frontmatter; the `--supersedes` row in the flag table; the second-spec write in Scope Boundaries; step 5 (settle the declaration) and step 8 (write the key and annotation); the supersession classification offered on a routing candidate in step 3; the body-link warning in step 7; and the **Declare a supersession** section of the markdown-only reference. The two `ductus exec` reduction notes for those steps go with them.
- **`framework/commands/consolidate.md`** — the `/{project}:supersede` row in the decision table; the "if the earlier spec delivered something, the command is `/{project}:supersede`" guidance and its restatement in the "did it ship?" question; step 3 (settle every `supersedes:` edge in both directions) and the **Settling `supersedes:` edges** reference section; the `supersedes:` mentions in steps 1, 4, 5 and 8; and the `ductus exec` note that step 3 does not run.
- **`framework/commands/analyze.md`** — supersession reciprocity in the `check-artifacts` family enumeration (**nine residual families becomes eight**), the **Supersession reciprocity** reference section, and the paragraph bounding what the family's silence means.
- **`framework/commands/help.md`** — the `/{project}:supersede` row.
- **`framework/constitution.md`** — the `§supersession-annotations` section entire (the three granularities, the blockquote/`derive-dependencies` exemption rule, the accumulation rule, the mechanical-edit classification, and the "annotate only what was delivered" closer); the criterion-level supersession bullet in `§spec-requirements`; and the `supersedes` row in the spec frontmatter schema table.
- **`framework/bootstrap/ductus.md`** and **`framework/bootstrap/govern.md`** — the `framework/commands/supersede.md` manifest row, and the hardcoded **"seventeen `framework/commands/*.md` rows"** count in the skills and OpenCode command sections, which becomes sixteen.
- **`framework/bootstrap/configure/claude.md`** and **`framework/bootstrap/configure/auggie.md`** — the `read-supersession-pair` and `write-supersession-annotation` permission entries, regenerated by `scripts/gen-configure-mcp.sh`.
- **`framework/runtime-tools.txt`** — both tool names.
- **`docs/slash-commands.md`** — the `/supersede` section; the two "reach for `/supersede` instead" callouts under `/consolidate`; the `--supersedes` mention in `/specify`'s flag line; and the flag-versus-command paragraph in **Refine**, which uses `/supersede` as its worked example and needs a different one.
- **`README.md`** — the `/supersede` bullet.
- **`scripts/gen-help-tables.sh`** — the `/{project}:supersede` entry.
- **`scripts/audit/readme-command-parity.sh`** and **`scripts/audit/README.md`** — both use `` `/specify --supersedes` `` as the worked example of Family 33's bare-token matching rule. The rule survives; the example names nothing once the flag is gone.
- **`AGENTS.md`** — the five-registration-sites gotcha cites `write-supersession-annotation` as the primitive whose registry entry was missed. The lesson is durable and stays; the example needs a primitive that still exists.

## Removal surface — runtime

- **`runtime/src/primitives/write_supersession_annotation.rs`** and **`runtime/src/primitives/read_supersession_pair.rs`** — both files, and their `pub mod` lines.
- **Five registration sites per primitive**: the CLI command enum and its dispatch arm in `runtime/src/main.rs`, the exec-path match arm in `runtime/src/interpreter/mod.rs`, the `#[tool]` in `runtime/src/mcp/server.rs`, the `PRIMITIVE_REGISTRY` entry in `runtime/src/schema/registry.rs`, and the `framework/runtime-tools.txt` row. `runtime/tests/mcp.rs` asserts the manifest is set-**equal** to the registry, so a half-removal fails loudly.
- **`runtime/src/schema/primitives.rs`** — the args and result types for both primitives, including the `criterion` argument that carries the annotation granularity.
- **`runtime/src/schema/extensions.rs`** and **`runtime/src/interpreter/payload.rs`** — the `classifyClaims` extension point: `SupersededClaim`, `ClassifyClaimsRequest`, its response type, and `build_classify_claims_request`.
- **`runtime/src/primitives/validate_frontmatter.rs`** — `validate_supersedes` and its call site, plus its tests.
- **`runtime/src/primitives/check_artifacts.rs`** — `check_supersession_reciprocity`, its call site, and the `supersession-reciprocity` family tests. The **shared blockquoted-citation predicate** `blockquote_cites` (`runtime/src/primitives/mod.rs`) was extracted so the annotation writer and this family could share it; those are its only two callers, so it goes with them.
- **`runtime/src/primitives/read_spec.rs`** — the two helpers documented as shared with `read-supersession-pair` return to single-caller status; the comments naming that sharing are stale once the second caller is gone.
- **`runtime/tests/two_spec_commands.rs`** — the step-ordering assertions for `/supersede`'s procedure.
- **Explicitly not touched:** the blockquote exclusion in `runtime/src/primitives/spec_links.rs`. It belongs to the shared body scanner behind `derive-dependencies`, `derive-references`, `check-corpus-links`, and `rewrite-spec-links`, and its stated reason — signpost links on `done` specs are navigation, not dependencies — predates supersession and outlives it.
- **`runtime/CHANGELOG.md`** — a **breaking** entry: two MCP tools and two CLI subcommands are removed, and a frontmatter key stops being validated.

## Dead-reference sweep

[§drift-prevention](../050-constitution/spec.md) requires that removing a name — a command, a capability, an identifier, **even a parenthetical descriptor** — updates every reference across live artifacts *in the same change*: specs including `done` spec bodies, rules, command sources, scripts the pipeline runs, CI configuration, docs, and the README. This spec removes six names at once — `/{project}:supersede`, `--supersedes`, the `supersedes:` key, `§supersession-annotations`, and the two primitive names — so the sweep is part of the work rather than a follow-up.

Three properties of the sweep are load-bearing, and all three come from that rule:

- **It is a mechanical edit, so `done` specs stay `done`.** The one `done` artifact outside 052 and 053 that carries a real reference is `specs/026-framework-self-audit/scenarios/link-check-consolidation.md`, which uses `` `/specify --supersedes` `` to illustrate Family 33's matching rule. Substituting a surviving flag uniformly across that scenario and the two audit artifacts that share the example keeps the edit mechanical; 026 takes no back-edge.
- **It must not be bundled with unrelated edits.** A non-uniform diff is a meaningful edit and reopens what it touches, so the substitution lands on its own rather than inside a broader rewrite of the files it happens to touch.
- **Ordinary English is not a reference.** "The primitives would have superseded" in the pre-commit hooks, `framework/rules/security-backend.md`'s note that CSP `frame-ancestors` supersedes `X-Frame-Options`, and the `review.md` line about a re-run superseding the prior report all use the verb and name no capability. They are left alone; a sweep that rewrote them would be the non-uniform diff the rule warns about.

`check-corpus-links` and `check-orphaned-references` are what prove the sweep landed — in particular the `#supersession-annotations` anchor, cited from `§spec-requirements` and from `runtime/src/schema/primitives.rs`, and the README's deep link into `docs/slash-commands.md#supersede--the-supersedes-key-on-one-spec-the-annotation-on-the-other`.

## Adopter cleanup

**No migration is added.** The only artifact this removal leaves in an adopter's tree is the installed `supersede` command file, and it sits exactly where the framework's existing slash-command cleanup already reaches. Dropping the manifest row is the whole adopter-side change: `enforce-manifest` prunes anything in the per-layout cleanup glob that is neither in the manifest nor pinned, so the stale command is removed on the adopter's next `/ductus` run.

That covers all three layouts, because the cleanup glob is defined per layout in the Agent Registry:

| Layout | Cleanup glob | Installed path |
| --- | --- | --- |
| `claude-style` | `*.md` in the commands dir | `{config_dir}/commands/{project}/supersede.md` |
| `antigravity` | `{project}-*/` skill dirs in `skills/` | `{config_dir}/skills/{project}-supersede/SKILL.md` |
| `opencode` | `*.md` in `command/{project}/` | `{config_dir}/command/{project}/supersede.md` |

It also already has the three properties a migration would have been written to provide: a pinned path is kept rather than deleted, a run with nothing stale present removes nothing and says nothing, and an adopter still on `govern`-era paths resolves through the same per-layout rules every other command does.

**Why the two existing migrations are not a precedent here.** `workflows-sunset` and `generator-primitives` both clean artifacts that sit *outside* the enforced set — a `workflows/` subdirectory of the commands dir plus a root `registry.json` and a `[workflows]` config section for the first, `.ductus/scripts/` for the second. Neither is reachable by a cleanup glob over top-level command files, which is why each needed a procedure of its own. Supersession installs no such artifact. Adding a registry entry here would duplicate machinery that already runs, and would hand `/{project}:audit` Family 10 a `target_paths` list to police for a removal nothing else has to do.

Two adopter-side non-events, stated so their absence is deliberate rather than overlooked: a `supersedes:` key already written into an adopter's spec frontmatter is left as inert YAML that no validator reads, and a sunset banner an adopter wrote is body prose that nothing mechanical touches.

## Acceptance Criteria

- [x] AC1: `framework/commands/supersede.md` does not exist, and neither does any generated copy of it under a host command or skill directory
- [x] AC2: No command source defers to `supersede.md` for Declaration semantics or any other statement, and none names `/{project}:supersede`, `--supersedes`, or the `supersedes:` key — the bare verb in `review.md`'s report-regeneration line is ordinary English and stays, per AC21
- [x] AC3: `/{project}:specify` accepts no `--supersedes` flag, dispatches no second-spec write, and offers no supersession answer on a routing candidate — a candidate admits *amend it* and *unrelated* only
- [x] AC4: `/{project}:consolidate` names no supersession alternative; its ship/no-ship decision table is rewritten to ask whether the earlier spec still describes something true
- [x] AC5: The constitution carries no `§supersession-annotations` section, no criterion-level supersession bullet in `§spec-requirements`, and no `supersedes` row in the spec frontmatter schema table
- [x] AC6: No anchor, link, or cross-reference anywhere in the repository resolves to a removed constitution section or to the removed command file — `check-corpus-links` and `check-orphaned-references` report clean
- [x] AC7: `write-supersession-annotation` and `read-supersession-pair` are absent from all five registration sites, and `cargo test --test mcp` passes with the manifest set-equal to the registry
- [x] AC8: The `classifyClaims` extension point and its request, response, and claim types are gone, and no extension point remains without a caller
- [x] AC9: `validate-frontmatter` contains no `supersedes` validation path
- [x] AC10: `check-artifacts` runs eight residual deterministic families, `supersession-reciprocity` is returned by no code path, and `analyze.md`'s enumeration and count match the implementation
- [x] AC11: `cargo build`, `cargo clippy`, and the full `cargo test` suite pass with no dead-code or unused-import warnings from the removal
- [x] AC12: `help.md`, `docs/slash-commands.md`, and `README.md` document sixteen commands with no `/supersede` among them, and `scripts/gen-help-tables.sh` carries no supersede entry
- [x] AC13: The installer manifest carries no `supersede.md` row, and every hardcoded command count in `framework/bootstrap/ductus.md` and `framework/bootstrap/govern.md` reads sixteen rather than seventeen
- [x] AC14: `framework/bootstrap/configure/claude.md` and `framework/bootstrap/configure/auggie.md` carry no supersession tool permissions, and `scripts/gen-configure-mcp.sh` reproduces them byte-for-byte
- [x] AC15: `/{project}:audit` reports zero findings, including Family 16 (installer-command-parity) and Family 33 (readme-command-parity)
- [x] AC16: No entry is added to `framework/migrations.toml` and no procedure file to `framework/migrations/`; an adopter carrying an installed supersede command has it removed by slash-command cleanup on the next `/{project}` run, in each of the three agent layouts, with a pinned path kept and no adopter spec content edited
- [x] AC17: `runtime/CHANGELOG.md` records the removal as breaking, naming both MCP tools, both CLI subcommands, and the frontmatter key that stops being validated
- [x] AC18: Spec 053's directory no longer exists, its content is consolidated into 052, and every inbound pointer to 053 — including this spec's own body link — resolves to 052
- [x] AC19: No live artifact references `/{project}:supersede`, `--supersedes`, the `supersedes:` key, `§supersession-annotations`, `write-supersession-annotation`, or `read-supersession-pair` — command sources, constitution, bootstrap, docs, README, `AGENTS.md`, `scripts/`, and every spec body included
- [x] AC20: The `` `/specify --supersedes` `` worked example is replaced by a surviving flag in `scripts/audit/readme-command-parity.sh`, `scripts/audit/README.md`, and `specs/026-framework-self-audit/scenarios/link-check-consolidation.md`, uniformly and in one edit, leaving 026 at `done`
- [x] AC21: Ordinary-English uses of "supersede" that name no capability — the pre-commit hooks, `framework/rules/security-backend.md`, `review.md`'s report-regeneration line — are unchanged
- [x] AC22: Spec 052 is `done`, carries no supersession section and no acceptance criterion whose deliverable was removed, retains its consolidation content and its existing directory name, and its H1 names consolidation alone
- [x] AC23: `blockquote_cites` is absent from the runtime and referenced by no caller
- [x] AC24: The blockquote exclusion in `runtime/src/primitives/spec_links.rs` is unchanged, and `derive-dependencies` produces byte-identical `dependencies:` frontmatter across the corpus before and after this change
- [x] AC25: `validate-frontmatter` reports `clean` on a spec whose frontmatter carries a `supersedes:` key, emitting no finding at any severity
- [x] AC26: `docs/slash-commands.md`'s flag-versus-command paragraph states the two-spec rule with no exception, naming `/fold` and `/consolidate` as the two-spec commands

## Open Questions

*None — all resolved.*

## Resolved Questions

- **How are 052 and 053 dispositioned?** → **053 is consolidated into 052; 052 is reopened, edited down, and returned to `done`.** 053 shipped reconciliation alone and nothing in it survives, so it goes through `/{project}:consolidate` into its parent — the directory is removed and every inbound pointer re-pointed, which also re-points this spec's own body link from 053 to 052 and so avoids the self-link that consolidating into 054 would have created. 052 shipped supersession **and** consolidation; it reopens `done → in-progress`, loses its supersession sections and the acceptance criteria whose deliverables no longer exist, keeps consolidation, and returns to `done` when this spec lands. Deleting those criteria is safe because `next-criterion` is monotonically non-decreasing and never reissues a retired label — that existing mechanism is what replaces the criterion-level annotation rule being removed. **052 keeps its directory name**: directory names here are stable numbered identifiers that inbound links and git history resolve through, and renaming would re-point every pointer for cosmetic gain; only the H1 is retitled.
- **Does the hand-authored sunset banner survive as a prose convention?** → **No.** `§supersession-annotations` is removed in full — the three granularities, the blockquote citation rule, the accumulation rule, and the mechanical-edit classification — and nothing replaces it. A historical-record convention earns its cost on a corpus with readers outside the project; this one does not have them yet, so the answer to "this spec's behavior is gone" is to edit the spec or consolidate it, with no third state. The existing banner on `005-workflows` is left in place as inert prose — it predates this machinery, it is `043-workflows-sunset`'s leftover rather than this spec's, and no follow-up item is opened for it.
- **What happens to an adopter's existing `supersedes:` key?** → **It is silently ignored, and no migration step touches it.** `validate-frontmatter` performs no unknown-key rejection — it checks a known set (`status`, `dependencies`, `folds-into`, `supersedes`, `review`) and ignores everything else (`runtime/src/primitives/validate_frontmatter.rs:70-135`) — so deleting `validate_supersedes` and its call site leaves the key inert YAML, exactly as any unrecognized key already is. Both alternatives mean building something inside a removal spec: stripping it would be the first migration that edits an adopter's spec content, and reporting it would require adding unknown-key checking that does not exist. Exposure is near-zero regardless — the key shipped in runtime 0.41.0 on 2026-08-30 and no spec in this corpus carries one.
- **What happens to an adopter's existing sunset banner?** → **Nothing touches it**, and that is intended rather than an omission. It is body prose, outside every mechanical surface, and the resolution above removes the convention without removing what was already written under it.
- **What replaces `/supersede` as the worked example in `docs/slash-commands.md`'s flag-versus-command paragraph?** → **Nothing — the paragraph loses its exception and states the rule plainly.** `--supersedes` was the *counter*-case: a two-spec operation permitted as a flag because `/{project}:specify` was writing one of the two specs anyway. No other capability is that case (`--fold-into` writes only the new spec's own frontmatter), so with supersession gone the rule needs no exception. The paragraph names `/{project}:fold` and `/{project}:consolidate` as the two-spec commands, states that neither fits inside a single-spec command as a flag, and keeps `/{project}:fold` gaining no `--into` as its whole example.
- **Is the shared blockquoted-citation predicate removed or retained?** → **Removed.** `blockquote_cites` (`runtime/src/primitives/mod.rs:1107`) has exactly two callers — `write_supersession_annotation.rs:123` and `check_artifacts.rs:1532` — and both are removed by this spec. No third caller exists.
- **Does the blockquote exemption stay?** → **Yes, untouched — and it is not `derive-dependencies`' to begin with.** The exclusion lives in the shared body scanner `runtime/src/primitives/spec_links.rs:126`, used by `derive-dependencies`, `derive-references`, `check-corpus-links`, and `rewrite-spec-links` alike, and its own comment gives a reason that predates supersession: signposts on `done` specs use blockquotes, and their forward-pointer links are navigation rather than dependencies. It also has a live dependent — `005-workflows`' banner carries blockquoted links to `043` and `019` that would become dependency edges if the exclusion were dropped. No code changes; the only prose being removed is the sentence tying the exclusion to annotations, which sits inside `§supersession-annotations` already.

## See also

- [043 — Workflows sunset](../043-workflows-sunset/spec.md) — the precedent for removing a shipped capability: framework surface, adopter migration, and a hand-authored sunset banner on the spec that shipped it
- [005 — Workflows](../005-workflows/spec.md) — carries the corpus's only sunset banner, written before any supersession machinery existed
