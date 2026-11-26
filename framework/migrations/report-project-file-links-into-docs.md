# report-project-file-links-into-docs

**Introduced in:** ductus 0.65.0
**Summary:** Report each project-file link to a markdown file under `docs/`, naming `/{project}:prune --project`, which can move it into a parts directory under `.ductus/docs/`; move and rewrite nothing (spec 087).

## Background

Since ductus 0.65.0 a project file — `AGENTS.md`, and `system.md`, `events.md` and `errors.md` under the configured spec root — may keep an index and move sections into **parts**, the markdown files directly under `.ductus/docs/agents/`, `system/`, `events/` and `errors/`, which every command reads, measures and prunes as the file. A procedure moved out of `AGENTS.md` used to go to `docs/`, where nothing measured it and every edit counted as code. Which file under `docs/` is a part and which is documentation for people is a judgment, so this entry moves nothing: it reports each link and names the command whose walk judges each target, shows the search for every file naming it, and writes only what the operator confirms.

## Procedure

1. **Preview.** Run `.ductus/bin/ductus prune-project` from the project root, with no argument (`.ductus/bin/ductus.exe` on Windows). With no argument it is a preview: it writes nothing in the project but, past `[artifacts] read-size-bytes`, its lists file under the gitignored `.ductus/results/`. Run it through the CLI even when the `ductus` server is live in this session, never as the MCP tool: the server keeps the binary it started with, which can predate the one Pre-flight brought the pointer to.

2. **Not checked.** When the call exits non-zero, or prints no JSON object, report `report-project-file-links-into-docs: not checked — .ductus/bin/ductus prune-project failed: {its error}` and stop. When the result carries no `move-candidates-count`, the binary predates the list: report `report-project-file-links-into-docs: not checked — .ductus/bin/ductus prune-project lists no move candidates, so the binary predates ductus 0.65.0; run /{project}:prune --project once the runtime is current` and stop. Never report either as no link.

3. **Read the candidates.** Read `move-candidates` from the result, or from the file `lists-file` names when `form` is `file`. Each entry names a project-relative `path` and `linked-from`, each project file or part linking it. Keep each entry whose `path` begins `docs/`.

4. **Report.** For each kept entry, report `report-project-file-links-into-docs: {linked-from, joined by ", "} links {path} — /{project}:prune --project proposes moving it under .ductus/docs/ when it is a procedure an agent reads or content of the file linking it, and re-points each referrer you confirm; documentation written for people stays in docs/`. With no kept entry, report nothing. Write nothing: no project file, part, referrer or configuration.

## Verification

- A project whose `AGENTS.md` links `docs/x.md` gets one line naming `AGENTS.md`, `docs/x.md` and `/{project}:prune --project`, and the procedure leaves its tracked and untracked files as they were.
- A project with no project-file link to a markdown file under `docs/` gets no line.
- A pointer binary whose `prune-project` result carries no `move-candidates-count` gets the not-checked line, never no line.
