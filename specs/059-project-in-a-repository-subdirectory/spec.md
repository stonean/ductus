---
status: done
dependencies: [058-findings-route-at-discovery]
next-criterion: 12
cross-spec-impact: [018-adopter-owned-pre-commit]
---

# 059 — Project in a repository subdirectory

Every runtime reader that asks git about a project's files names them from the git work tree, and the pre-commit hook and the CI template run from the project root, so a project that sits in a subdirectory of its repository behaves as it would at the repository root.

## Motivation

A project need not sit at its repository's root. Git names every path in its trees, its index, and its diffs from the work tree, while a project names its specs from the project root; the two differ by the project's directory inside the repository. [058](../058-findings-route-at-discovery/spec.md) found two readers that did not convert, `compute-review-scope` and the inbox age, and fixed both through one helper, `workdir_prefix` (`runtime/src/primitives/mod.rs:1362`, called at `compute_review_scope.rs:144` and `inbox_standing.rs:187`).

058's third review found four more with the same defect, older than 058, and its task 46 routed them here. Each discovered the repository and then looked up a spec-root-relative path in its trees, so in a subdirectory project it found no history for the spec:

- `check-stuck` looked for the spec's `in-progress` commit under `{specs-root}/{feature}/spec.md` (`runtime/src/primitives/check_stuck.rs:41-45`). It found none, counted no commits, and never reported a spec stuck.
- `check-artifacts`' scenario-to-task family searched history for `{specs-root}/{feature}/tasks.md` to learn whether a `done` spec's scenario ever had a task (`runtime/src/primitives/check_artifacts.rs:419-431`). It found no revision, so a scenario whose task had been pruned was reported as never having had one: the finding that walk exists to suppress.
- `diff-cross-spec` and `derive-boundary` looked for the first commit touching `{specs-root}/{feature}/` (`runtime/src/primitives/diff_cross_spec.rs:55-60`, `runtime/src/primitives/derive_boundary.rs:45-57`). They found none and answered as for a spec never committed: cross-spec impact unknown, and a boundary of the spec directory alone.

Clarification examined every other git reader in the runtime and found the defect in six more, most of them silent:

- The exec payload's gitignore guard refuses an Affected Files path that git ignores, so a secret the project keeps out of git is never sent to the host (`runtime/src/interpreter/payload.rs:1122`, `:1458`). It hands `status_should_ignore` a project-relative path, where libgit2 takes one rooted at the work tree (`git_status_should_ignore`, `include/git2/status.h`). The project's own `.gitignore` never applies to the path it is asked about, so an ignored file under a subdirectory project passes the guard, and only the basename secret-pattern check still stands before the payload.
- `list_tracked_specs`, `list_untracked_specs`, and `list_staged_specs` open the project root as the repository (`runtime/src/primitives/mod.rs:2193`, `:2227`, `:2266`). `Repository::open` does not search upward (git2 0.21 documents `open` as "at `path`" and `discover` as "at or above `path`"), so in a subdirectory project each fails and takes its no-repository path. `derive-dependencies` and `derive-references` then enumerate specs by walking the worktree, so `--write` rewrites untracked drafts, which the tracked-specs rule of spec `017-derive-dont-ask` forbids; `untracked-skipped` is always empty; and `--staged` rewrites nothing.
- The rename exemption on review and analyze freshness looks project-relative paths up in a diff index keyed from the work tree (`runtime/src/primitives/analyze_subjects.rs:303`, `runtime/src/primitives/mechanical_sweep.rs:178`). Every lookup misses, and a missing path counts as a real change, so after a uniform repo-wide rename both records read as stale. The error is in the safe direction, but the exemption never runs.
- `check-corpus-links --scope repository` opens the project root as the repository (`runtime/src/primitives/check_corpus_links.rs:133`). It fails loudly, but with the wrong reason: `the git index could not be read`.

Two surfaces outside the runtime share the premise that the project root is the repository root:

- `/ductus` wires the hook with `git config core.hooksPath .githooks` (`framework/bootstrap/ductus-procedure.md` §Hook Installation). Git resolves a relative `core.hooksPath` from the root of the working tree (git-config(1) `core.hooksPath`; githooks(5) DESCRIPTION), so in a subdirectory project git never finds the project's hook, and none of its passes run. The hook also changes to `git rev-parse --show-toplevel` before running them (`framework/bootstrap/hooks/ductus-pre-commit:47-48`), where neither the runtime pointer nor the spec root is found. Audit Family 22 observes the resolution: a real commit with `core.hooksPath` set to the project's hooks directory, named from the work tree, runs the project's hook, and against the pre-059 hooks that hook then failed to find the inner hook at the work tree's root.
- The adopter CI template calls `.ductus/bin/ductus` from the checkout root (`framework/templates/ci/adopter-generators.yml`). In a subdirectory project the step fails loudly, and the template gives the adopter no hint of the fix. Adopters copy it by hand (spec `043-workflows-sunset`).

The prose-claim sweep during implementation found the same defect in two command sources, which tell a host to parse paths out of git output that names them from the work tree's root. `implement.md`'s markdown-only cross-spec check reads `git diff --stat`, and `amend.md`'s reconcile pass reads `git status --porcelain`, which names every path from the repository root whatever the working directory (git-status(1), porcelain v1). In a subdirectory project each reads the feature's own files as `proj/specs/…` and matches none of them. By operator decision these join 059.

## Edge Cases

- **A project at the repository root.** Its path from the work tree is empty, and every reader, the hook, and the CI template behave exactly as they do today.
- **No work tree, or a project outside it.** Where the project's path from the work tree cannot be resolved (`workdir_prefix` returns none), each reader takes the path it takes today with no repository, and reports what it could not examine as it does there. No new failure mode is introduced.
- **Paths outside the project.** An index entry, a diff path, or a tracked file outside the project directory is dropped, never reported as a `../` path, as `compute-review-scope` does.
- **A project reached through a symlink.** `workdir_prefix` canonicalizes both the work tree and the project root (`runtime/src/primitives/mod.rs:1363-1365`), so a symlinked project resolves the same path from the work tree as its target.
- **A project inside a submodule.** Discovery finds the innermost repository (libgit2's `git_repository_discover`: "the lookup ends when the first repository is found"), whose work tree holds the project, and the hook's work-tree root is that repository's. Nothing is resolved against the superproject.
- **More than one ductus project in one repository.** A repository has one `core.hooksPath`. The second project's `/ductus` finds it pointing at the first project's hooks directory and takes the existing custom-hooks-directory branch: it skips wiring and warns with the manual integration snippet, which names the project's hook by its path from the work tree.
- **A repository that already sets `core.hooksPath`, or uses husky, lefthook, or pre-commit.** The existing detection branches apply, with the value `/ductus` compares against and writes named from the work tree. A third-party hook system's markers are looked for at the work-tree root as well as the project root: a subdirectory project cannot see the root's `.pre-commit-config.yaml` from where it stands, and `pre-commit` installs without setting `core.hooksPath` (its `install` writes into the git directory's `hooks/` through `_hook_paths` and never sets the key, `pre_commit/commands/install_uninstall.py`), so nothing else would warn before `/ductus` overrode it.
- **A subdirectory project wired before 059.** Its `core.hooksPath` is `.githooks`, which a pre-059 `/ductus` wrote and which points at nothing at the work-tree root. When the work-tree root has no `.githooks` directory, `/ductus` rewires it to the project's hooks directory. When it has one, the value is someone else's, and the custom-hooks-directory branch applies.
- **An outer hook created before 059.** The adopter-owned outer hook still invokes the inner hook from the work-tree root, where a subdirectory project's inner hook is not. `/ductus` does not wire the hook while it does, and warns with the line to replace: the file is the adopter's, and wiring it as it stands would fail every commit.

## Acceptance Criteria

- [x] AC1: `check-stuck` finds a subdirectory project's `in-progress` commit and counts the commits touching its `tasks.md` since then, as it does at the repository root
- [x] AC2: `check-artifacts` finds a pruned task in a subdirectory project's `tasks.md` history and raises no scenario-to-task finding for its scenario
- [x] AC3: `derive-boundary` finds a subdirectory project's first spec-directory commit and `diff-cross-spec` its window's start, and each reports every path project-relative, dropping paths outside the project, as `compute-review-scope` does
- [x] AC4: Each runtime git reader this spec names converts between project paths and work-tree paths through `workdir_prefix`, keeps its existing behavior when the project is the repository root, and has a test against a project in a subdirectory of its repository
- [x] AC5: The exec payload's gitignore guard asks git about each Affected Files path named from the work tree, so in a subdirectory project a file the project's own `.gitignore` ignores is refused, and a pattern that does not match the path's location in the work tree does not refuse it
- [x] AC6: In a subdirectory project, `derive-dependencies` and `derive-references` enumerate only the specs tracked in the git index, never an untracked draft; report untracked specs in `untracked-skipped`; and with `--staged` rewrite the specs staged in the pending commit, as they do at the repository root
- [x] AC7: The review and analyze freshness checks apply the rename exemption to a subdirectory project's changed subjects, so a uniform repo-wide rename leaves both records current, as it does at the repository root
- [x] AC8: `check-corpus-links --scope repository` in a subdirectory project examines the tracked markdown files under the project root, names each one project-relative, and leaves out files outside the project
- [x] AC9: In a subdirectory project, `/ductus` sets `core.hooksPath` to the project's `.githooks` directory named from the work tree, recognizes that value as already wired on a later run, and `framework/bootstrap/hooks/ductus-pre-commit` runs its passes from the project root rather than from `git rev-parse --show-toplevel`
- [x] AC10: `framework/templates/ci/adopter-generators.yml` carries a commented `defaults.run.working-directory` that a project in a subdirectory sets to its path, so every step runs from the project root
- [x] AC11: Every git instruction in `framework/commands/` whose output a host parses for paths names them from the project root: **superseded in part:** `implement.md`'s markdown-only cross-spec check, which ran `git diff --name-only --relative`, is gone, since `diff-cross-spec` owns that filter. `amend.md`'s reconcile pass parses `git status --porcelain`, whose shape no user setting changes, and names each path from the project root by dropping whatever precedes the feature's directory

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Do the other git readers share the defect?** Yes, six of them, and two surfaces outside the runtime share the premise behind it; §Motivation lists each with its citation. Grounded by reading every `Repository::discover` and `Repository::open` caller under `runtime/src` (no reader shells out to `git`), the libgit2 contract for `git_status_should_ignore` (a path "rooted at the repo's workdir"), and git-config(1) and githooks(5) for how a relative `core.hooksPath` resolves. The hook finding rested on that documentation at clarify time; audit Family 22 has since observed it with a real commit (`scripts/audit/adopter-shell-behavior.sh`, the subdirectory-project case). By operator decision (2026-09-26), 059 takes all of it: the six readers, the hook and its wiring, and the CI template. The alternatives were to route the hook to `018-adopter-owned-pre-commit` as a scenario, reopening it, or to discard it. One spec keeps one concern, and fixing `list_staged_specs` changes nothing until the hook runs from the project root, so the two are one fix. The hook fix falsifies `018-adopter-owned-pre-commit`'s body, which states the hook's toplevel `cd` and the `core.hooksPath .githooks` wiring branches, so 059 declares that impact in `cross-spec-impact:` and owes 018 a correction with a back-link here. 018 reopens once, for that edit, when the hook fix lands.
