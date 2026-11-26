# section-digest-backfill

**Introduced in:** ductus 0.65.0
**Summary:** Write `section-digest` into each `analysis.md` that carries none and whose `analyzed-digest` still matches its spec's design subjects, so that spec's next `/{project}:analyze` judges each stored verdict's rule only against the sections changed since; leave every other record as it was (spec 088).

## Background

Since ductus 0.61.0 an analysis record carries `section-digest`, a digest of each section of its spec's design subjects (`spec.md`, `plan.md`, `data-model.md` and each scenario), and a run judges a rule whose verdict it stores only against the sections whose digest moved. A record written earlier carries none, so its spec's next analysis judges every loaded rule in full. A record whose `analyzed-digest` still matches its design subjects was judged against exactly the sections they hold now, so their digests are the ones its run would have written. A `done` spec's analysis read `spec.md` before the done flip rewrote its `status:` line, so a record also matches when `spec.md` with `status:` set to another lifecycle status reproduces the recorded digest, and it gains that text's digests. The write asserts no analysis that did not happen: the recorded digest proves the run read that text, which `audit-record-relocate`'s "Do not backfill the digests" could not say of its own rewrite.

## Procedure

1. **Run the backfill.** Read `[pinned] files` from the active config file, then run `.ductus/bin/ductus backfill-section-digest` from the project root with one `--pinned {path}` per entry (`.ductus/bin/ductus.exe` on Windows). Run it through the CLI even when the `ductus` server is live in this session, never as the MCP tool: the server keeps the binary it started with, which can predate the one Pre-flight brought the pointer to. A record already carrying `section-digest` is counted and left, so a converged project gets no write.

2. **Not checked.** When the call prints no JSON object on stdout, it failed or the binary predates ductus 0.65.0: report `section-digest-backfill: not checked — .ductus/bin/ductus backfill-section-digest failed: {its error}; run it by hand once the runtime is current, or each record gains section-digest at its spec's next /{project}:analyze` and stop. Never report it as nothing to backfill. A JSON result is read at step 3 whatever the exit status: the CLI exits 1 when `refused-count` is non-zero.

3. **Read the result.** Read `written`, `unmatched`, `pinned`, `unreadable` and `refused` from the result, or from the file `lists-file` names when `form` is `file`; each names a feature directory under `specs-root`. When `refused-count` is non-zero, run step 1 once more, since each refused record changed while the call read it: report the written records of both calls, and every other list from the second.

4. **Report.** Write nothing else: no spec, record or configuration.
   - When `written-count` is non-zero: `section-digest-backfill: wrote section-digest into {N} analysis record(s)`.
   - Each `unmatched` entry: `section-digest-backfill: {specs-root}/{feature}/analysis.md left without section-digest — its design subjects changed since that analysis, so its spec's next /{project}:analyze writes it`.
   - Each `pinned` entry: `warning: {specs-root}/{feature}/analysis.md is pinned; leaving it without section-digest — run backfill-section-digest by hand if you want it written.`
   - Each `unreadable` entry: `section-digest-backfill: {specs-root}/{feature}/analysis.md not examined: {reason}`.
   - Each `refused` entry: `section-digest-backfill: {specs-root}/{feature}/analysis.md changed while it was read and was left as it is — run .ductus/bin/ductus backfill-section-digest again`.

5. **Leave `done` specs at `done`.** The run writes only `analysis.md` and leaves no unchecked box or unticked criterion, so under [§spec-lifecycle](../constitution.md#spec-lifecycle) a `done` spec stays `done`.

## Verification

- A record whose `analyzed-digest` matches its design subjects, as they stand or with `spec.md`'s `status:` set to another status, gains `section-digest`, and with that block removed is byte-identical to before.
- A record whose design subjects changed since its analysis is named and left byte-identical.
- A second run writes nothing.
- A pointer binary with no `backfill-section-digest` subcommand gets the not-checked line, never no line, and leaves the tree as it was.
- `.ductus/config.toml` differs from before the run only by `[migrations].last_applied`.
