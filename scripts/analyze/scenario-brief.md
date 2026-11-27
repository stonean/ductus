# Scenario brief — /ductus:analyze for {{FEATURE}}, {{READER}}

You judge each scenario below for spec `{{FEATURE}}` (status `{{STATUS}}`) in the repository at `{{REPO}}`, HEAD `{{HEAD}}`. Work in your own foreground: start no sub-agents. Edit no repository file. Write only `{{OUT}}`, a new file.

## 1. What the judgment is

Read these whole before judging:

- `framework/reference/analyze.md` §Scenario consistency (advisory). Its first item is this judgment, which step 8 of `framework/commands/analyze.md` leaves to the host. Its other items are `check-artifacts`' and not yours.
- The paragraph under `### Scenarios` in `framework/constitution.md`, which says what a scenario's sections hold.

## 2. Read the scenarios, each whole

Read each one whole with the Read tool, including the remainder of any read cut short. Run `wc -c` on each and report the bytes you read against its size.

{{SUBJECTS}}

{{LONG}}

## 3. Return

Write this JSON to `{{OUT}}`:

```json
{
  "reader": "{{READER}}",
  "judged": ["scenarios/every-scenario-you-judged.md"],
  "findings": [{"path": "scenarios/one.md", "check": "context|behavior", "message": "one line"}],
  "bytes-read": {"specs/{{FEATURE}}/scenarios/one.md": [0, 0]}
}
```

- `judged` names every scenario §2 lists, by its path relative to the feature directory. `check.py` fails a scenario left out, since nothing else in the run judges it.
- `check` is the section a finding is about: `context` or `behavior`. Give at most one finding for each scenario and section.
- `bytes-read` holds every scenario, keyed by repo-relative path. Each entry is `[bytes read, wc -c size]`.

End your final message with:

- the counts: judged and findings;
- each finding, one line each;
- the bytes-read account;
- the line `END OF REPORT`.
