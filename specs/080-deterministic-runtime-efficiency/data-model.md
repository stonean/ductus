# 080 — Deterministic runtime efficiency Data Model

The shapes this feature adds or changes. Each one is checked, when it lands, against a record the runtime actually wrote (§drift-prevention), not against the code.

## `render-status`

Arguments: none, since `ductus exec` passes a step's primitive no arguments.

```json
{ "rendered-markdown": "Target: …\n\n| Feature | … |\n…", "project": "ductus", "form": "inline" }
```

- `rendered-markdown` — the markdown rendering of the view below, held with the rest of the result to the read size.
- `project` — the slash-command namespace `Host::load` resolves, the `{project}` of every `/{project}:` hint in the view. Pi's bridge names its registered commands from it on a runtime that serves no `render-help`, whose `project` it reads otherwise.
- `form` — `inline` while the whole view fits, and `file` past it, when the view is cut.
- `lists-file` — in the file form only: `.ductus/results/render-status/all-specs/{sha256}.json`, which holds `{"view": [line, …]}`, the whole view's lines, one per entry. Every line of the cut view that stands for a cut names it.

The primitive takes no `format`: the Pi verifying run recorded that a displayed message renders markdown (`research.md`, task 12).

## `render-help`

Arguments: none.

```json
{ "guide": "## ductus — Spec-Driven Development Pipeline\n\n…", "project": "ductus" }
```

- `guide` — the guide in the installed help copy, the first of `Host::command_file_candidates("help")` that exists. It is the text between the first line after the copy's `## Instructions` heading that is exactly `---` and the last such line. Each line that is wholly an HTML comment is removed, along with a blank line the removal would leave beside another, and the leading and trailing blank lines are trimmed. It is markdown, with the project's name in each command as the copy spells it.
- `project` — the slash-command namespace `Host::load` resolves, read without any spec, which Pi's bridge names `{project}-status` and `{project}-help` from at load.
- No installed copy is an error naming every path tried. A copy with no `## Instructions` heading, fewer than two such lines after it, or nothing but blank lines and comments between them is an error naming the copy.

## `read-section`

| Argument | Type | Meaning |
| --- | --- | --- |
| `path` | string | The document, repo-relative. A path leaving the project is refused |
| `headings` | list of strings, at least one | Each section's heading text, without the `#` marks. An empty list is refused |

| Result field | Type | Meaning |
| --- | --- | --- |
| `path` | string | The document, as passed |
| `sections` | list of `{heading, text}` | Each heading found, in the order asked, with its section: from its heading line to the next heading of the same or a higher level, outside fenced blocks and HTML comments. Absent in the file form |
| `sections-count` | integer | Entries in `sections`, in either form |
| `not-found` | list of strings | Each heading asked for that the document lacks |
| `form`, `lists-file` | | As *Bounded results* below gives them |

## `discover-rule-files`' rules

| Argument | Type | Meaning |
| --- | --- | --- |
| `rules` | boolean, `false` when absent | Also list each selected file's rules, then each `extra-files` entry's |
| `verification` | boolean, `false` when absent | With `rules`, add each rule's Verification. Without `rules` it is refused |
| `all` | boolean, `false` when absent | Select every rule file in the directory, past its suffix, `[rules] surfaces` and the disabled entries, neither of which is then validated, as analyze loads them |
| `extra-files` | list of strings | Repo-relative rule files outside the directory, listed after the selected ones. Without `rules` it is refused, and one leaving the project is refused |

| Result field | Type | Meaning |
| --- | --- | --- |
| `rule-files` | list of `{file, rules, other-headings}` | With `rules`, each file read: its rules in heading order, each `{id, heading, tier, statement, verification}`, and under `all` `other-headings`, each `{line, heading, body}` that opens no rule, `body` being the lines under it up to the next heading, blank lines at either end dropped. Absent in the file form |
| `rule-files-count` | integer | Entries in `rule-files`, with `rules`, in either form |
| `unreadable` | list of `{file, reason}` | Each file that could not be read, never dropped |
| `form`, `lists-file` | | As *Bounded results* below gives them, on every call: `notices` is bounded with `rules` and without (022 task 32) |

`tier` is `must` or `should`, from the Statement's RFC 2119 keyword, and absent when it carries neither. `verification` is present only with the argument of that name.

## The pipeline view

One model (`runtime/src/primitives/pipeline_view.rs`), rendered as markdown (`render-status`, model path) or as fixed-width text (`ductus hook status`, hook path). The two differ only in the table and its emphasis; every other line is the same in both. The blocks, in order, separated by blank lines:

1. **Target block.** `Target: {feature} / {status} / next: {next-action}`, ending `— session {label}` for an identified process. Then `Scenario: {slug} ({section}) — open-questions: {n}` when a scenario is targeted, then one `Notice: …` line per session notice and one `Unreadable: …` line per unreadable session file. With no target, the block's first line is `No session target.`
2. **Table.** The columns are Feature, Status, Plan, Tasks, Data-model, Scenarios, Dependencies and Next Action, one row per spec in directory order.
   - *Markdown:* a pipe table, with the target's Feature cell in bold.
   - *Text:* each column padded to its widest cell, columns separated by two spaces, with no pipes, separator row or emphasis. Every row starts with a two-character gutter: `→` and a space on the target's row, two spaces on every other row.
3. **Summary.** One item per line, each starting with its label, in this order: `Counts:`, `Blocked:`, `Recovery:`, `Scenario questions:`, `Folds:`, `Tags:`, `Disabled rule files:`, `Unknown config keys:`, then the reopenings line and `Inbox:`.
   - The reopenings line keeps the form spec 079 gave it, `Reopenings since {date}: …` or `Reopenings: ? …`.
   - The reopenings line and `Inbox:` render on every run, and `Counts:` whenever a spec exists; the others render when they apply.
   - `Scenario questions: {n} in {m} spec(s)` is followed by one line per unresolved question, indented two spaces: `{spec} · {scenario}`, so a scenario carrying three questions has three lines. The question's text is not in the view.
4. **References.** `References:`, then each spec that declares references, its slug (bold in markdown) followed by one `- …` line per reference. Present only when a spec declares references.
5. **Next actions.** `Next actions:`, then one line per next action, indented two spaces: `{next-action}: {spec}, {spec}`. It groups every spec but the target whose Next Action is not `done (spec is complete)`, in order of first appearance in the table. Absent when there is none.
6. **Prompt.** `Run /{project}:target <feature> to select one.`, the view's only `/{project}:target` hint.

**Bound.** The result each surface serializes is held to `[artifacts] read-size-bytes` in bytes: `render-status`'s result on the model path, and the block response `ductus hook status` prints. A result exactly at the bound fits. Each line standing for a cut ends `not shown — the view is over the read size`, followed by `; the whole view is in {lists-file}` on the model path. When the result is over, each step applies in turn until it fits:

1. the `done (spec is complete)` rows are removed from the table, and a block of its own follows the table with the line `done (spec is complete): {n} spec(s)`, since a line directly under a markdown table would read as a row;
2. the most leading rows that fit are kept, and that block ends `{n} spec(s) not shown — …`;
3. with no row left, `Next actions:` keeps the most leading names that fit, in group order. A group cut short ends `, … {n} more`, a group with none left reads `{next-action}: {n} spec(s)`, and the block ends with an indented `{n} of {total} spec(s) not shown — …` line;
4. each of these collapses in turn, and after each the rows, then the names, are fitted again:
   - the References block, to `References: {n} spec(s) not shown — …`;
   - `Disabled rule files:` and `Unknown config keys:`, each to its count and config file;
   - `Tags:`, `Folds:`, `Recovery:` and `Blocked:`, each to its count;
   - the scenario-question lines, to one indented `{n} question line(s) not shown — …` line under their header;
   - the out-of-set statuses in `Counts:`, to one `{n} spec(s) in {m} other status(es)` part;
   - `Next actions:`, to `Next actions: {n} spec(s) under {m} next action(s) not shown — …`;
   - the Notice and Unreadable lines, to `Notice: {n} notice(s) not shown — …` and `Unreadable: {n} session file(s) not shown — …`.

A cut view therefore never reads as complete. What every step leaves is the floor, the lines *Bound (AC24)* in `plan.md` lists, and a read size below it gets the floor, over the bound.

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

## `ductus hook help`

As `ductus hook status`, except that the reason is `render-help`'s `guide`, unchanged, and the failures are `render-help`'s errors.

## The hook entries in `{cli-config-dir}/settings.local.json`

```json
{
  "hooks": {
    "UserPromptExpansion": [
      {
        "matcher": "{project}:status",
        "hooks": [
          { "type": "command", "command": "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus\" hook status", "timeout": 10 }
        ]
      },
      {
        "matcher": "{project}:help",
        "hooks": [
          { "type": "command", "command": "\"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus\" hook help", "timeout": 10 }
        ]
      }
    ]
  }
}
```

An entry is ductus's status entry when an inner hook's `command` equals the first string above, and its help entry when it equals the second. On Windows each string names `ductus.exe`. Entries are compared by those strings alone, and every other entry, whatever its event or matcher, is left as it is. Each matcher holds a `:`, so Claude Code tests it as an unanchored regular expression on the command name.

## `merge-permissions`' canonical call

| Argument | Type | Meaning |
| --- | --- | --- |
| `canonical` | boolean, `false` when absent | Merge Claude's canonical allow, deny and retired sets, which the runtime holds, beside any `allow`, `deny` and `revoke` passed, and ensure the directories below |

Under `canonical`, `allow` is step 2's set of `framework/bootstrap/configure/claude.md`, its static entries in step 2's order and then each served tool in the registry's order, which is not the order of step 2's generated block; `deny` is step 3's and `revoke` is step 4's, each in its order. Each is followed by the entries the caller passed. The caller's `allow` entries are still refused past the ceiling. `permissions.additionalDirectories` is created when absent and then holds, compared by exact string, the absolute path of the configured spec root and of `{config-dir}/commands/{project}`, with no trailing slash. `{config-dir}` is the settings file's own directory. Every other entry stays where it is.

| Result field | Type | Meaning |
| --- | --- | --- |
| `revoked` | list of strings | The distinct retired entries removed, in the retired set's order |
| `directories-added` | list of strings | The paths appended to `additionalDirectories`, empty unless the call was canonical |

Every other field of the result is as before.

## `merge-status-hook`

| Argument | Type | Meaning |
| --- | --- | --- |
| `path` | string | The settings file, repo-relative |

| Result field | Type | Meaning |
| --- | --- | --- |
| `action` | `created` / `updated` / `unchanged` / `skipped` | `created` — no file existed, and both entries were written; `updated` — either entry was added; `unchanged` — both were already there |
| `claude-version` | string or null | The version `claude --version` reported, null when unreadable |
| `reason` | string or null | Why the entries were skipped: older than 2.1.101, or the version unreadable |

## `ManifestEntry.tool-search-line`

`tool-search-line`: boolean, `false` when absent. When `true`, `apply-manifest` derives the entry's run tools after substitution: its step tools, and each tool the command file invokes anywhere or a section it cites invokes (`plan.md`, *Batched (AC29)*). It drops the always-loaded set and CLI-only names from them, and, when any remain, appends to the `## Instructions` preamble blockquote:

```markdown
>
> **Claude Code:** load this command's `ductus` tools in one `ToolSearch` call, and send the command's first `ductus` call in that same message, since a tool can be called beside the search that loads it: `select:mcp__ductus__{a},mcp__ductus__{b}`
```

The names are sorted. When none remain, nothing is written.

## `_meta` on `tools/list`

Each tool the installed `target.md`'s steps name carries `"_meta": { "anthropic/alwaysLoad": true }`. No other tool carries a `_meta` key.

## Bounded results

A primitive bounded through `bounded_lists::place`, the grows-set and `discover-rule-files`, takes the shape `process-decisions` and `compute-review-scope` already return:

- `form` is `inline` while the serialized result fits `read-size-bytes`, and `file` past it;
- a `{list}-count` per moved list is present in both forms, except `dashboard`'s session lists and `derive-dependencies`' `unwritable`, below;
- each moved list is present only in the inline form, where `scenario-files-unreadable`, `cross-spec-impact` and `dashboard`'s session lists are also absent when empty, as before, and `derive-dependencies`' `unwritable` is too;
- in the file form, `lists-file` names `.ductus/results/{primitive}/{key}/{sha256}.json`, which holds the lists whole under their inline keys, pretty-printed, so a list of strings holds one entry per line. `{key}` is the feature argument, `rewrite-spec-links`' `from`, `all-specs` for a corpus-wide result, `repository` for `check-corpus-links`' repository scope, `paths` for `lint-markdown`, and the path's file stem for `read-section`, or `document` when the stem is empty or holds a character outside letters, digits, `-` and `_`.

A call that writes, `rewrite-spec-links`, an applied prune or `derive-dependencies` with `write`, has written when its result is bounded, so a list file that cannot be written fails a call whose writes stand. Each member's row is added to this table as it lands.

| Primitive | Lists moved |
| --- | --- |
| `read-spec` | `sections`, `acceptance-criteria`, `open-questions`, `scenario-open-questions`, `scenario-files-unreadable`; the result carries no audit record in either form |
| `read-tasks` | `tasks` |
| `read-section` | `sections` |
| `derive-boundary` | `boundary` |
| `lint-markdown` | `violations` |
| `prune-tasks` | `sections` for one feature; `features`, `skipped` under `all` |
| `prune-plan` | `features`, `skipped` under `all` |
| `dashboard` | `specs`, `tags-union`; `session-notices` and `session-unreadable` when non-empty, each with a count present only in the file form, since the expiry sweep leaves a session file it cannot parse; `config`'s `unknown-keys`, under `config` in the file, its count inline there in both forms (022 task 32) |
| `check-review-gate` | `violations`, `cross-spec-impact`, `analyze-stale-paths` |
| `check-artifacts` | `findings`, `skipped` |
| `check-corpus-links` | `broken`, `examined`, `skipped` |
| `check-unfolded-specs` | `unfolded` |
| `rewrite-spec-links` | `rewritten`, `skipped`, `kept-tails`, `mentions` |
| `discover-rule-files` | `notices` (022 task 32); `rule-files`, with `rules` |
| `derive-dependencies` | `updated`, `unwritten`, `untracked-skipped`, `absent`, `cycles`, `unparseable`, `unwritable`, the last absent inline when empty and its count absent at zero in both forms |
| `derive-routing-candidates` | `candidates` |
| `traverse-deps` | `dependencies`, `cycles` |

`render-status` is bounded in a shape of its own, given under *`render-status`* above: its view stays inline, cut, and its file holds the whole view, with no count.

The counts are each list's `{list}-count`. The prune primitives flatten a summary or a walk into their result, so `form` and `lists-file` sit inside the half that moves lists: `prune-tasks`' summary, and the walk both share. A one-feature result is still its summary on the wire. `prune-plan`'s one-feature summary does not grow with the corpus and carries neither.
