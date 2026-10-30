---
status: in-progress
dependencies: []
next-criterion: 12
---

# 056 — Bootstrap archive-boundary split

Split `framework/bootstrap/ductus.md` so the file an adopter curls, installs into every agent, loads into context at every invocation, and byte-compares on every run carries only what the run needs **before the framework archive exists**. The remainder ships in the archive, which the run fetches anyway, and is read from the extracted tree.

## Motivation

`framework/bootstrap/ductus.md` was a single file carrying the whole adoption procedure. Four costs compounded, and all four were paid per invocation rather than once:

- The file was **curled** by `install.sh` on every install and refresh.
- It was **installed into every selected agent** — verbatim for `claude-style` and `opencode`, body-wrapped as a skill for `antigravity` — so an adopter on three agents held three copies.
- It was **loaded into the agent's context at every `/ductus` invocation**, whole, before the run knew which of its sections it would need.
- It was **byte-compared against upstream on every run** by the self-update check, and rewritten whenever it differed.

The file's own `§Pre-flight Phase` existed to keep the restart-requiring checks cheap: it states that neither check "pays the cost of the multi-hundred-KB archive; both run on a small fetch or no fetch". That property held for the *fetches* the phase performs and had eroded for the *file* that performs them.

The **cause was structural rather than editorial**. Composition re-measured 2026-09-15 over the 146,953-byte pre-split file: **90.4% prose, 6.9% tables, 2.7% fenced blocks** (the backlog item's 90.2 / 7.0 / 2.8, which held to within 0.2 points). So the reductions a cleanup pass reaches — dead shell fallback, inline permission blobs, retired-release tokens — recover low single digits. Size was not what a dead-code pass would have bought.

## The boundary

### It is execution order, not reading order

The boundary is **derived, not chosen**: the installed half is exactly what the run needs before an archive exists, and the archive half is exactly what cannot run until one does.

The authority for that order is `§Instructions` — the numbered walker — and **not** the order sections appear in the file. `§Instructions` **step 1** assembles the walker context, and that context already carries both the project inputs and `manifest-entries`, "the per-strategy list described in **Shared Files** and **Per-Agent Scaffolding**". Step 2 fetches the archive and step 3 extracts it. So three sections that *read* as though they came after the boundary are in fact step-1 inputs:

- `§Collect Project Inputs`
- `§Shared Files`
- `§Per-Agent Scaffolding`

Reading the file top-to-bottom puts these after `§Pre-flight abort` and suggests they move. They do not. This is the single most important fact about the split, and it is the one a section-order reading gets wrong.

### What stays, and why

| Region | Why it stays |
| --- | --- |
| Frontmatter through `§Pre-flight Phase` | Runs before any archive exists — the whole point of the phase |
| `§Collect Project Inputs` | `§Instructions` step 1 resolves the inputs before the step-2 fetch |
| `§Shared Files`, `§Per-Agent Scaffolding` | `§Instructions` step 1 builds `manifest-entries` from these tables |
| `§Project Configuration` | `§Pre-flight Checks`, `§ductus runtime detection`, `§Instructions` step 1 and `§Collect Project Inputs` each read a key from it |
| `§File Fetching` | It is the fetch-and-extract specification; the markdown-only path cannot fetch the archive from inside the archive |
| `§Placeholder Substitution` | The substitution map is step-1 context, and the self-install keep-literals rule is read on the stale-write path |
| `§Post-Write Integrity Check` | Executable forward-dependency from `§Stale → defer to pre-flight abort`, step 2 |
| `§Re-Run Behavior` | `§Agent Selection` points at it for the agent-removal boundary, before any fetch |

The keep-every-placeholder-literal rule is the other executable forward-dependency (cited from the same stale-write path, step 1). It already lives inside §`ductus` self-installation, which is inside `§Per-Agent Scaffolding`, so it stays without being moved.

### Measured

Measured 2026-09-15 over `framework/bootstrap/ductus.md` at 146,953 bytes / 1186 lines, read in full. Every level-2 section past `§Pre-flight abort` was classified by the rule above — *does a step that runs before extraction read it?* — with a coverage assertion that every post-boundary section lands in exactly one column:

| Installed half | Bytes | Archive half | Bytes |
| --- | --- | --- | --- |
| Frontmatter → `§Pre-flight Phase` | 61,288 | `§Pre-run Migrations` | 6,067 |
| `§Collect Project Inputs` | 5,396 | `§Frontmatter Migration` | 4,769 |
| `§Project Configuration` | 12,627 | `§Security Audit (brownfield)` | 5,332 |
| `§File Fetching` | 5,165 | `§Hook Installation` | 8,061 |
| `§Shared Files` | 9,126 | `§What This Command Does NOT Do` | 489 |
| `§Per-Agent Scaffolding` | 10,878 | `§Edge Cases` | 4,446 |
| `§Placeholder Substitution` | 2,078 | `§Post-Scaffolding Output` | 8,087 |
| `§Post-Write Integrity Check` | 1,097 | `§Idempotency` | 484 |
| `§Re-Run Behavior` | 682 | `§Directory Creation` | 881 |
| **Total** | **108,337** (73.7%) | **Total** | **38,616** (26.3%) |

So what an adopter curls, installs, loads into context, and byte-compares drops by **26.3% — 38,616 bytes per invocation**.

Exactly **two** executable forward-dependencies cross the original boundary, both on the pre-flight stale-write path, and both resolve inside the installed half under this assignment. Every other forward reference is **narrative** — an ordering note, a deferred-output pointer, a "described in `§X`" — measured at 38 occurrences across 13 distinct targets, of which `§Closing restart` alone accounts for 11. None is a step the installed half executes.

### Why the cut is 26%, not the ~40% this was scoped at

`specs/inbox.md` estimated ~40%. Walking the boundary section by section against what each step actually reads gives 26.3%. The whole difference is two sections that read as post-boundary and are not:

- **`§Project Configuration` (12,627 B)** — `§Pre-flight Checks` reads `[paths] specs-root` from it, `§ductus runtime detection` reads `[runtime] path`, `§Instructions` step 1 reads `[pinned] files`, and `§Collect Project Inputs` cites it for the active-file write policy. All four run before extraction. Splitting the schema across two files would put a canonical record in two places, which [§drift-prevention](../../framework/constitution.md#drift-prevention) forbids outright.
- **`§File Fetching` (5,165 B)** — it *is* the fetch-and-extract specification. On the markdown-only path the host cannot fetch the archive using instructions that live inside the archive.

A first derivation here cut at `§Collect Project Inputs` on section order alone, produced 61,288 / 85,665, and concluded the recorded figures had been **transposed**. They had not. The recorded figures encoded the step-1 constraint that reading had not yet found, and re-deriving a coarse manifest-inclusive cut at the item's own measurement commit `5a519cbb` reproduces them within ~3KB per side. Two lessons, and the second is the durable one: a measurement that disagrees with a recorded one is a reason to **look for the constraint the record encodes** before concluding the record is wrong; and an estimate taken from a coarse cut is not wrong so much as **unwalked** — the rigorous boundary is smaller, and only a section-by-section pass finds that.

### What the split does not reach

No audit family's extraction target moves, and no `runtime/` source changes. Each was checked against the assignment above rather than assumed:

| Consumer | Subject | Half |
| --- | --- | --- |
| `check_artifacts.rs` `adopter_destinations` | `§Shared Files` tables | installed |
| `check_step_references.rs` `BOOTSTRAP_FILES` | `§Instructions` numbered steps | installed |
| Family 16 `installer-command-parity.sh` | `§Per-Agent Scaffolding` slash-command table | installed |
| Family 23 `sweep-target-manifest-parity.sh` | `§Shared Files` manifest | installed |
| Family 35 `manifest-destination-links.sh` | both manifest tables | installed |
| Family 36 `self-url-resolution.sh` | the `archive/` URL in `§File Fetching` (since 061, the `codeload.github.com/<owner>/<repo>/tar.gz/` URL there) | installed |
| `installer-registry-parity.sh` | `§Agent Registry` | installed |
| `host-namespace-parity.sh` | `§Derived values` | installed |
| `runtime-probe-parity.sh` | the store probe | installed |
| Family 21 `transitional-bootstrap-parity.sh` | `ductus.md` ↔ `govern.md` byte-identity | installed |

`adopter_destinations` is the one that would have failed quietly: it fails toward an **empty set** rather than an error, so moving the manifest tables out from under it would have silently stopped suppressing adopter-destination findings instead of reporting a problem. Keeping them installed is what makes this a `framework/` change.

## Behavior

### What the adopter installs

`install.sh` and the `ductus` self-install step place the **installed half** only. The archive half is never installed into an agent, never curled by `install.sh`, and never byte-compared — it is read from the extracted archive the run fetches at `§Instructions` step 2.

### What the run reads, and when

The installed half runs through `§Pre-flight abort` exactly as before. Past the archive fetch the run resolves the archive half from the extracted tree and continues through it. A run that aborts in pre-flight never resolves it — the same set of sections `§Pre-flight abort` already skips.

### Staleness

The self-update check keeps comparing the installed file against upstream, now over the smaller file. The archive half needs no staleness check and gets none: it is re-fetched on every run, so it cannot be stale. This **narrows** what a stale installed copy can misdescribe — today an installed copy predating a procedure change carries the old procedure, and after this change it cannot.

### The retired bootstrap alias

`framework/bootstrap/govern.md` stays a byte-identical copy of the installed half, which is what audit Family 21 already requires and why it requires it: an unmigrated adopter's self-update byte-compares against that path and **writes whatever it finds** over their installed command. It needs no archive-half counterpart of its own — the archive half is resolved from the archive, which is the same tree for every adopter, so such a run reaches it by the path the installed half names.

### Rationale disposition

Rationale does not move as one class. (`specs/inbox.md` puts it at 17–25% of the file — a figure this spec carries as that item's rather than its own, because nothing here acts on it: the disposition below is a rule about *which* rationale moves, not a budget.)

- **Rationale that constrains execution stays** with the step it constrains, in whichever half that step lands. It is what stops an agent doing the tempting wrong thing, and a pointer to it elsewhere would be read after the mistake.
- **Dated incident narrative moves** to the spec that decided the fix, with a pointer back from the procedure.

## Acceptance Criteria

- [x] AC1: `framework/bootstrap/ductus.md` contains no step that requires the framework archive, and every step it does contain resolves within it — including both executable forward-dependencies named in §The boundary.
- [x] AC2: The archive half ships under `framework/bootstrap/`, is reachable from the extracted tree by the path the installed half names, and appears in no manifest that writes into an adopter's agent directory.
- [x] AC3: `install.sh` places only the installed half, for every one of its four agent arms, and its frontmatter-delimiter payload check still passes against it.
- [x] AC4: The `ductus` self-install step writes only the installed half, at each layout's install path, and the Post-Write Integrity Check's body assertion holds against it for every layout.
- [x] AC5: The self-update check byte-compares the installed half alone; no staleness check is defined for the archive half, and its freshness is stated to come from the archive fetch.
- [x] AC6: `framework/bootstrap/govern.md` is byte-identical to the post-split `framework/bootstrap/ductus.md`, so audit Family 21 passes unchanged; no `govern-procedure.md` counterpart is created beside it.
- [x] AC7: `adopter_destinations` in `check_artifacts.rs` still derives the same non-empty destination set from `framework/bootstrap/ductus.md` after the split as before it, verified by probe in both directions, and no `runtime/` source file changes.
- [x] AC8: Every consumer listed in §What the split does not reach still resolves its subject from `framework/bootstrap/ductus.md` after the split, each verified by running the family rather than by reading the assignment; any consumer whose subject does move is re-pointed and still reports a finding on an absent subject rather than treating an empty extraction as agreement.
- [x] AC9: `scripts/audit/run-all.sh` reports no findings, and the three generators plus `derive-dependencies` and `derive-references` report no drift.
- [x] AC10: The whole local gate passes: `npx markdownlint-cli2`, the six `lint-*.sh` scripts, `scripts/tests/*.sh`, `shellcheck -S warning` over the tracked shell set, and under `runtime/` `cargo fmt --check`, `cargo clippy --release --all-targets --locked -- -D warnings`, and `cargo test --release --locked`.
- [x] AC11: The reduction to the installed half is stated in the plan from a measurement taken **after** the split, against the 146,953-byte pre-split file, rather than from this spec's estimate.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What is the archive half named, and how does the installed half address it?** Resolved: `framework/bootstrap/ductus-procedure.md`, addressed as `{framework-root}/framework/bootstrap/ductus-procedure.md` — the framework root `§Archive fetch and extract` already derives and calls "the local mirror of the `ductus` repo for the rest of the run". (Resolved as `{tempdir}/ductus-main/…`; the root is now derived from the extraction rather than spelled, because the archive's directory is named after the ref the run resolved — see the signpost below.) No new resolution mechanism is introduced: the installed half keeps `§File Fetching`, so it holds the step that produces that root before it needs the path.

- **Exactly which sections land in the archive half?** Resolved by walking every level-2 section past `§Pre-flight abort` against the rule *does a step that runs before extraction read it?*, with a coverage assertion that each lands in exactly one column. The result is the table in §Measured: nine sections stay, nine move, 26.3% moves. Two answers were counter-intuitive and are recorded there with their reasons — `§Project Configuration` and `§File Fetching` both stay. `§Edge Cases` moves whole rather than being split: its entries are a reader's index, no step dispatches from it, and duplicating it would create the second copy of a single record that [§drift-prevention](../../framework/constitution.md#drift-prevention) forbids.

- **Does the archive half carry frontmatter and an `## Instructions` section of its own?** Resolved: **no**, and it is allowlisted as reference prose instead. Proven by probe in both directions — a `framework/bootstrap/*.md` file without `## Instructions` returns exit 2 from `ductus parse --check` (*"legacy prose — no parseable Instructions section"*), and `scripts/lint-procedure-parseability.sh` globs `framework/bootstrap/*.md`, so a new file there is in scope automatically. Giving it a synthetic `## Instructions` would satisfy the lint while obliging `check_step_references.rs`'s `BOOTSTRAP_FILES` to gain it, turning a `framework/` change into a `runtime/` change with a version bump and a release tag. The archive half genuinely **is** reference prose: `ductus exec ductus` walks `§Instructions`, which stays installed, so nothing dispatches from it. Operator decision, 2026-09-15. `runtime/legacy-prose-commands.txt` gains the entry and its header — which today asserts every entry is a `framework/commands/*.md` file — is corrected in the same change.

- **Does `runtime/tests/parity.rs` stage this file, and does any golden carry bytes from it?** Resolved: **no**, on both counts. The only fixture-local bootstrap procedure in the tree is `runtime/tests/fixtures/ductus-basic/framework/bootstrap/install.md`, for a command named `install`; `read_parity_spec` resolves by command name and never reaches `framework/bootstrap/ductus.md`. So no golden carries this file's bytes and no re-bless is implied — unlike `framework/commands/*.md`, where `implement-basic.jsonl` does.

- **Does this declare `cross-spec-impact` on the spec that owns the audit families?** Resolved: **no**, because no family's subject moves. Every extraction target — both manifest tables, `§Agent Registry`, `§Derived values`, the store probe, the `archive/` URL (since 061, the `codeload.github.com/<owner>/<repo>/tar.gz/` URL Family 36 derives the slug from), and `§Instructions`' numbered steps — lands in the installed half, so no `scripts/audit/*.sh` and no `runtime/src/**` file changes and no criterion of that spec is falsified. Recorded as a resolved question rather than left silent because the answer turns on the section assignment above: had `§File Fetching` moved, Family 36 would have lost its subject, and had `§Shared Files` moved, `adopter_destinations` would have started failing toward an empty set without erroring.

> **Signpost:** the archive half's address reads `{framework-root}` since [061 — Updates track the latest release tag](../061-updates-track-the-latest-release-tag/spec.md). A run now fetches the framework archive at the one source it resolves, the latest release by default, and GitHub names the extracted directory after that ref (`ductus-main/` for `main`, `ductus-<tag>/` for a release), so the bootstrap derives the root from the extraction instead of spelling `ductus-main/`. The decision this question records — one pointer, no new resolution mechanism — is unchanged.
