//! `derive-dependencies` — regenerate every spec's frontmatter
//! `dependencies:` from the sibling-spec links in its body.
//!
//! Body inline links are authoritative; the frontmatter is a derived index
//! (spec 017). This primitive replaces `.ductus/scripts/gen-spec-deps.sh`,
//! which ran the same derivation in `awk` on every adopter's machine on every
//! commit. §runtime-boundary principle 3 names shell pipelines that parse
//! frontmatter or markdown structure as *not* a sanctioned substitute for a
//! primitive; the script predates the runtime being required, and this is the
//! promotion (see `scenarios/adopter-generator-promotion.md`).
//!
//! ## What it derives
//!
//! For each tracked feature spec, the set of `NNN-slug` sibling directories
//! named by an inline link in the harvestable body — `](../NNN-slug/…)` or
//! `](<specs-root>/NNN-slug/…)` — sorted and deduplicated. The harvestable
//! region (what the four exclusions drop) is [`super::spec_links`]'s
//! contract, shared with `derive-references`.
//!
//! **Self-links are recorded, never stripped.** A spec linking itself is a
//! 1-cycle, and the cycle check below is what surfaces it; silently dropping
//! the edge would hide the defect it exists to report.
//!
//! ## Empty is `[]`, not absent
//!
//! A spec with no sibling links gets `dependencies: []` — the key present and
//! empty. This differs from `derive-references`, where an empty index removes
//! the key entirely. The asymmetry is deliberate and load-bearing: unifying
//! the two would rewrite frontmatter across every spec in a corpus and read
//! as a mass mechanical edit. Each primitive keeps its own rule.
//!
//! ## Cycle detection
//!
//! After the rewrite — so any diff is visible in the working tree even when
//! the run fails — Tarjan's SCC runs over the derived graph. Any SCC of size
//! greater than one, and any self-loop, is a cycle. The graph always spans
//! *every* tracked spec even under `--staged`, because a staged edge can
//! close a cycle through a spec that is not staged.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::primitives::spec_links::{
    harvestable_lines, has_unterminated_frontmatter, is_frontmatter_fence,
};
use crate::primitives::{Result, read_text, rel_path, write_atomic};
use crate::schema::paths;
use crate::schema::primitives::{DeriveDependenciesArgs, DeriveDependenciesResult};

/// Execute the `derive-dependencies` primitive.
///
/// # Errors
///
/// Returns [`super::PrimitiveError::Io`] when a spec cannot be read or the
/// rewrite cannot be persisted, and [`super::PrimitiveError::Git`] when a
/// repository contains the project but its index or status cannot be read —
/// the listing of which specs to derive then has no answer, and guessing one
/// would rewrite drafts or skip specs silently. A cycle is a **domain
/// outcome** reported in the result, not an error: the caller decides whether
/// it blocks.
pub fn run(args: &DeriveDependenciesArgs, repo: &Path) -> Result<DeriveDependenciesResult> {
    let specs_root = paths::Paths::load(repo).specs_root;
    let specs_dir = repo.join(&specs_root);

    let tracked = super::list_tracked_specs(repo, &specs_root)?;
    let untracked = super::list_untracked_specs(repo, &specs_root)?;
    let staged = if args.staged {
        Some(super::list_staged_specs(repo, &specs_root)?)
    } else {
        None
    };

    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut updated = Vec::new();
    let mut unwritten = Vec::new();
    let mut unparseable = Vec::new();
    let mut absent = Vec::new();
    let mut read_count: usize = 0;

    for spec in &tracked {
        let path = repo.join(spec);
        if !path.is_file() {
            // Tracked in the index but not on disk. Named rather than dropped,
            // for the reason recorded on `derive_references` — counting it in
            // `examined` asserts a subject that was never read.
            absent.push(spec.clone());
            continue;
        }
        read_count += 1;
        let own_slug = super::spec_feature_slug(spec, &specs_root).unwrap_or_default();
        let content = read_text(&path)?;
        if has_unterminated_frontmatter(&content) {
            // No closing fence means the splice below has no anchor, so the
            // spec is left alone. Say so rather than letting it read as clean.
            unparseable.push(spec.clone());
            continue;
        }

        let deps = harvest(&content, &specs_root);
        graph.insert(own_slug, deps.iter().cloned().collect());

        let rewritten = splice_dependencies(&content, &deps);
        if rewritten == content {
            continue;
        }
        // Differs. Whether it is written depends on the staged filter.
        let is_target = staged.as_ref().is_none_or(|set| set.contains(spec));
        if !is_target {
            // Examined and found drifted, but deliberately left alone so
            // committing one spec never rewrites another. Reporting this as
            // "in sync" would assert the opposite of what was observed.
            unwritten.push(spec.clone());
            continue;
        }
        if args.write {
            write_atomic(&path, &rewritten)?;
        }
        updated.push(spec.clone());
    }

    let cycles = find_cycles(&graph);

    Ok(DeriveDependenciesResult {
        drift: !updated.is_empty(),
        updated,
        unwritten,
        examined: u32::try_from(read_count).unwrap_or(u32::MAX),
        untracked_skipped: untracked,
        unparseable,
        absent,
        cycles,
        specs_root: rel_path(&specs_dir, repo),
        wrote: args.write,
    })
}

/// Harvest the sorted, deduplicated sibling slugs from a spec's body.
fn harvest(content: &str, specs_root: &str) -> BTreeSet<String> {
    let mut slugs = BTreeSet::new();
    for line in harvestable_lines(content) {
        scan_line(line.text, specs_root, 1, &mut slugs);
    }
    slugs
}

/// Collect every sibling-spec link target on one line: `](../NNN-slug` or
/// `](<specs-root>/NNN-slug`.
///
/// Hand-matched rather than compiled from a format string. A regex here would
/// have to interpolate the configured spec root, which makes construction
/// fallible on a value this function cannot validate — and the only honest
/// responses to that are an `unwrap` or an error path for a case that cannot
/// happen. Matching directly has neither, and the grammar is three tokens.
///
/// `depth` is how many directory levels below the spec root the *citing* file
/// sits, which is how many `../` segments its relative links carry: 1 for a
/// `spec.md`, 2 for a `scenarios/{slug}.md`. The repo-root-relative form
/// (`](<specs-root>/NNN-slug`) is depth-independent and always matches.
///
/// `pub(crate)` with that parameter so the pre-`done` gate's cross-spec-impact
/// check reads back-links with this matcher rather than growing a second one
/// (spec 050, `a-declared-cross-spec-impact-gates-done`).
pub(crate) fn scan_line(line: &str, specs_root: &str, depth: usize, out: &mut BTreeSet<String>) {
    let mut cursor = 0;
    while let Some(pos) = line[cursor..].find("](") {
        // Always advance past the delimiter, so a non-matching link cannot
        // spin the loop.
        cursor += pos + 2;
        let rest = &line[cursor..];
        let relative = {
            let mut tail = Some(rest);
            for _ in 0..depth {
                tail = tail.and_then(|t| t.strip_prefix("../"));
            }
            tail
        };
        let after_prefix = if let Some(tail) = relative {
            tail
        } else if let Some(tail) = rest
            .strip_prefix(specs_root)
            .and_then(|tail| tail.strip_prefix('/'))
        {
            tail
        } else {
            continue;
        };
        if let Some(slug) = leading_slug(after_prefix) {
            out.insert(slug.to_string());
        }
    }
}

/// The feature-directory segment at the head of `s`, in **either** form
/// spec 051 defines. `None` when the first segment is not a feature
/// directory.
///
/// Delegates to [`super::parse_feature_dir`] rather than re-deriving the
/// grammar. It used to test *exactly* three ASCII digits and a hyphen,
/// which disagreed with the shared rule in both directions at once: a link
/// to a branch-scoped spec (`1234.1-slug`) or to a four-digit one
/// (`1000-slug`) harvested **no** edge, silently, so the dependency never
/// appeared in the frontmatter and nothing reported its absence.
///
/// The guard is load-bearing and so is widened rather than removed:
/// `harvest` writes this set straight into `dependencies:` with no
/// downstream validation against the enumerated corpus, which is what
/// distinguishes it from the audit script that carried the same grammar
/// and could drop it outright.
///
/// **The charset check is separate from the form check, and both are
/// required.** [`super::parse_feature_dir`] deliberately does *not* hold a
/// sequential directory's trailing slug to the slug grammar — it exists to
/// recognize directories that already exist on disk, where an adopter's
/// spec root may legitimately hold a name that predates the grammar's
/// enforcement. This call site is the opposite trust context: its input is
/// a link target copied out of a document body, and its output is written
/// verbatim into a YAML flow sequence. Delegating the form alone let
/// `](../001-x], status: done, [y/spec.md)` render as
/// `dependencies: [001-x], status: done, [y]`, splicing a second key onto
/// the line — proven by probe, in the pass that introduced it. So the form
/// comes from the shared grammar and the charset stays local, which is
/// what the predicate this replaced was quietly providing.
///
/// **The charset also decides where the candidate ends**, which is the
/// second thing the old predicate was doing silently. It scanned forward
/// while the bytes were `[a-z0-9-]` and stopped, so `](../002-b)` yielded
/// `002-b`; terminating on `/` alone instead left the `)` attached and
/// dropped the edge — a directory-only link to a sibling spec harvested
/// nothing. Both regressions came from the same move and neither was
/// visible in this corpus, which happens to hold no such link. Scanning to
/// the first byte outside the feature-name charset restores the old
/// boundary exactly while leaving the *form* rule widened, and it closes
/// the splice by construction: a `]`, a comma or a space ends the
/// candidate rather than entering it.
fn leading_slug(s: &str) -> Option<&str> {
    let end = s
        .find(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '.'))
        .unwrap_or(s.len());
    let seg = &s[..end];
    super::is_feature_slug(seg).then_some(seg)
}

/// Replace the frontmatter's `dependencies:` entry — the key line *and* any
/// indented continuation beneath it — with the derived list.
///
/// Replacing only the key line is correct for the inline flow form
/// (`dependencies: [a, b]`) but corrupts the equally-valid block form:
///
/// ```text
/// dependencies:          ->   dependencies: []
///   - 000-stale                 - 000-stale     <- orphaned, invalid YAML
/// ```
///
/// The two shell generators diverged on exactly this; one had the
/// continuation skip and the other did not.
///
/// A spec with no `dependencies:` key is left untouched rather than having
/// one inserted: where the key belongs in the frontmatter is an authoring
/// decision, and `validate-frontmatter` owns reporting its absence.
fn splice_dependencies(content: &str, deps: &BTreeSet<String>) -> String {
    let new_line = if deps.is_empty() {
        "dependencies: []".to_string()
    } else {
        format!(
            "dependencies: [{}]",
            deps.iter().cloned().collect::<Vec<_>>().join(", ")
        )
    };

    let line_ending = super::line_ending_of(content);
    let mut out: Vec<String> = Vec::new();
    let mut fm_seen = false;
    let mut in_fm = false;
    let mut replaced = false;
    let mut skipping = false;

    for (idx, raw) in content.lines().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);

        if is_frontmatter_fence(line) {
            if !fm_seen && idx == 0 {
                in_fm = true;
                fm_seen = true;
                out.push(line.to_string());
                continue;
            }
            if in_fm {
                in_fm = false;
                skipping = false;
                out.push(line.to_string());
                continue;
            }
        }

        if in_fm && skipping {
            // Continuation lines of the replaced key are indented.
            if line.starts_with(' ') || line.starts_with('\t') {
                continue;
            }
            skipping = false;
        }

        if in_fm && !replaced && line.starts_with("dependencies:") {
            out.push(new_line.clone());
            replaced = true;
            skipping = true;
            continue;
        }

        out.push(line.to_string());
    }

    let mut joined = out.join(line_ending);
    if content.ends_with('\n') {
        joined.push_str(line_ending);
    }
    joined
}

/// Tarjan's strongly-connected-components over the derived graph.
///
/// Returns each cycle's members in sorted order, the cycles themselves sorted
/// by their least member, so output is stable across runs. An SCC of size one
/// is a cycle only when the node links itself.
///
/// Iterative rather than recursive: a spec corpus is author-controlled and a
/// deep dependency chain would otherwise put stack depth at the mercy of the
/// input.
fn find_cycles(graph: &BTreeMap<String, Vec<String>>) -> Vec<Vec<String>> {
    #[derive(Clone, Copy)]
    struct Frame {
        node: usize,
        edge: usize,
    }

    let nodes: Vec<&String> = graph.keys().collect();
    let index_of: BTreeMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, name)| (name.as_str(), i))
        .collect();
    let adjacency: Vec<Vec<usize>> = nodes
        .iter()
        .map(|name| {
            graph[*name]
                .iter()
                .filter_map(|dep| index_of.get(dep.as_str()).copied())
                .collect()
        })
        .collect();

    let n = nodes.len();
    let mut index = vec![usize::MAX; n];
    let mut lowlink = vec![usize::MAX; n];
    let mut on_stack = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut next_index = 0usize;
    let mut cycles: Vec<Vec<String>> = Vec::new();

    for root in 0..n {
        if index[root] != usize::MAX {
            continue;
        }
        let mut call_stack = vec![Frame {
            node: root,
            edge: 0,
        }];
        index[root] = next_index;
        lowlink[root] = next_index;
        next_index += 1;
        stack.push(root);
        on_stack[root] = true;

        while let Some(frame) = call_stack.last_mut() {
            let v = frame.node;
            if frame.edge < adjacency[v].len() {
                let w = adjacency[v][frame.edge];
                frame.edge += 1;
                if index[w] == usize::MAX {
                    index[w] = next_index;
                    lowlink[w] = next_index;
                    next_index += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    call_stack.push(Frame { node: w, edge: 0 });
                } else if on_stack[w] {
                    lowlink[v] = lowlink[v].min(index[w]);
                }
                continue;
            }

            // Every edge explored: close the node.
            call_stack.pop();
            if let Some(parent) = call_stack.last() {
                lowlink[parent.node] = lowlink[parent.node].min(lowlink[v]);
            }
            if lowlink[v] == index[v] {
                let mut component = Vec::new();
                while let Some(w) = stack.pop() {
                    on_stack[w] = false;
                    component.push(nodes[w].clone());
                    if w == v {
                        break;
                    }
                }
                let is_self_loop = component.len() == 1 && adjacency[v].contains(&v);
                if component.len() > 1 || is_self_loop {
                    component.sort();
                    cycles.push(component);
                }
            }
        }
    }

    cycles.sort();
    cycles
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn spec(deps_line: &str, body: &str) -> String {
        format!("---\nstatus: done\n{deps_line}\n---\n\n{body}\n")
    }

    /// A project in a subdirectory of its repository, with two tracked specs
    /// and an untracked draft. The project root is not a repository to open,
    /// and the index names every path from the work tree, so every listing
    /// took its no-repository path: `--write` rewrote the draft, nothing was
    /// reported untracked, and `--staged` rewrote nothing (spec 059).
    #[test]
    fn a_subdirectory_project_derives_from_its_index() {
        use crate::primitives::git_fixture;
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project) = git_fixture::subdirectory_project(tmp.path());
        let a = project.join("specs/001-a/spec.md");
        let c = project.join("specs/003-c/spec.md");
        git_fixture::write(&a, &spec("dependencies: []", "nothing"));
        git_fixture::write(&c, &spec("dependencies: []", "nothing"));
        git_fixture::commit_all(&repository, "feat: specs");
        let draft_path = project.join("specs/002-draft/spec.md");
        let draft = spec("dependencies: [999-wrong]", "[a](../001-a/spec.md)");
        git_fixture::write(&draft_path, &draft);

        let write = DeriveDependenciesArgs {
            write: true,
            staged: false,
        };
        let result = run(&write, &project).unwrap();
        assert_eq!(std::fs::read_to_string(&draft_path).unwrap(), draft);
        assert_eq!(result.examined, 2);
        assert_eq!(result.untracked_skipped, vec!["specs/002-draft/spec.md"]);

        // Both tracked specs drift; only the first is staged.
        git_fixture::write(&a, &spec("dependencies: []", "[c](../003-c/spec.md)"));
        git_fixture::write(&c, &spec("dependencies: []", "[a](../001-a/spec.md)"));
        let mut index = repository.index().unwrap();
        index
            .add_path(Path::new("proj/specs/001-a/spec.md"))
            .unwrap();
        index.write().unwrap();
        let staged = DeriveDependenciesArgs {
            write: true,
            staged: true,
        };
        let result = run(&staged, &project).unwrap();
        assert_eq!(result.updated, vec!["specs/001-a/spec.md"]);
        assert_eq!(result.unwritten, vec!["specs/003-c/spec.md"]);
        assert!(
            std::fs::read_to_string(&a)
                .unwrap()
                .contains("dependencies: [003-c]")
        );
        assert!(
            std::fs::read_to_string(&c)
                .unwrap()
                .contains("dependencies: []")
        );
    }

    #[test]
    fn harvests_relative_and_rooted_links_sorted_and_deduped() {
        let root = "specs";
        let content = spec(
            "dependencies: []",
            "See [b](../002-b/spec.md), [a](specs/001-a/spec.md), and [b again](../002-b/plan.md).",
        );
        let deps: Vec<String> = harvest(&content, root).into_iter().collect();
        assert_eq!(deps, vec!["001-a", "002-b"]);
    }

    #[test]
    fn harvests_branch_scoped_and_four_digit_siblings() {
        // Both forms `parse_feature_dir` accepts must produce an edge. The
        // predicate here used to demand *exactly* three digits, so a link to
        // either derived no dependency at all — silently, since an absent
        // edge looks the same as a spec that cites nobody.
        let root = "specs";
        let content = spec(
            "dependencies: []",
            "See [staged](../1234.1-staged/spec.md) and [big](../1000-thousandth/spec.md).",
        );
        let deps: Vec<String> = harvest(&content, root).into_iter().collect();
        assert_eq!(deps, vec!["1000-thousandth", "1234.1-staged"]);
    }

    #[test]
    fn a_non_feature_directory_still_yields_no_edge() {
        // Widening the grammar must not turn every relative link into a
        // dependency: `harvest` writes this set straight into the
        // frontmatter with no downstream validation, so the guard is the
        // only thing standing between a doc link and a false edge.
        let root = "specs";
        let content = spec(
            "dependencies: []",
            "See [docs](../docs/guide.md), [bare](../notes/a.md), and [nodigits](../abc-def/spec.md).",
        );
        assert!(harvest(&content, root).is_empty());
    }

    #[test]
    fn a_crafted_link_target_cannot_splice_the_frontmatter_line() {
        // `harvest`'s output is written verbatim into a YAML flow sequence,
        // and its input is a link target copied out of a document body. The
        // shared grammar recognizes the *form* but deliberately leaves a
        // sequential slug's charset open, because it exists to match
        // directories already on disk. Delegating the form alone rendered
        // `dependencies: [001-x], status: done, [y]` — a second key spliced
        // onto the line. Proven by probe before this guard existed.
        //
        // The candidate ends at the `]`, so the crafted target contributes an
        // ordinary (if spurious) slug and nothing else — which is exactly what
        // the predecessor predicate did. What must never survive is a value
        // carrying a character that means something in the rendered list.
        let root = "specs";
        let content = spec(
            "dependencies: []",
            "[x](../001-evil], status: done, [y/spec.md) and [z](../001-ok-slug/spec.md).",
        );
        let deps: Vec<String> = harvest(&content, root).into_iter().collect();
        assert_eq!(deps, vec!["001-evil", "001-ok-slug"]);
        assert!(
            deps.iter().all(|d| !d.contains([']', '[', ',', ':', ' '])),
            "a harvested slug must be safe in a YAML flow sequence: {deps:?}"
        );
    }

    #[test]
    fn a_directory_only_link_still_harvests() {
        // `](../002-b)` names a sibling spec without naming a file in it. The
        // predecessor predicate scanned while the bytes were `[a-z0-9-]` and
        // stopped, so the `)` ended the candidate; terminating on `/` alone
        // left it attached and silently dropped the edge. This corpus holds no
        // such link, so nothing here would have noticed.
        let root = "specs";
        let content = spec("dependencies: []", "See [b](../002-b) for the contract.");
        let deps: Vec<String> = harvest(&content, root).into_iter().collect();
        assert_eq!(deps, vec!["002-b"]);
    }

    #[test]
    fn honors_a_non_default_specs_root() {
        let root = "governance";
        let content = spec("dependencies: []", "[a](governance/001-a/spec.md)");
        let deps: Vec<String> = harvest(&content, root).into_iter().collect();
        assert_eq!(deps, vec!["001-a"]);
        // The default root must not match under an override.
        let content = spec("dependencies: []", "[a](specs/001-a/spec.md)");
        assert!(harvest(&content, root).is_empty());
    }

    #[test]
    fn self_links_are_recorded_not_stripped() {
        let root = "specs";
        let content = spec("dependencies: []", "[self](../001-a/spec.md)");
        let deps: Vec<String> = harvest(&content, root).into_iter().collect();
        assert_eq!(deps, vec!["001-a"]);
    }

    #[test]
    fn empty_harvest_renders_an_empty_list_not_an_absent_key() {
        let content = spec("dependencies: [001-a]", "no links here");
        let out = splice_dependencies(&content, &BTreeSet::new());
        assert!(out.contains("dependencies: []"));
    }

    #[test]
    fn splice_replaces_the_block_form_including_continuations() {
        let content =
            "---\nstatus: done\ndependencies:\n  - 000-stale\n  - 001-also\nnext: 3\n---\nbody\n";
        let deps: BTreeSet<String> = ["002-new".to_string()].into_iter().collect();
        let out = splice_dependencies(content, &deps);
        assert!(out.contains("dependencies: [002-new]"));
        assert!(!out.contains("000-stale"), "orphaned continuation survived");
        assert!(!out.contains("001-also"));
        assert!(out.contains("next: 3"), "following key was consumed");
    }

    #[test]
    fn splice_leaves_a_spec_without_the_key_untouched() {
        let content = "---\nstatus: done\n---\nbody\n";
        let deps: BTreeSet<String> = ["001-a".to_string()].into_iter().collect();
        assert_eq!(splice_dependencies(content, &deps), content);
    }

    #[test]
    fn splice_only_touches_frontmatter() {
        // A body line starting with `dependencies:` must not be rewritten.
        let content = "---\nstatus: done\ndependencies: []\n---\ndependencies: not frontmatter\n";
        let deps: BTreeSet<String> = ["001-a".to_string()].into_iter().collect();
        let out = splice_dependencies(content, &deps);
        assert!(out.contains("dependencies: [001-a]"));
        assert!(out.contains("dependencies: not frontmatter"));
    }

    #[test]
    fn an_indented_fence_inside_a_block_scalar_is_not_a_delimiter() {
        // Regression: a trimmed comparison treats the indented `---` inside a
        // YAML block scalar as the closing fence, ending the frontmatter early
        // so a `dependencies:` key *below* it is silently never rewritten.
        // Column-zero anchoring is what makes this content rather than syntax.
        let content = "---\nstatus: done\ndescription: |\n  a block containing\n  ---\n  more text\ndependencies: []\nnext-criterion: 3\n---\n\nbody\n";
        let deps: BTreeSet<String> = ["001-a".to_string()].into_iter().collect();
        let out = splice_dependencies(content, &deps);
        assert!(
            out.contains("dependencies: [001-a]"),
            "key below an indented fence was not rewritten:\n{out}"
        );
        assert!(
            out.contains("  ---"),
            "block-scalar content was altered:\n{out}"
        );
    }

    #[test]
    fn a_fence_with_trailing_whitespace_still_closes_the_block() {
        // The mirror case: `---   ` IS a delimiter, so a trim-start-only
        // comparison wrongly treats the frontmatter as unterminated.
        let content =
            "---\nstatus: done\ndependencies: []\n---   \n\ndependencies: not frontmatter\n";
        let deps: BTreeSet<String> = ["001-a".to_string()].into_iter().collect();
        let out = splice_dependencies(content, &deps);
        assert!(out.contains("dependencies: [001-a]"), "{out}");
        assert!(
            out.contains("dependencies: not frontmatter"),
            "body line was rewritten as frontmatter:\n{out}"
        );
    }

    #[test]
    fn splice_is_idempotent() {
        let content = spec("dependencies: [001-a]", "[a](../001-a/spec.md)");
        let deps: BTreeSet<String> = ["001-a".to_string()].into_iter().collect();
        let once = splice_dependencies(&content, &deps);
        assert_eq!(once, content, "in-sync spec should not be rewritten");
        assert_eq!(splice_dependencies(&once, &deps), once);
    }

    #[test]
    fn splice_preserves_crlf_line_endings() {
        let content = "---\r\nstatus: done\r\ndependencies: []\r\n---\r\nbody\r\n";
        let deps: BTreeSet<String> = ["001-a".to_string()].into_iter().collect();
        let out = splice_dependencies(content, &deps);
        assert!(out.contains("dependencies: [001-a]\r\n"));
        assert!(!out.contains("dependencies: [001-a]\n\n"));
    }

    #[test]
    fn detects_a_two_node_cycle() {
        let graph: BTreeMap<String, Vec<String>> = [
            ("001-a".to_string(), vec!["002-b".to_string()]),
            ("002-b".to_string(), vec!["001-a".to_string()]),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            find_cycles(&graph),
            vec![vec!["001-a".to_string(), "002-b".to_string()]]
        );
    }

    #[test]
    fn detects_a_self_loop() {
        let graph: BTreeMap<String, Vec<String>> =
            [("001-a".to_string(), vec!["001-a".to_string()])]
                .into_iter()
                .collect();
        assert_eq!(find_cycles(&graph), vec![vec!["001-a".to_string()]]);
    }

    #[test]
    fn an_acyclic_graph_reports_nothing() {
        let graph: BTreeMap<String, Vec<String>> = [
            ("001-a".to_string(), vec!["002-b".to_string()]),
            ("002-b".to_string(), vec!["003-c".to_string()]),
            ("003-c".to_string(), vec![]),
        ]
        .into_iter()
        .collect();
        assert_eq!(find_cycles(&graph), [] as [Vec<String>; 0]);
    }

    #[test]
    fn edges_to_absent_specs_are_ignored_by_the_cycle_check() {
        // A link to a spec that does not exist is a broken-link concern, not
        // a cycle; it must not panic the index lookup either.
        let graph: BTreeMap<String, Vec<String>> =
            [("001-a".to_string(), vec!["999-missing".to_string()])]
                .into_iter()
                .collect();
        assert_eq!(find_cycles(&graph), [] as [Vec<String>; 0]);
    }

    #[test]
    fn cycle_output_is_deterministic_across_equivalent_graphs() {
        let graph: BTreeMap<String, Vec<String>> = [
            ("003-c".to_string(), vec!["001-a".to_string()]),
            ("001-a".to_string(), vec!["002-b".to_string()]),
            ("002-b".to_string(), vec!["003-c".to_string()]),
        ]
        .into_iter()
        .collect();
        let first = find_cycles(&graph);
        assert_eq!(
            first,
            vec![vec![
                "001-a".to_string(),
                "002-b".to_string(),
                "003-c".to_string()
            ]]
        );
        assert_eq!(find_cycles(&graph), first);
    }

    #[test]
    fn a_deep_chain_does_not_overflow_the_stack() {
        // The iterative walk exists for this; a recursive one blows up here.
        let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for i in 0..5000 {
            let node = format!("{i:05}-n");
            let next = format!("{:05}-n", i + 1);
            graph.insert(node, vec![next]);
        }
        graph.insert("05000-n".to_string(), vec![]);
        assert_eq!(find_cycles(&graph), [] as [Vec<String>; 0]);
    }
}
