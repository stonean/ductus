# 080 — Deterministic runtime efficiency Data Model

The shapes this feature adds or changes. Each one is checked, when it lands, against a record the runtime actually wrote (§drift-prevention), not against the code.

## `render-status`

Arguments: none, since `ductus exec` passes a step's primitive no arguments.

```json
{ "rendered-markdown": "Target: …\n\n| Feature | … |\n…", "project": "ductus" }
```

- `rendered-markdown` — the markdown rendering of the view below.
- `project` — the slash-command namespace `Host::load` resolves, the `{project}` of every `/{project}:` hint in the view. Pi's bridge names its registered command from it.

If the Pi verifying run records that a displayed message renders no markdown, the primitive gains `format` (`markdown` by default, or `text`). It does not exist before then.

## The pipeline view

One model (`runtime/src/primitives/pipeline_view.rs`), rendered as markdown (`render-status`, model path) or as fixed-width text (`ductus hook status`, hook path). The two differ only in the table and its emphasis; every other line is the same in both. The blocks, in order, separated by blank lines:

1. **Target block.** `Target: {feature} / {status} / next: {next-action}`, ending `— session {label}` for an identified process. Then `Scenario: {slug} ({section}) — open-questions: {n}` when a scenario is targeted, then one `Notice: …` line per session notice and one `Unreadable: …` line per unreadable session file. With no target, the block's first line is `No session target.`
2. **Table.** The columns are Feature, Status, Plan, Tasks, Data-model, Scenarios, Dependencies and Next Action, one row per spec in directory order.
   - *Markdown:* a pipe table, with the target's Feature cell in bold.
   - *Text:* each column padded to its widest cell, columns separated by two spaces, with no pipes, separator row or emphasis. Every row starts with a two-character gutter: `→` and a space on the target's row, two spaces on every other row.
3. **Summary.** One item per line, each starting with its label, in this order: `Counts:`, `Blocked:`, `Recovery:`, `Scenario questions:`, `Folds:`, `Tags:`, `Disabled rule files:`, `Unknown config keys:`, then the reopenings line and `Inbox:`.
   - The reopenings line keeps the form spec 079 gave it, `Reopenings since {date}: …` or `Reopenings: ? …`.
   - Only `Counts:`, the reopenings line and `Inbox:` render on every run; the others render when they apply.
   - `Scenario questions: {n} in {m} spec(s)` is followed by one line per unresolved question, indented two spaces: `{spec} · {scenario}: {question}`, with the question flattened to one line.
4. **References.** `References:`, then each spec that declares references, its slug (bold in markdown) followed by one `- …` line per reference. Present only when a spec declares references.
5. **Next actions.** `Next actions:`, then one line per next action, indented two spaces: `{next-action}: {spec}, {spec}`. It groups every spec but the target whose Next Action is not `done (spec is complete)`, in order of first appearance in the table. Absent when there is none.
6. **Prompt.** `Run /{project}:target <feature> to select one.`, the view's only `/{project}:target` hint.

**Bound.** Each rendering is held to `[artifacts] read-size-bytes` in bytes; a rendering exactly at the bound fits. When it is over, a block of its own follows the table, since a line directly under a markdown table would read as a row:

1. the `done (spec is complete)` rows are removed from the table, and the block's first line is `done (spec is complete): {n} spec(s)`;
2. if it is still over, the most leading rows that fit are kept, and the block ends `{n} spec(s) not shown — the view is over the read size`.

A cut view therefore never reads as complete. A view whose other blocks alone are over the bound keeps no row and still counts them.

## `ductus hook status`

A host surface, not a primitive.

- **Input.** Claude Code's `UserPromptExpansion` input on stdin, which the hook reads to its end and uses no field of. The project root is `CLAUDE_PROJECT_DIR`, else the working directory.
- **Success.** Exit 0 and one JSON object on stdout:

```json
{
  "decision": "block",
  "reason": "<the text rendering>",
  "hookSpecificOutput": { "hookEventName": "UserPromptExpansion", "suppressOriginalPrompt": true }
}
```

- **Failure.** Exit 1, with the error on stderr and nothing on stdout. Exit 2 would block the expansion, so the hook never uses it.

## The hook entry in `{cli-config-dir}/settings.local.json`

```json
{
  "hooks": {
    "UserPromptExpansion": [
      {
        "matcher": "{project}:status",
        "hooks": [
          { "type": "command", "command": "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus\" hook status", "timeout": 10 }
        ]
      }
    ]
  }
}
```

The entry is ductus's when an inner hook's `command` equals the string above. On Windows the string names `ductus.exe`. Entries are compared by that string alone, and every other entry, whatever its event or matcher, is left as it is.

## `merge-status-hook`

| Argument | Type | Meaning |
| --- | --- | --- |
| `path` | string | The settings file, repo-relative |

| Result field | Type | Meaning |
| --- | --- | --- |
| `action` | `created` / `updated` / `unchanged` / `skipped` | `created` — no file existed; `updated` — the entry was added; `unchanged` — it was already there |
| `claude-version` | string or null | The version `claude --version` reported, null when unreadable |
| `reason` | string or null | Why the entry was skipped: older than 2.1.101, or the version unreadable |

## `ManifestEntry.tool-search-line`

`tool-search-line`: boolean, `false` when absent. When `true`, `apply-manifest` derives the entry's step tools after substitution. It drops the always-loaded set and CLI-only names from them, and, when any remain, appends to the `## Instructions` preamble blockquote:

```markdown
>
> **Claude Code:** load this command's `ductus` tools in one `ToolSearch` call before step 1: `select:mcp__ductus__{a},mcp__ductus__{b}`
```

The names are sorted. When none remain, nothing is written.

## `_meta` on `tools/list`

Each tool the installed `target.md`'s steps name carries `"_meta": { "anthropic/alwaysLoad": true }`. No other tool carries a `_meta` key.

## Bounded results

A primitive in the grows-set takes the shape `process-decisions` and `compute-review-scope` already return through `bounded_lists::place`:

- `form` is `inline` while the serialized result fits `read-size-bytes`, and `file` past it;
- a `{list}-count` per moved list is present in both forms;
- each moved list is present only in the inline form;
- in the file form, `lists-file` names `.ductus/results/{primitive}/{feature}/{sha256}.json`, which holds the lists whole, one entry per line.

Each member's row is added to this table as it lands.

| Primitive | Lists moved | Counts |
| --- | --- | --- |
| `read-tasks` | `tasks` | `tasks-count` |
| `dashboard` | `specs`, `tags-union` | `specs-count`, `tags-union-count` |
