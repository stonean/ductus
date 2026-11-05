# prune — reference

The reset body and the design-record heading set are compiled into `prune-tasks` and `prune-plan` from the framework templates, so a project that has customized its own tasks or plan template diverges there, and only there.

**Where a named section lives.** A section named below with no heading here is in the command file, `prune.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

**`tasks.md`.** `prune-tasks` classifies each task section **spent** (≥ 1 checkbox, all checked), **pending** (any unchecked), or **no-checkbox** (zero checkboxes), counting only real task-list checkboxes (a `- **Done when**:` line is not one). **keep-pending** (default) drops every spent section and a phase container left governing none; **reset** (`--reset`) keeps the file's `# …` heading and replaces everything below it with the template's initial tasks body, refused unless the spec status is `done` or `--force` is supplied.

**`plan.md`.** `prune-plan`'s description states the design record.

- **Decided** — a section is kept without being proposed when `analysis.md`'s `decisions:` list stores a `waived` decision under `plan-record — plan.md §{heading} is outside the design record`, or one the host matches to it by judgment.
- **Reopen** — on a `done` spec the spec reopens when `tasks.md` holds an unchecked checkbox line more times than HEAD's does, so a line moved or re-indented adds nothing. It is compared once before the first write, for the preview, and once more after the run's writes to the spec, which decides the reopen (step 9). Anything else leaves the spec `done`, a `plan.md` edit included, whatever it corrects (§spec-lifecycle).

**`--project`.** `prune-project`'s description states what it segments and returns; judge and confirm each entry as the command's **Project files (`--project`)** says.

Confirm with the user before every write, and leave no backup: recovery is git history.
