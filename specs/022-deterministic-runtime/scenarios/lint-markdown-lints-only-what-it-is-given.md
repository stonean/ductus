---
section: "The primitive library"
---

# Lint-markdown-lints-only-what-it-is-given

## Context

`lint-markdown` passes its paths to `markdownlint-cli2`, which appends the config's `globs` to the command-line arguments unless `--no-globs` is passed. The shipped `.markdownlint-cli2.jsonc` sets `globs` to `**/*.md`, so every call lints the whole repository (verified 2026-10-26 at HEAD c0952038): here `markdownlint-cli2 AGENTS.md` lints 742 files, and with `--no-globs`, 1. Scratch repro: with `a.md` clean and `b.md` carrying trailing spaces, `ductus lint-markdown --path a.md` reports `b.md`'s MD009. `check-review-gate` calls the same primitive with the feature's markdown glob, so (read from source) a violation anywhere in the repository blocks a feature's completion as "markdownlint violation(s) in {feature dir}", and the config ships to adopters with strategy update.

## Behavior

- Given paths or globs, `lint-markdown` lints exactly the files they name and reports violations only in those files.
- Given no paths, it lints what the config's `globs` name, as today.
- `check-review-gate`'s lint check reports only violations inside the feature directory, so a violation elsewhere in the repository never blocks a feature's completion.

## Edge Cases

- The config's `ignores` still apply to the given paths.
- A given glob matching no file: reported as nothing examined, not as clean.
- An adopter config without `globs`: behavior unchanged.
- The markdown-only path: the host lints the named files the same way, so the two paths agree.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
