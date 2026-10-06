# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.
<<<<<<< HEAD

## 129. The pi command candidate

- [x] Implement the behavior described in `scenarios/the-pi-command-candidate.md`
- [x] Add {cli-config-dir}/prompts/{project}-{command_name}.md as the last candidate of Host::command_file_candidates (064), keeping the plural-then-singular order of the two pre-existing shapes
- [x] Extend the command_file_candidates_cover_both_layouts_plural_first test to the third element and add the .pi session-fixture test (the flat project-hyphenated form resolves last)
- [x] Sync the config-dir enumerations in the write-session doc comment and the check-corpus-links generated-copies comment to include .pi
- [x] Record the three-shape resolution order in 022's data-model.md under Per-project file resolution

- **Done when**: the third candidate resolves last with the pre-existing two in order, both pinned by tests, and the enumerations and data-model carry the shape. The release that ships it is 064's task 13, cut only after 022 returns to done.

## 130. Extend check-command-flags with the argument-hint token direction

- [x] Implement the behavior described in `scenarios/argument-hint-needs-a-token.md`

- **Done when**: check-command-flags reports a declared argument-hint whose body holds no substitution token; target/link/prune carry the token; and Family 30 is green with the new denominator on stderr.

## 131. Implement scenario: [every-agents-generated-copies-are-excluded](scenarios/every-agents-generated-copies-are-excluded.md)

- [x] Implement the behavior described in `scenarios/every-agents-generated-copies-are-excluded.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 132. Implement scenario: [a-multi-number-renumber-is-one-rewrite](scenarios/a-multi-number-renumber-is-one-rewrite.md)

- [x] Implement the behavior described in `scenarios/a-multi-number-renumber-is-one-rewrite.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.
=======
>>>>>>> 7a5da6ee112275a12643e6a045c97709f221aca5
