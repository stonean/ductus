# 088 — Rule judgments cost only what changed Data Model

Two fields join the analysis record, whose other fields [066's data model](../066-audit-records-and-the-completion-gate/data-model.md) and [082's](../082-rule-assessments-converge-across-runs/data-model.md) define; `discover-rule-files`' `verdicts` gains one list, and its currency narrows to the rules a change reaches; and one primitive, `backfill-section-digest`, is added.

## Record fields

| Field | Type | Meaning |
| --- | --- | --- |
| `rules-judged` | integer | Each distinct assessable loaded rule the run judged, in full or against the changed sections only, whose judgment the record keeps |
| `rules-carried` | integer | Each distinct assessable loaded rule the run did not judge whose stored verdict stood: a fired verdict carried forward, or a rule current and not fired |

`write-analysis` writes both on every write, `0` when no rule file is loaded, beside the rule-assessment fields of 082's data model, and accepts neither as an argument. A record written before 088 holds neither, which says nothing of what its run judged, never that it judged none.

- A rule whose `addressed` citation does not resolve, or whose carried `waived` verdict lost its departure entry, is in `unjudged-rules` and counted in neither.
- So `rules-judged` + `rules-carried` + the length of `unjudged-rules` is the number of distinct assessable rules the loaded files define, the rules a run assessed.

```yaml
rule-digest:
  quality-cross.md: 51ab…
rules-judged: 1
rules-carried: 4
unjudged-rules:
  - QUAL-TEST-001
```

## `discover-rule-files` with a `feature`: `rejudged-files`

A loaded rule file whose digest differs from the record's `rule-digest` entry, the loaded set being the one that entry names, is compared rule by rule with its copy at the record's `analyzed-against` commit. A rule is current, subject to 082's other conditions, when that copy holds a rule of its ID whose Statement and Verification, the fields `section_digests` hashes for it besides the ID, are the same. A rule whose text differs, or that the copy does not hold, is in `judge`. A file whose digest moved while no rule's text did, a reorder or a rule removed, keeps each remaining rule's verdict.

A loaded rule in any file is in `judge` too when its Statement or Verification names a changed rule: one whose text differs, one new to its file, one its file held at `analyzed-against` and no longer holds, or any rule of a file listed under `rejudged-files`. A name is a token the rule-ID grammar matches that is such a rule's ID and not the naming rule's own; a rule naming one only through another is not judged for it.

The copy stands in for the text the run judged only when its own file digest, by `section_digests`, equals the record's entry. When it does not, or cannot be read, every rule of the file is in `judge`, and `verdicts` names the file under `rejudged-files`:

```json
{ "rejudged-files": [{ "file": "security-backend.md", "reason": "commit-not-held" }], "rejudged-files-count": 1 }
```

| `reason` | When |
| --- | --- |
| `no-analyzed-against` | The record names no `analyzed-against` |
| `commit-not-held` | The repository holds no commit by that sha: a shallow clone, a commit never fetched, or a sha that names none |
| `not-at-commit` | The commit's tree holds no such file at the rule-file directory's path, or its blob is not UTF-8 |
| `judged-copy-differs` | The copy at the commit does not digest as the record's entry says: the run read rule text the commit does not hold |
| `history-unreadable` | The project is in no repository, or git could not read the commit, its tree or the blob |

`rejudged-files` is absent when empty, with `rejudged-files-count` inline beside it in both forms and absent at zero, bounded as `bounded_lists::place` bounds the other `verdicts` lists. The exec walker and `write-analysis` decide currency by the same comparison and report no reason.

## `backfill-section-digest`

One argument, `pinned`: a list of paths from the project root, the project's `[pinned] files`, which the caller reads and passes, as it does to `apply-manifest`; CLI `--pinned`, repeatable. It walks every feature directory under the configured spec root by `list_feature_dirs`, and for each one holding an `analysis.md`:

- **Pinned.** A record whose path `pinned` names is named under `pinned` in the result and left untouched.
- **Read.** By `load_analyze_record`: a record that does not parse is named under `unreadable` with its reason and left untouched; one carrying `section-digest` is counted under `has-section-digest`; one whose design subjects hold no section is counted under `no-design-subject`.
- **Match.** The record matches when its `analyzed-digest` entries for the design subjects equal their digests as they stand, or with `spec.md`'s `status:` value set to another of the lifecycle statuses, tried in their canonical order. `matched-at` names the status of the `spec.md` text that matched.
- **Write.** A matching record gains the `section-digest` of that text, rendered as `write-analysis` renders the field and placed before its frontmatter's closing fence, every other byte as it was. A record that matches at no status is named under `unmatched` and untouched.
- **Guard.** The record is read again under the project-wide lock just before the rename, and one another writer changed since it was read is named under `refused` and left as that writer made it.

```json
{
  "specs-root": "specs", "examined": 79,
  "written": [{ "feature": "002-project-scaffolding", "matched-at": "in-progress" }], "written-count": 13,
  "unmatched": ["060-exec-analyze-assesses-each-loaded-rule"], "unmatched-count": 36,
  "has-section-digest": 30, "no-design-subject": 0,
  "unreadable-count": 0, "refused-count": 0, "form": "inline"
}
```

`examined` counts the feature directories holding an `analysis.md`. `pinned`, `unreadable` and `refused` are absent when empty, each with its count inline in both forms; past `[artifacts] read-size-bytes` `written`, `unmatched`, `pinned`, `unreadable` and `refused` are absent, their counts inline, and `lists-file` names the file under `.ductus/results/` holding them. The CLI exits non-zero when `refused-count` is non-zero, naming each refused record on stderr with a line saying to run it again.
