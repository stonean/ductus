# 059 — Project in a repository subdirectory Plan

Implements [059 — Project in a repository subdirectory](spec.md).

## Overview

Every git reader in the runtime opens its repository through one new helper, `ProjectRepository`, which discovers the repository from the project root and carries the project's path from the work tree. Each reader converts project paths to git paths on the way in (`to_git`) and back on the way out (`to_project`, which drops a path outside the project). The helper is built on 058's `workdir_prefix`, so the conversion rule lives in one place.

Outside the runtime, the adopter hook finds its project root from its own location rather than from `git rev-parse --show-toplevel`. `/ductus` wires `core.hooksPath` to the hooks directory named from the work tree. The CI template names the working directory a subdirectory project sets. Spec 018, whose body states the old wiring, is corrected with a back-link here, which discharges `cross-spec-impact:`.

The runtime changes ship in a patch release, `0.54.1`. The hook, procedure, and template changes reach adopters from `main`.

## Technical Decisions

### One helper opens a project's repository, and every reader uses it

The runtime has twelve production call sites that open a repository. Measured by `grep -rn "Repository::discover\|Repository::open"` over `runtime/src`, excluding test modules, they are in `interpreter/payload.rs`, `mod.rs` (three), and `check_stuck`, `check_artifacts`, `diff_cross_spec`, `derive_boundary`, `analyze_subjects`, `check_corpus_links`, `compute_review_scope`, and `inbox_standing`. No reader shells out to `git`. Ten of the twelve are this spec's. `compute_review_scope` and `inbox_standing` are 058's fixes.

Each reader today repeats one of two mistakes. The `discover` sites join a project-relative path into a lookup that git keys from the work tree. The `open` sites (`mod.rs:2193`, `:2227`, `:2266`, `check_corpus_links.rs:133`) do not search upward at all (git2 0.21 documents `open` as "at `path`" and `discover` as "at or above `path`"). The fix is one type in `runtime/src/primitives/mod.rs`, beside `workdir_prefix` (`mod.rs:1362`):

```rust
/// A repository opened for a project, with the project's path from its work tree.
pub(crate) struct ProjectRepository {
    pub(crate) repository: git2::Repository,
    /// `/`-joined, empty at the work tree's root, else ending in `/`.
    prefix: String,
}

impl ProjectRepository {
    /// Discover the repository containing `project`. Errs when there is none, and
    /// when `workdir_prefix` cannot place the project inside its work tree.
    pub(crate) fn discover(project: &Path) -> Result<Self, git2::Error>;
    /// A project-relative, `/`-separated path as git names it.
    pub(crate) fn to_git(&self, rel: &str) -> String;
    /// A git path as the project names it, or `None` when it lies outside the project.
    pub(crate) fn to_project<'a>(&self, git_path: &'a str) -> Option<&'a str>;
}
```

The prefix is built exactly as `compute_review_scope`'s private `history_prefix` builds it (`compute_review_scope.rs:138-149`), component by component with `/`, so it is right on every platform. `history_prefix` is deleted and `compute_review_scope` uses the helper.

### An unresolvable project path is the no-repository case, not the root

`workdir_prefix` returns `None` when the repository has no work tree, the project cannot be canonicalized, or the project lies outside the work tree (`mod.rs:1353-1356`). After a successful discovery that happens only for a bare repository or under a `core.worktree` override; discovery does not read `GIT_DIR` or `GIT_WORK_TREE`, which libgit2 consults only when a repository is opened from the environment (`repository.c`, the `use_env` branch). `history_prefix` reads it as an empty prefix ("reads history from the root as before", `compute_review_scope.rs:140-141`), which silently finds no history. The spec's edge case chooses the no-repository path instead. `discover` returns an error in that case, so each reader takes the branch it already takes with no repository. Every one of those branches is already distinguishable from a clean result:

| Reader | No-repository branch today |
| --- | --- |
| `check-stuck`, `diff-cross-spec`, `derive-boundary`, `compute-review-scope` | `PrimitiveError::Git` through `?` |
| `check-artifacts` scenario-to-task walk | `None`, which suppresses every finding by design (`check_artifacts.rs:405-412`) |
| rename exemption (`exempt_renames`) | keeps every candidate, reporting stale (`analyze_subjects.rs:290-294`) |
| exec payload gitignore guard | no repository, so no gitignore refusal; the basename secret check and the containment check still run |
| `list_tracked_specs` | the worktree walk (`mod.rs:2205-2214`) |
| `list_untracked_specs`, `list_staged_specs` | empty |
| `check-corpus-links --scope repository` | `guidance`, never a clean verdict (`check_corpus_links.rs:77-91`) |

`compute_review_scope` changes behavior in that one case, from reading history at the root to an error. This is deliberate. The two callers would otherwise disagree about the same condition, and the old answer was the silent one.

### Each reader converts where it touches git

| Reader | In (`to_git`) | Out (`to_project`) |
| --- | --- | --- |
| `check_stuck.rs:41-45` | `spec_rel`, `tasks_rel` | — |
| `check_artifacts.rs:419-420` | the `tasks.md` history path | — |
| `diff_cross_spec.rs:55-106` | `spec_prefix` for `first_commit_for_prefix`, and the diff pathspec, matched literally | every diff path, before it is matched against the project-relative spec root, feature directory and inbox; a path outside the project is dropped |
| `derive_boundary.rs:45-95` | `spec_prefix` for `first_commit_for_prefix` and the own-directory skip | every changed path before `zone_glob`; a path outside the project is dropped |
| `analyze_subjects.rs:303-321` | each candidate before `changed_beyond_spelling` | — (candidates stay project-relative in the result) |
| `payload.rs:1122`, `:1132` | the canonical path of each Affected Files entry, named from the project root, before `status_should_ignore` — the file read, not the plan's spelling of it | — |
| `list_tracked_specs` (`mod.rs:2193`) | — | each index entry before `is_spec_path` |
| `list_untracked_specs` (`mod.rs:2227`) | the status pathspec, matched literally | each status entry before `is_spec_path` |
| `list_staged_specs` (`mod.rs:2266`) | the delta pathspec, matched literally | each delta path before `is_spec_path` |
| `collect_tracked_markdown` (`check_corpus_links.rs:132`) | — | each index entry before `repo.join`; a path outside the project is skipped |

A pathspec carrying the project's path from the work tree is matched literally (`disable_pathspec_match`): that path may hold `*`, `?` or `[`, which as a glob would match no file, and the empty result would read as clean. A found repository whose index or status cannot be read is an error for every spec listing, never the worktree walk or an empty list.

`find_in_progress_commit` and `first_commit_for_prefix` are unchanged. They already take a git path, and `compute_review_scope` already hands the first one a prefixed path (`compute_review_scope.rs:59-60`). `derive-boundary`'s spec glob and guidance stay project-relative, because the boundary is enforced against project paths.

A plan path that escapes the project (`../x`) is refused by the containment check, `classify_contained`, as `out-of-repo`, inside a git repository and outside one alike. That check runs before the gitignore query, and the order matters: libgit2 answers "ignored" for any path containing `..` (a probe at the work tree's root returned `should_ignore("../outside.txt") = Ok(true)`), so with the gitignore query first, as the layers stood before this spec, an escape inside a git repository was refused under the `.gitignore` label instead. The refusal held; only the label misled, and it contradicted 022's `writecode-payload-canonicalize-paths`, whose relative escape is rejected with `out-of-repo`. Task 16 moved containment ahead of the gitignore query: a missing path is skipped before git is asked, an outside one is refused as `out-of-repo`, and only a path inside the project is asked about gitignore before it is read. The secret-pattern check still runs first.

### `check-corpus-links` names the reason it could not run

With `discover`, the repository scope's `false` return means no repository contains the project, or its index could not be read. The guidance names both instead of the index alone (`check_corpus_links.rs:86-87`). Files outside the project are outside the subject rather than excluded from it, so they are not counted in `excluded_by_construction`. The spec-corpus scope that the adopter hook runs walks the filesystem and is unchanged.

### Tests build a repository with the project in a subdirectory

A `#[cfg(test)]` fixture in `mod.rs` builds a temporary repository with the project at `proj/`, writes `.ductus/config.toml` there, and commits through one shared `commit_all`. Each reader named in AC4 gets a test driven from `proj/`. The existing root-layout tests stay, and they are the regression half of AC4. Each new test is shown to fail with its conversion removed before it is trusted (§design-principles). The AC5 test asserts both directions:

- a file ignored by the project's own `.gitignore` with an anchored pattern is refused;
- a project file that a pattern anchored at the work-tree root would match if the path were read from the root is not refused.

The first case must not use a name the basename secret check already catches, or the test passes without the guard.

### The hook finds its project root from its own location

`framework/bootstrap/hooks/ductus-pre-commit:47-48` changes to its own directory's parent: `ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"`. That is the project root, because the manifest always places the file at `{project}/.githooks/ductus-pre-commit` (`framework/bootstrap/ductus.md:661`, §Shared Files, destinations named from the project root). Git runs hooks from the work-tree root (githooks(5)), so the toplevel says nothing about which project a hook belongs to, while the file's location does. At the repository root both answers are the same directory.

The staged-spec listing gains `--relative` (`ductus-pre-commit` `staged_specs=…`). `git diff --cached --name-only` names paths from the work-tree root whatever the working directory (git-diff(1) `--relative`: only that option shows pathnames relative to a subdirectory; audit Family 22's subdirectory case observes it), and the shape pattern accepts exactly one leading segment, so without `--relative` a subdirectory project's staged specs match nothing, and none are labelled or re-staged. `--relative` names them from the project root and drops other projects' paths. It changes nothing at the root.

The outer stub, `framework/bootstrap/hooks/pre-commit`, invokes the inner hook by its own location, `"$(dirname "${BASH_SOURCE[0]}")/ductus-pre-commit"`, rather than `./.githooks/ductus-pre-commit` from the toplevel. It keeps its `cd` to the toplevel, since that is where an adopter's own checks have always run.

### `/ductus` wires the hooks directory named from the work tree

`framework/bootstrap/ductus-procedure.md` §Hook Installation computes `P = git rev-parse --show-prefix` (empty at the root, else ending in `/`) and `H = {P}.githooks`, and runs the ladder against `H`:

1. `core.hooksPath` is `H` — already wired, as today.
2. **New.** `P` is non-empty, `core.hooksPath` is `.githooks`, and the work-tree root has no `.githooks` directory. That is the value a pre-059 `/ductus` wrote from the subdirectory, pointing at nothing, so it is rewired to `H` as in case 5.
3. `core.hooksPath` points at any other path — skip and warn, as today.
4. A third-party hook system is detected. The markers are now checked at the project root **and** the work-tree root. `pre-commit` (python) installs into `.git/hooks` without setting `core.hooksPath` (`_hook_paths` and `install` in `pre_commit/commands/install_uninstall.py`), so a marker at the work-tree root that the project root cannot see would otherwise be clobbered by setting it.
5. No conflicts — `git config core.hooksPath H`.

Cases 2 and 5 are guarded by one precondition. If `{project}/.githooks/pre-commit` exists and still invokes `./.githooks/ductus-pre-commit` (the pre-059 stub line) while `P` is non-empty, wiring is skipped. The warning names the line to replace. The outer file is adopter-owned (`create` strategy), so `/ductus` never edits it, and wiring it as it stands would make every commit fail on a path that does not exist.

The manual integration snippet names the inner hook by its path from the work-tree root, `./{P}.githooks/ductus-pre-commit`, since hook runners run there. The existing permission sets need no change: every agent's bootstrap settings already allow `git rev-parse *` (`framework/bootstrap/ductus.md:54-57`), and `git config core.hooksPath *` covers the new value (`framework/bootstrap/configure/claude.md:66`).

### Family 22 drives the hook from the work-tree root

`scripts/audit/adopter-shell-behavior.sh` builds its fixtures with the project at the repository root and runs the hook from there (`adopter-shell-behavior.sh:68-101`, `:178`). A new fixture puts the project at `proj/`, stages a spec, and runs the outer stub and the inner hook the way git does, from the work-tree root. It asserts three things: the stub runtime was invoked, the staged spec was labelled and re-staged, and the hook exited 0. It is shown to fail against the current hook first. Family 22's existing fixtures are the root regression.

### The CI template names the working directory a subdirectory project sets

`framework/templates/ci/adopter-generators.yml` gains a commented job-level `defaults.run.working-directory`, with a comment explaining it is set to the project's path when the project is not at the repository root. Every step that calls `.ductus/bin/ductus` or reads `.ductus/` and the spec root is a `run` step. `git status --porcelain` reports the whole work tree from any directory, with every path named from the repository root (git-status(1), OUTPUT and porcelain v1). The checkout step is a `uses` step and is unaffected by the default.

### Spec 018 is corrected, which discharges the declared impact

018's body embeds a copy of the outer stub, including the toplevel `cd` and the root-relative invocation (`specs/018-adopter-owned-pre-commit/spec.md:30-44`). It also restates the ladder against `.githooks` and points at `framework/bootstrap/ductus.md` §Hook Installation, which has since moved to `ductus-procedure.md` (`spec.md:52-61`), and it restates the snippet (`spec.md:63-75`). The correction replaces the copy with a pointer to `framework/bootstrap/hooks/pre-commit`. The ladder and the snippet become pointers to `ductus-procedure.md` §Hook Installation (§drift-prevention: a reference is a pointer, never a copy), and a signpost links back to 059. This is a meaningful edit, so 018 reopens `done → in-progress` through the status primitive, and returns to `done` through its own review, analysis, and gate. 018's ticked criteria are read against the new behavior in the same pass, since a criterion stating the root-only wiring may need an annotation.

### Release

The bump is `0.54.0` → `0.54.1`, a patch. The runtime changes are bug fixes; no primitive's arguments or result schema change, and `check-corpus-links` changes only its guidance text. The version goes in `version`, `runtime/Cargo.toml:3`, and a `runtime/CHANGELOG.md` section, and `runtime/Cargo.lock` is refreshed by one build without `--locked`. Audit Family 20 checks that they agree. The tag `ductus-v0.54.1` is cut after 059 and 018 are `done`, following the `AGENTS.md` release entry. No primitive is added, so `framework/runtime-tools.txt` and the configure permission lists are unchanged.

The MCP tools answer with the binary the session started on, so no review or analysis of 059 runs against the MCP server until it is running a binary built after the runtime tasks.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/mod.rs` | Modify | `ProjectRepository`; the three index listers (AC6); the test fixture |
| `runtime/src/primitives/check_stuck.rs` | Modify | AC1 |
| `runtime/src/primitives/check_artifacts.rs` | Modify | AC2 |
| `runtime/src/primitives/diff_cross_spec.rs` | Modify | AC3 |
| `runtime/src/primitives/derive_boundary.rs` | Modify | AC3 |
| `runtime/src/primitives/compute_review_scope.rs` | Modify | onto the helper; `history_prefix` removed |
| `runtime/src/interpreter/payload.rs` | Modify | AC5 |
| `runtime/src/primitives/derive_dependencies.rs` | Modify | AC6 tests |
| `runtime/src/primitives/derive_references.rs` | Modify | AC6 tests |
| `runtime/src/primitives/analyze_subjects.rs` | Modify | AC7 |
| `runtime/src/primitives/check_corpus_links.rs` | Modify | AC8 |
| `framework/bootstrap/hooks/ductus-pre-commit` | Modify | AC9: project root from own location; `--relative` |
| `framework/bootstrap/hooks/pre-commit` | Modify | outer stub invokes the inner hook by location |
| `framework/bootstrap/ductus-procedure.md` | Modify | AC9: ladder and snippet |
| `scripts/audit/adopter-shell-behavior.sh` | Modify | subdirectory fixture |
| `framework/templates/ci/adopter-generators.yml` | Modify | AC10 |
| `specs/018-adopter-owned-pre-commit/spec.md` | Modify | cross-spec correction |
| `version`, `runtime/Cargo.toml`, `runtime/Cargo.lock`, `runtime/CHANGELOG.md` | Modify | release `0.54.1` |

## Trade-offs

- **A conversion at each call site, without a helper, rejected.** It is the shape 058 left in `compute_review_scope`, and ten more copies of it is how the two mistakes above spread in the first place.
- **Reading an unresolvable prefix as empty, rejected.** It keeps `history_prefix`'s behavior, but its answer is "no history," which is the silent wrong answer this spec removes.
- **Finding the hook's project by searching upward for `.ductus/`, rejected.** The file's own location is exact and needs no search. Searching could stop at a nested project's `.ductus/`.
- **Wiring a pre-059 outer stub anyway, rejected.** Every commit would then fail on a path that does not exist until the adopter edits the file. Skipping follows the ladder's existing precedent for conflicts. The cost is that the hook stays unwired until the adopter replaces the line and re-runs `/ductus`, and the warning is the only signal of that.
- **`inbox_standing`'s `workdir_relative` stays as it is.** It canonicalizes the inbox file's parent so a symlinked inbox is blamed under its own name (`inbox_standing.rs:182-188`), which is a different subject from the project root, and its `None` is already the undeterminable state.
- **Limitation: one hooks path per repository.** A second ductus project in the same repository is warned, not wired (spec §Edge Cases). Running several projects' hooks from one `core.hooksPath` is not attempted.
- **Limitation: a hook reached through a symlink placed elsewhere resolves the link's directory.** The snippet invokes the inner hook by its real path, so the supported paths do not go through a link.
- **Not swept: spec 017's install ladder** (`specs/017-derive-dont-ask/spec.md:168-170`). It describes the pre-018 ladder, stale since 018 replaced it, and 059 makes nothing in it newly false at the root.

### Prose-claim sweep (task 13)

Measured by `git grep` for `core.hooksPath`, `show-toplevel`, and `./.githooks/ductus-pre-commit` over the live-artifact set named in `AGENTS.md`, then by meaning for claims that a project sits at its repository's root across `README.md`, `docs/`, `framework/bootstrap/`, and `framework/commands/`. Every hit falls into one of these classes:

- **Corrected by 059.** The two hooks, `ductus-procedure.md` §Hook Installation (ladder, both-files description, spec-017 migration recovery, snippet, status line), the CI template, and 018's body (task 14).
- **True as written.** The configure permission entries (`git config core.hooksPath *` covers `{P}.githooks`). `ductus.md` and `govern.md` line 227 and 029's `state-a-deterministic-path-forcing` scenario name the command generically. This repository's own `scripts/install-hooks.sh` and `.githooks/pre-commit` wire this repository, which is its own root. Every "repo root" and "repo-relative" in `README.md`, `docs/`, `framework/bootstrap/`, and `framework/commands/` means the directory ductus runs in, which is the project root and still where those files belong; none is a claim about git's work tree.
- **True at the root, not swept.** 067's `adopter-generator-promotion` scenario (line 68) says `/ductus` activates the hook with `git config core.hooksPath .githooks`. That is still the exact command at the repository root, and the point it supports, that the value is local config a clone never carries, is unchanged. Editing it would reopen 022 for a full re-review to add an example.
- **Historical.** `runtime/CHANGELOG.md`, `framework/migrations/ductus-rename.md`, and 017's and 018's plans, reviews, and tasks.
- **Predates 059, recorded as disposition task 17.** Three live pointers still name `framework/bootstrap/ductus.md` §Hook Installation, which 056 moved to `ductus-procedure.md`.
- **Same defect, found by this sweep.** Two command-source instructions parse paths out of git output that names them from the work tree's root. `implement.md`'s markdown-only cross-spec check runs `git diff --stat` (step 13's note and the completion gate's step 4), and `amend.md`'s reconcile pass parses `git status --porcelain`, which always names paths from the repository root (git-status(1), porcelain v1). In a subdirectory project each reads the feature's own files as `proj/specs/…`. The other git instructions in `framework/commands/` and `framework/bootstrap/` test only emptiness, an exit code, or a commit count, and read their pathspecs from the working directory, so they are unaffected.
