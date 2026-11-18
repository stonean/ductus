# Step-15 brief — /ductus:analyze for {{FEATURE}}

You judge step 15 of `/ductus:analyze` for spec `{{FEATURE}}` (status `{{STATUS}}`) in the repository at `{{REPO}}`, HEAD `{{HEAD}}`. Work in your own foreground: start no sub-agents. Edit no repository file. Write only `{{OUT}}`, a new file.

## 1. What the step judges

Read these whole before judging:

- Step 15 of `framework/commands/analyze.md`. It is one long line: `sed -n '/^15\. /,/^16\. /p' framework/commands/analyze.md | fold -w 1000`.
- These sections of `framework/reference/analyze.md`, which the step points at:
  - §Grounding (advisory)
  - §Decision reasons (advisory)
  - §Spec integrity (blocking)
  - §Plan consistency (blocking if plan exists)

## 2. Read the subjects, each whole

Read each one whole with the Read tool, including the remainder of any read cut short. Run `wc -c` on each and report the bytes you read against its size.

{{SUBJECTS}}

{{LONG}}

## 3. Return

Write this JSON to `{{OUT}}`:

```json
{
  "grounding": [{"path": "spec.md", "message": "one claim, naming its section"}],
  "integrity": [{"check": "code-block|rationale|system", "path": "plan.md", "message": "one line"}],
  "integrity-checked": {"code-block": true, "rationale": true, "system": true},
  "decision-reasons": [{"path": "plan.md", "message": "one line"}],
  "bytes-read": {"specs/{{FEATURE}}/spec.md": [0, 0]}
}
```

- Report each grounding claim on its own, never two in one entry.
- `integrity-checked` says which of the three integrity items you judged. Set an item to `false` when its subject could not be read.
- `bytes-read` holds every subject, keyed by repo-relative path. Each entry is `[bytes read, wc -c size]`.

End your final message with:

- the counts;
- each finding, one line each;
- the bytes-read account;
- the line `END OF REPORT`.
