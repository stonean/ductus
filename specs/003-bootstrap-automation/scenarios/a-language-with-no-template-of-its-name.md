---
section: "Scaffolding Steps"
---

# A-language-with-no-template-of-its-name

## Context

`/ductus` asks for the project's primary languages and appends each one's patterns to `.gitignore`, fetched from `https://raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore` (`framework/bootstrap/ductus.md`, the `.gitignore` merge, step 5; this spec's AC13). GitHub names its templates by ecosystem, not always by language: it has no `TypeScript.gitignore` (404, fetched 2026-11-02), and its `Node.gitignore` is the template a TypeScript or JavaScript project needs.

A real adopter bootstrap for 077's task 10, giving the language TypeScript, fetched that 404 at `ductus-v0.59.11` and on `main` alike, and the subject got no language patterns. The step states no outcome for a fetch that fails, so a host may halt there or carry on without saying so, and AC13's claim that patterns are appended for each primary language is untrue for such a language (077 task 10, owner, 2026-11-02).

## Behavior

A language with no github/gitignore template of its name neither halts the run nor passes in silence. The bootstrap keeps a map from such a language to the template that covers it — TypeScript and JavaScript to `Node` — holding only names a real bootstrap run has fetched and confirmed; a mapped language is fetched under its mapped name. The run's closing summary names every language whose patterns were not appended, with the reason.

## Edge Cases

- **Two languages that resolve to one template**, TypeScript and JavaScript both to `Node`, append it once.
- **An unmapped language whose template does not exist** (a 404) is named in the closing summary as having no template of its name, so the operator can name GitHub's template on the next run.
- **A fetch that fails for another reason** (no network, a server error) is reported apart from a template that does not exist, since the repairs differ: retry, or name another language.
- **A re-run** leaves an existing `# {Language}` section alone, as step 5 already does, so a language resolved on a later run adds its section then.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does a language with no template of its name map to the one that covers it, or is it reported and skipped?** Both: a map in the bootstrap sends TypeScript and JavaScript to `Node`, holding only names a real bootstrap run has fetched and confirmed, and any unmapped language whose fetch fails is reported by name with its reason (owner, 2026-11-12). No artifact in this repository records github/gitignore's template names, and none can be fetched to check here, so a map entry is admitted only on a real run's evidence; the report covers every language the map does not. Rejected: reporting and skipping alone, which leaves the common TypeScript case without patterns until the operator acts; and a map with no report, which leaves an unmapped 404 as silent as it is today.
