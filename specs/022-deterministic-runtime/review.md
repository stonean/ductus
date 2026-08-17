---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-17T14:02:09Z
reviewed-against: 5e6df7fca8acbcb1bb79e91e61465ab584c040e3
diff-base: 8c57d4bd227b0c2fe880d0308dd7f502726ce7dc
must-violations: 0
should-violations: 0
low-confidence: 1
captured-issues: 2
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

First full-scope review of this spec since `9a9c38b3` (2026-08-03). The prior review was narrow — one scenario's staleness-scope correction — and its diff base predates this window, so all 97 commits and 557 files here were unreviewed: +9209/-1760 across 150 code files spanning `0.29.0` through `0.29.8`. **0 MUST, 0 SHOULD outstanding, 1 low-confidence — not blocking.** One SHOULD was found and fixed before this report was written: `install.sh` reported a successful bootstrap install without verifying the payload was a bootstrap. `curl -f` rejects an HTTP error status but a 200 carrying a captive-portal page or an empty body passes it, and the antigravity arm — which splits `ductus.md` on its frontmatter — then produced a 21-byte `SKILL.md` holding only the wrapper: an empty skill, exit 0, "installed the antigravity bootstrap". Reproduced rather than reasoned about, and fixed in `5e6df7f` with a shape check ahead of the `case` so no arm can report an install it did not verify; the guard was proven in both directions (real bootstrap 126225 bytes/6 delimiters accepted, captive-portal page 46/0 and empty body 0/0 both refused, each naming its counts and the fetch URL). No `runtime/` change, so no version bump or tag. The one remaining entry is low-confidence by design: `CFG-ENV-001` is MUST-worded and `host_is_insecure_allowed` does read the env per call, but the read is bounded (1 + at most `MAX_FETCH_REDIRECTS` hops), nothing anywhere calls `set_var`/`remove_var` so a cached read would be semantically identical, and re-reading per hop is arguably correct because each redirect target gets a full independent screen — recorded as plausible-but-unconfirmed rather than ranked either way. What the passes found clean is the substance of this review: `schema/paths.rs` validates the configured spec-root against a strict `[A-Za-z0-9_-]` allowlist that rejects separators, `..`, a lone `.`, and regex metacharacters with a test per case, and `resolve_config` deliberately makes one existence probe so a `/ductus` migration cannot land between a read and its provenance tag (`BE-RACE-001`); `mechanical_sweep` fails **closed** on a diff it could not finish reading, with the direction argued against `QUAL-CLAIM-001` in its own doc comment, and discards a partial walk rather than folding a prefix that would read as exempt; `label_criteria` assigns `next.max(max_label + 1)` so deleting the highest criterion cannot reissue its label, and a malformed `next-criterion` errors instead of defaulting; claim honesty is structural rather than incidental across `check_artifacts`, `check_orphaned_references`, and `read_spec`, each carrying a `skipped`/`unreadable` list with `clean` deliberately not widened to absorb it; `write_review` writes the inbox ahead of the report so a report can never assert a capture that did not happen, and delegates to `append_inbox` rather than reimplementing it; `fetch_archive`'s SSRF screen is https-only, resolves and screens every candidate address, unwraps IPv4-mapped IPv6 so `::ffff:127.0.0.1` cannot smuggle, and pins the connection with `resolve_to_addrs` to close the DNS-rebinding window between validation and connect; every resource bound is a named constant with under/at/over boundary tests (`MAX_FETCH_BYTES`, `MAX_EXTRACT_BYTES`, `MAX_EXTRACT_ENTRIES`, `MAX_FETCH_REDIRECTS`), satisfying `BE-INPUT-006` and `CFG-CONST-003/004`. The two past release-gate defects `AGENTS.md` records are both closed here and verified in the file rather than assumed: `release-assets` now needs `[audit, build]` and `publish` needs `[audit, build, release-assets, acquire]`, so the self-audit gates the binaries adopters download and not merely the crates.io publish, and `fetch-depth: 0` carries an explanatory comment naming Family 19 on both the release and framework-checks audit checkouts. `mechanical_sweep_parity` extracts the real Python out of `scripts/audit/review-freshness.sh` rather than a copy of it and asserts `compared > 0`, so a green result that examined nothing is itself a failure — `QUAL-CLAIM-001` applied to the test that pins the rule's two implementations. Only five `expect` calls exist in non-test changed code, all on genuinely infallible operations (hardcoded regex compilation, TOML serialization of a table just constructed) with justifying messages. Whole-surface evidence at this HEAD: `clippy --release --all-targets -D warnings` exit 0; the full test suite exit 0 with no failures; `shellcheck -S warning` clean across all 35 changed shell scripts; and `scripts/audit/run-all.sh` exit 0 run against committed state, after the commit rather than before it.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: CFG-ENV-001 — insecure-host allowlist is read from the environment per call rather than cached once

- **File**: `runtime/src/primitives/fetch_archive.rs:295-299`
- **Rule**: Every environment variable MUST be declared as either optional with a default fallback defined as a named constant, or required with no default (in which case CFG-ENV-003 governs its startup validation). Secrets MUST be declared required and MUST NOT carry an in-application default value (see BE-DATA-003). All environment variables MUST be read once at startup and the value cached; per-call reads from os.environ (or equivalent) are forbidden.
- **Finding**: `host_is_insecure_allowed` calls `std::env::var(FETCH_ALLOW_INSECURE_HOSTS_ENV)` on each invocation, and `validate_fetch_url` calls it for the initial URL plus every redirect hop, so a single fetch performs up to 1 + MAX_FETCH_REDIRECTS reads instead of one cached read. Recorded low-confidence because the rule's three stated harms do not land here: the count is bounded rather than a hot path, no code anywhere in the crate calls `set_var`/`remove_var` so a value cached at first use would be identical, and the default is documented at the constant so it is not invisible to readers. Re-reading per hop is also defensible on its own terms, since each redirect target is meant to receive a complete independent screen.
- **Auto-fixable**: yes
- **Suggested fix**: Wrap the parsed allowlist in a `std::sync::LazyLock<Option<Vec<String>>>` (or resolve it once in `run` and thread it into `validate_fetch_url`) so the variable is read once per process. Safe as written — nothing mutates the environment in-process, and the existing parity tests set the variable on a subprocess before it starts, so a startup read still observes it.

## Waived findings

*None.*

## Captured issues

- Architectural exploration: re-frame the runtime's LLM extension points (`writeCode`, `writeSpecBody`, `assessSpecQuality`, future multi-turn points) as named Anthropic-style Skills the host loads at the seam, rather than ad-hoc JSON envelopes. Potential benefits: structural cache anchoring (Skills are a natural cache boundary); third-party hosts integrate against an emerging Skills protocol instead of ductus-specific JSON; `constitution-excerpts` becomes a bundled resource rather than an inline string array. Speculative — depends on Anthropic's Skills protocol stabilizing and is a larger redesign than 022's current scope. Revisit after the writeCode payload-bundling scenario on 022 ships and the cache-anchored shape proves out the pattern. Surfaced 2026-05-19 during runtime-improvement investigation. **On hold per user 2026-07-11.**
- [ ] rule: `AGENTS.md` carries adopter-beneficial rules in a file adopters never receive. It is contributor-only — the Shared Files manifest ships `framework/constitution.md` → `.ductus/constitution.md`, the rule files, the hooks and the templates, but never this repo's `AGENTS.md` — so a rule learned here that is true for *any* ductus project reaches nobody. Surveyed 2026-08-17: 56 entries, of which ~12 are strongly universal (both §Design Principles entries; a markdown link in a spec body creating a `dependencies:` edge; `git checkout -- specs/{feature}/` silently reverting uncommitted pipeline state; never `git add -A` because it sweeps untracked `/specify` drafts; never hand-writing an `AC{n}` label; `create-scenario`'s auto-appended question scaffolding; routing a new rule to its surface's home spec instead of a new spec; re-opening a done spec via `set-status` for on-disk-edit-only cases; the prose-claim sweep that identifier sweeps miss; `fetch-depth: 0` for history-reading CI checks; treating `.ductus/config.toml` as a shared database rather than one spec's schema; and recording a superseded acceptance criterion in the spec body), ~10 borderline, and ~24 genuinely ductus-only (trunk-based commits, the retired repo name, the `runtime/` tag loop, the agent registry, cargo/rustup gotchas, primitive wiring sites). §recommendations was promoted to the constitution on 2026-08-17 as the first instance and is the model: canonical text in the constitution, a contributor-side mirror in `AGENTS.md` pointing at it. The rest needs a spec — this is a governed artifact that ships to every adopter, and promoting a dozen entries is not a one-pass edit. Prompted by the operator asking whether the recommendation rule belonged in the constitution.

## Observations

*None.*

## Skipped passes

*None.*
