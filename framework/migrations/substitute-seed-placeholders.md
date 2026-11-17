# substitute-seed-placeholders

**Introduced in:** ductus 0.60.0
**Summary:** Fill the three substitution keys an earlier bootstrap left literal in `AGENTS.md` and `CLAUDE.md` — `{project-name}`, `{One-line project description.}` and `{project}` — with the values this run substitutes into new copies.

## Background

Through ductus 0.59.11, bootstrap step 4 mapped the `skip` rows for `AGENTS.md` and `CLAUDE.md` to `apply-manifest`'s `skip-if-conflict` strategy, which substitutes nothing, so an adoption on the runtime path wrote both files with those placeholders literal. Since `071-command-reference-split` task 20 (`b0b1f9d0`, 2026-10-26) both rows map to `create`, which substitutes, but that reaches new adoptions only: both files are installed once and never rewritten (027's scenario `seed-files-keep-no-literal-placeholder`).

The adopter owns both files, so the migration fills only the three keys step 4 substitutes into their templates. The 0.59.0 templates also carried `{name}`, `{slug}` and `{NNN-feature-name}` as documentation the bootstrap never substitutes; those and every other brace-delimited token are left alone.

## Procedure

The values are the ones this run's substitutions map holds for `project-name`, `One-line project description.` and `project` — the map step 4's Shared Files pass applies, which **Collect Project Inputs** settled before this section runs. Apply steps 1–5 to `AGENTS.md`, then to `CLAUDE.md`, each at the project root. A step that says *stop* ends the walk for that file only.

1. **Resolve the file.** When it does not exist, stop silently and report nothing.

2. **Find the tokens.** Count the occurrences of each of the three literal strings `{project-name}`, `{One-line project description.}` and `{project}`, matched exactly and case-sensitively. When none occurs, the file is converged: stop silently and report nothing.

3. **Skip a pinned file.** When the file is listed in `.ductus/config.toml` `[pinned] files`, report one line and stop without writing:

   `warning: {file} is pinned and still holds {tokens}; replace them by hand with the project's name, description and slash-command namespace.`

4. **Fill.** Replace every occurrence of each token whose value is not empty with that value, taken literally, `{project-name}` first. Leave a token whose value is empty as it is. Preserve every other byte, line endings included. Write atomically, keeping the file's mode: a temporary file in the same directory, then a rename.

5. **Report.** When step 4 filled any, `substitute-seed-placeholders: filled {N} placeholder(s) in {file}`; and for each token left in step 4, `substitute-seed-placeholders: {file} still holds {token} — this run has no value for it; replace it by hand`.

`{file}` is `AGENTS.md` or `CLAUDE.md`, as walked; `{tokens}` lists the tokens step 2 found, comma-separated.

## Verification

- After a fill, `git diff` of the file shows only the three tokens replaced by the run's values; `{name}`, `{slug}`, `{NNN-feature-name}` and every other brace token are unchanged.
- A pinned file is unchanged, and the run reported the line naming it.
- After a fill, a second run finds no token and reports nothing; a pinned file's report repeats on every run until it is edited.
- The file's line-ending style is unchanged.
