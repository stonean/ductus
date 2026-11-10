# 062 — Concurrent session targets Data Model

## Files

| Path | Holds | Written by |
| --- | --- | --- |
| `.ductus/session.toml` | The shared default: the most recent target change in the working tree, plus the per-contributor `cli-config-dir` | Every target or clear write; `/ductus` host-config writes; `retarget-sessions` |
| `.ductus/sessions/{key}.toml` | One identified process's own target and bookkeeping | That process's resolutions and writes; `retarget-sessions`; removed by the expiry sweep |
| `.ductus/sessions/.lock` | Nothing; the advisory lock's handle | Created on first use, never deleted |
| `.ductus/sessions/.gitignore` | `*` | The first process to hold the session lock, when absent (`scenarios/first-writers-create-the-ignore-file-once.md`) |

The shared default's keys and field order are unchanged
(`runtime/src/primitives/write_session.rs:219-235`): `feature`, `path`,
`scenario`, `scenario-path`, `set-at`, `cli-config-dir`, all optional.

## Sessions on different specs

The ordinary case. Each identified process's target lives in its own file,
so two sessions working on different specs at the same time hold two
independent targets. A target write in one never changes the other's file,
and each resolves only its own (AC1, AC2):

```text
.ductus/sessions/review.toml               feature = "055-example"
.ductus/sessions/9b1c07e2-….toml           feature = "061-release-tags"
.ductus/session.toml                       feature = "061-release-tags"  # the later write
```

The shared default only records the most recent target change. A process
reads it to adopt it when it has no target of its own: on its first
resolution, and again once the expiry sweep has removed its record. In
between, only its own writes, or a fold or consolidation that removes the spec
it targets, change its target. `peers`, `seen-peers` and the co-target
notice below exist for the other case, where two sessions land on the same
feature.

## Per-process record: `.ductus/sessions/{key}.toml`

`{key}` is the sanitized identity: the `DUCTUS_SESSION` value or the platform
session identifier, passed through the slug rule.

```toml
source = "claude-code"            # "named" | a platform source; for labels
feature = "055-example"           # absent when cleared
path = "specs/055-example"        # absent when cleared
scenario = "edge-case"            # optional, with scenario-path
scenario-path = "specs/055-example/scenarios/edge-case.md"
set-at = "2026-09-29T14:00:00Z"   # this process's last target write, or the default's stamp when adopted; a retarget keeps it
used-at = "2026-09-29T15:30:00Z"  # last resolution or write; drives expiry
cleared = true                    # present only when cleared; no target keys then
seen-peers = ["review", "claude-code:3f2a9c1d"]  # labels last reported

[notice]                          # pending removal notice; absent when none
cause = "fold"                    # "fold" | "consolidate"
from = "1234.1-retry-budget"
to = "055-example"                # absent for consolidate
by = "review"                     # label of the session whose command removed it
```

Notes:

- `used-at` is refreshed on every resolution and write, under the lock. The
  sweep removes a file whose `used-at` is more than seven days old, and it
  never removes the caller's own file.
- `seen-peers` is the peer set this session was last told about. A resolution
  that computes a different set replaces it, with a co-target notice unless
  the new set is empty, which leaves nobody to name.
- `[notice]` is delivered and removed by the owner's next resolution. A second
  removal before delivery replaces it: only the latest fate of the target is
  true.
- A file that fails to parse is an operational error for its owner (AC22), and
  it is reported but never deleted by another process's sweep.

## `resolve-session`

Arguments: none. The identity comes from the runtime process's environment.

Result:

```json
{
  "identity": "review",
  "source": "own",
  "target": {
    "feature": "055-example",
    "path": "specs/055-example",
    "scenario": null,
    "scenario-path": null
  },
  "notices": [
    { "kind": "co-target", "message": "Session claude-code:3f2a9c1d (last used 2026-09-29T15:02:00Z) also targets 055-example." }
  ]
}
```

- `identity`: the display label, or `null` when unidentified.
- `source`: `own`, `adopted`, `default` (unidentified), `cleared`, or `none`.
- `target`: `null` for `cleared` and `none`.
- `notices[].kind`: `adopted`, `co-target`, `folded`, or `consolidated`.
  `message` is the rendered sentence the host displays.
- `unreadable`: repo-relative session files that could not be parsed while
  checking for other sessions on the same feature, named rather than read as
  "none" (AC22); omitted when empty. The dashboard carries the same list as
  `session-unreadable`.

## `write-session` result additions

The existing `path` (the default's repo-relative path) and `created` fields are
unchanged. Added:

- `identity`: the display label; absent when unidentified, so an unidentified
  write's result keeps its pre-062 shape (AC3).
- `own-path`: the per-process file written; absent when unidentified.
- `peers`: `[{ "session", "feature", "scenario", "last-used" }]`, the other
  unexpired sessions targeting the same feature. `scenario` is `null` when the
  peer targets none. `last-used` is the peer record's `used-at`, an RFC 3339
  date-time in UTC with the `Z` offset, to the second, the form `set-at` has
  (`2026-11-10T01:32:46Z`), or `null` when that record holds none. Empty on
  clear and host-config writes, and on an unidentified write.
- `expired`: labels of the per-process targets the sweep removed.
- `unreadable`: repo-relative paths the write could not read — a file that
  does not parse, or whose `used-at` the sweep cannot read: other sessions'
  files, which the sweep leaves in place, and the process's own record or the
  shared default when either did not parse, which this write replaces (AC22).
  A replaced default's `cli-config-dir` is not carried forward.
- `notices`: a pending removal notice this write replaced along with the
  process's own record, delivered here so that no write, whichever command
  makes it, discards one undelivered (AC21).

The four lists are omitted when empty, like the two fields above when
absent.

## `retarget-sessions`

Arguments:

| Key | Required | Meaning |
| --- | --- | --- |
| `from` | yes | The removed feature's directory name |
| `feature`, `path` | with a re-target | The new target |
| `scenario`, `scenario-path` | optional, together | The new target's scenario |
| `clear` | instead of a new target | Clear rather than re-target |
| `cause` | yes | `fold` or `consolidate` |

Supplying both `clear` and a new target is an `InvalidArgument`; supplying
neither is a `MissingArgument` naming the argument the cause still needs.
Paths are checked with the existing `validate_no_traversal`, an `InvalidPath`.
Every name a session stores is rendered into other sessions' notices and
dashboard lines, so names are held to an allowlist (`BE-INPUT-002`): `from` and
`feature` must be feature directory names (`parse_feature_dir`) of visible
ASCII with no path separator, and `scenario` a scenario slug; anything else is
an `InvalidArgument` naming the argument, checked before any session is
touched. The only names this refuses that the corpus grammar admits are
legacy sequential directories whose slug holds a character outside visible
ASCII — whitespace, a control or non-ASCII character — or a backslash: no
session can target one until it is renamed. `write-session` holds its `feature` and `scenario` to the same
rule.

Result:

- `retargeted`: labels of the sessions re-pointed, with `default` for the
  shared default.
- `cleared`: labels of the sessions cleared, with `default` for the shared
  default.
- `unreadable`: paths that could not be parsed. These are left in place and
  reported, because a removal cannot prove they do not name `from`.
