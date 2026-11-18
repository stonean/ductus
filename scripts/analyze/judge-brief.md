# Rule-judging brief — /ductus:analyze steps 11–12 for {{FEATURE}}, cell {{CELL}}

You judge the rules of share {{SHARE}} against spec `{{FEATURE}}` (status `{{STATUS}}`) in the repository at `{{REPO}}`, HEAD `{{HEAD}}`. Work in your own foreground: start no sub-agents. Edit no repository file. Write only `{{OUT}}`, a new file.

## 1. What a judgment is

Before judging, read these whole. They define a rule's trigger, its tier, a load-conditioned yield, a departure's waiver and a citation; this brief adds only the run's facts.

- Steps 11 and 12 of `framework/commands/analyze.md`. Each step is one long line, so read them with `sed -n '/^11\. /,/^13\. /p' framework/commands/analyze.md | fold -w 1000`.
- `framework/reference/analyze.md` §Rule judging in a subagent and §Stored rule verdicts.

## 2. Read the design subjects, each whole

Read each file whole with the Read tool. When a read is cut short (a line cap, truncation, a saved-output pointer), read the remainder with an offset. Run `wc -c` on each and report the bytes you read against its size.

{{SUBJECTS}}

{{LONG}}

## 3. Read your rule share, whole

`{{SHARE_FILE}}` ({{SHARE_BYTES}} bytes) holds each rule's ID, tier, Statement and whole Verification, from `discover-rule-files`. A finding counts in its rule's tier as the share gives it. Each file's other-heading lines list the headings that open no rule. Report under `malformed-headings` any that looks like a rule heading the parser rejected.

{{SHARE_LONG}}

## 4. This run

- **Loaded rule files**, for a yield "where `{file}` is also loaded": {{LOADED}}.
- **Departures:** {{DEPARTURES}}
- Judge from what the subjects say. Do not read source code to decide a rule.

## 5. Return

Answer every ID in your share. Each goes in `judged`, fired or not, or in `unjudged` with its reason. Write this JSON to `{{OUT}}`:

```json
{
  "cell": "{{CELL}}",
  "judged": ["EVERY-ID-YOU-ANSWERED"],
  "verdicts": {
    "RULE-ID": {"verdict": "addressed", "cite": "plan.md#Exact Heading"},
    "RULE-ID-2": {"verdict": "finding", "at": "spec.md#Exact Heading", "message": "one line"},
    "RULE-ID-3": {"verdict": "waived"}
  },
  "unjudged": [{"id": "RULE-ID", "reason": "why it could not be judged"}],
  "malformed-headings": [],
  "bytes-read": {"specs/{{FEATURE}}/spec.md": [0, 0]}
}
```

- `verdicts` holds only the fired rules.
- A `cite` or `at` is `<path>#<Heading>`. The path is relative to the feature directory, and the heading is a heading's exact, whole text without its `#` marks.
- `bytes-read` holds every design subject and the share file, keyed by repo-relative path. Each entry is `[bytes read, wc -c size]`.

End your final message with:

- the counts: judged, addressed, finding, waived and unjudged;
- each finding, one line each;
- the bytes-read account;
- the line `END OF REPORT`.
