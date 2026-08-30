//! `check-corpus-links` — relative markdown links in the spec corpus that
//! resolve to nothing.
//!
//! Three checks sit near this ground and none of them covers it.
//! `check-orphaned-references` scopes to the five adopter-owned bootstrap
//! referrers pointing into ductus-managed roots, so spec-to-spec links are
//! outside it by design. `/{project}:analyze` is bounded to one feature
//! directory plus its declared dependencies, so it cannot see the corpus.
//! `scripts/audit/broken-relative-links.sh` (Family 26) performs exactly the
//! right check and is **maintainer-only** — adopters never invoke
//! `/{project}:audit`.
//!
//! So an adopter who deletes or renames a spec directory dangles every inbound
//! pointer — sibling body links, scenario links one tier deeper, and the
//! `dependencies:` edges derived from them — and nothing reports it. The
//! failure is the silent kind: no error, no gate, and the damage surfaces only
//! when a reader follows a pointer to nothing. This primitive is what the
//! adopter's pre-commit hook runs, so a deletion fails at the commit that makes
//! it rather than at a reader's next traversal.
//!
//! Read-only, and it **reports rather than repairs**. The right fix depends on
//! why the target went missing — a depth error is re-pointed, a deliberate
//! removal is named in prose instead of linked — and a rewrite that guessed
//! wrong is worse than a report that is precise.
//!
//! Defined by `specs/022-deterministic-runtime/scenarios/adopter-corpus-link-integrity.md`.

use std::path::{Component, Path, PathBuf};

use crate::primitives::spec_links::is_frontmatter_fence;
use crate::primitives::{Result, inline_code_spans, rel_path};
use crate::schema::paths;
use crate::schema::primitives::{
    BrokenCorpusLink, CheckCorpusLinksArgs, CheckCorpusLinksResult, CorpusLinkSkip,
};

/// Report every relative markdown link under the spec root whose target does
/// not exist.
///
/// # Errors
///
/// Never returns `Err`. Every condition that stops the scan is a **domain
/// outcome** carried in the result: an unreadable file lands in `skipped`, and
/// a spec root that could not be listed sets `guidance`. Reporting either as
/// an error would collapse *could not examine* into *failed to run*, and the
/// caller's whole job is to tell those apart from *examined and clean*.
pub fn run(_args: &CheckCorpusLinksArgs, repo: &Path) -> Result<CheckCorpusLinksResult> {
    let specs_dir = paths::specs_dir(repo);
    let specs_root = rel_path(&specs_dir, repo);

    let mut result = CheckCorpusLinksResult {
        specs_root: specs_root.clone(),
        ..CheckCorpusLinksResult::default()
    };

    let mut files = Vec::new();
    let mut walk_skips = Vec::new();
    if !collect_markdown(
        &specs_dir,
        repo,
        MAX_WALK_DEPTH,
        &mut files,
        &mut walk_skips,
    ) {
        // The subject could not be established. Reported rather than allowed
        // to look like a corpus with no broken links: the whole point is that
        // a silent zero and a real zero must not read alike.
        result.guidance = format!(
            "the spec root `{specs_root}` could not be listed, so no link was checked — this is \
             not the same as finding no broken links"
        );
        return Ok(result);
    }
    files.sort();
    result.skipped.append(&mut walk_skips);

    for path in files {
        let relative = rel_path(&path, repo);
        // Adopter-facing templates are excluded by construction: their links
        // resolve in a scaffolded feature directory, not in the template's
        // own, so a broken link here is the correct state. Counted, never
        // silently dropped.
        if is_template_path(&relative, &specs_root) {
            result.excluded_by_construction += 1;
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            result.skipped.push(CorpusLinkSkip {
                path: relative,
                reason: "unreadable-or-not-utf8".into(),
            });
            continue;
        };
        result.examined.push(relative.clone());
        let here = path.parent().unwrap_or(repo).to_path_buf();
        scan_file(&text, &relative, &here, repo, &mut result);
    }

    if result.examined.is_empty() && result.skipped.is_empty() {
        result.guidance = format!(
            "no markdown files were examined under `{specs_root}` — the link scan found nothing \
             because it looked at nothing"
        );
    }
    Ok(result)
}

/// How deep the walk descends before recording a skip and stopping.
///
/// A spec corpus is three tiers at most (`{root}/{feature}/scenarios/`), so
/// this is far past any real tree. It exists for the pathological one:
/// `is_dir()` follows symlinks, so a link pointing at an ancestor makes the
/// walk descend forever and overflow the stack. A cap turns that into a
/// recorded skip, which is the same answer this primitive gives everywhere
/// else — say what was not examined rather than die or, worse, return a
/// clean-looking partial result.
const MAX_WALK_DEPTH: u32 = 32;

/// Walk `dir` recursively, appending every `.md` file to `out`.
///
/// Returns `false` when the **root** itself could not be listed — the one
/// case that must not read as a clean scan, and the caller turns it into
/// `guidance`.
///
/// A *subdirectory* that cannot be listed, and one deeper than
/// [`MAX_WALK_DEPTH`], each record a `skipped` entry instead. Letting either
/// contribute nothing silently would be this primitive committing the defect
/// it exists to catch: the files that directory holds would be absent from
/// `examined`, and no caller can notice a list that is one shorter than it
/// should be.
fn collect_markdown(
    dir: &Path,
    repo: &Path,
    depth_budget: u32,
    out: &mut Vec<PathBuf>,
    skipped: &mut Vec<CorpusLinkSkip>,
) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.filter_map(std::result::Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if depth_budget == 0 {
                skipped.push(CorpusLinkSkip {
                    path: rel_path(&path, repo),
                    reason: "walk-depth-exceeded".into(),
                });
                continue;
            }
            if !collect_markdown(&path, repo, depth_budget - 1, out, skipped) {
                skipped.push(CorpusLinkSkip {
                    path: rel_path(&path, repo),
                    reason: "directory-unreadable".into(),
                });
            }
        } else if path.extension().is_some_and(|ext| ext == "md") {
            out.push(path);
        }
    }
    true
}

/// Whether `relative` sits under the spec root's `templates/` directory.
fn is_template_path(relative: &str, specs_root: &str) -> bool {
    relative.starts_with(&format!("{specs_root}/templates/"))
}

/// Scan one file's body for broken relative links, appending to `result`.
fn scan_file(
    text: &str,
    relative: &str,
    here: &Path,
    repo: &Path,
    result: &mut CheckCorpusLinksResult,
) {
    let mut fm_seen = false;
    let mut in_fm = false;
    let mut in_fence = false;

    for (idx, raw) in text.lines().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);

        if is_frontmatter_fence(line) {
            if !fm_seen && idx == 0 {
                in_fm = true;
                fm_seen = true;
                continue;
            }
            if in_fm {
                in_fm = false;
                continue;
            }
        }
        if in_fm {
            continue;
        }

        // Fenced code. Toggled line by line rather than stripped wholesale:
        // deleting the block would shift every line number after it, so a
        // finding would cite the wrong line — further off the deeper into the
        // file it sits.
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        // Blockquoted lines are **not** skipped. The blockquote exemption
        // belongs to `derive-dependencies`, where it decides whether a link
        // induces an edge; a link inside a sunset banner still has to resolve.
        for target in link_targets(line) {
            classify(target, relative, idx + 1, here, repo, result);
        }
    }
}

/// Every `](target)` target on one line, with inline code spans removed first.
///
/// Stripping the spans is load-bearing rather than tidy: documentation that
/// discusses linking quotes link syntax constantly, always inside a span, and
/// Family 26 reports seven false positives on this corpus without it — every
/// one a doc correctly describing a link rather than making one.
fn link_targets(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let spans = inline_code_spans(line);
    let mut cursor = 0;
    while let Some(pos) = line[cursor..].find("](") {
        let open = cursor + pos;
        // Always advance past the delimiter, so a non-matching link cannot
        // spin the loop.
        cursor = open + 2;
        if spans.iter().any(|s| s.contains(&open)) {
            continue;
        }
        let rest = &line[cursor..];
        let Some(end) = rest.find(')') else { continue };
        let target = &rest[..end];
        if target.is_empty() || target.chars().any(char::is_whitespace) {
            continue;
        }
        out.push(target);
    }
    out
}

/// Decide what one link target is, and record it when it is broken.
fn classify(
    target: &str,
    relative: &str,
    line: usize,
    here: &Path,
    repo: &Path,
    result: &mut CheckCorpusLinksResult,
) {
    // Scheme-bearing targets and bare fragments are out of scope: nothing on
    // the filesystem answers for them.
    if target.starts_with('#') || has_scheme(target) {
        return;
    }
    // A fragment on a relative target is stripped and the file part checked.
    let rel = target.split('#').next().unwrap_or("");
    if rel.is_empty() {
        return;
    }
    // A pattern is not a reference. A candidate containing `*` or `NNN` is
    // documentation naming a shape, and testing it against the filesystem
    // would manufacture findings out of prose.
    if is_shape(rel) {
        result.shapes_skipped += 1;
        return;
    }
    // Resolution is **lexical**, against the citing file's own directory —
    // except for a root-absolute target, which every markdown renderer
    // resolves against the *repository* root rather than the filesystem's.
    // Never canonicalized: canonicalization would make the result depend on
    // symlinks, so the same corpus would answer differently in two checkouts.
    let base = if rel.starts_with('/') { repo } else { here };
    if lexical_join(base, rel).exists() {
        return;
    }
    // The one-directory-up hint is meaningless for a root-absolute target:
    // its base is fixed, so there is no depth to have got wrong.
    let deeper = lexical_join(base, &format!("../{rel}"));
    let guidance = if !rel.starts_with('/') && deeper.exists() {
        format!("the target resolves one directory up — write `../{rel}`")
    } else {
        "confirm the target still exists; if a later spec removed it, name it in prose instead of \
         linking it"
            .to_string()
    };
    result.broken.push(BrokenCorpusLink {
        path: relative.to_string(),
        line,
        target: target.to_string(),
        guidance,
    });
}

/// Whether `target` carries a URL scheme (`https:`, `mailto:`, …).
///
/// Matched structurally rather than against a list, so a scheme nobody
/// enumerated is still recognized. A Windows-style drive letter cannot be
/// confused for one: a scheme is at least two characters before the colon.
fn has_scheme(target: &str) -> bool {
    let Some(colon) = target.find(':') else {
        return false;
    };
    if colon < 2 {
        return false;
    }
    let head = &target[..colon];
    head.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
        && head.starts_with(|c: char| c.is_ascii_alphabetic())
}

/// Whether `rel` is a documentation shape rather than a path.
fn is_shape(rel: &str) -> bool {
    rel.contains("NNN")
        || rel.contains('{')
        || rel.contains('}')
        || rel.contains('*')
        || rel == "..."
        || rel.ends_with("/...")
}

/// Join `rel` onto `base` and resolve `.` / `..` **textually**.
///
/// `Path::join` alone leaves `..` in the path, and the filesystem would then
/// resolve it through whatever symlinks it crosses. Doing it lexically is what
/// makes the answer a property of the text rather than of the checkout.
fn lexical_join(base: &Path, rel: &str) -> PathBuf {
    let mut out = base.to_path_buf();
    for component in Path::new(rel).components() {
        match component {
            Component::ParentDir => {
                out.pop();
            }
            other @ Component::Normal(_) => out.push(other.as_os_str()),
            // `.` contributes nothing, and a root or prefix component is
            // dropped rather than pushed: `PathBuf::push` of an absolute path
            // *replaces* the buffer, so pushing it would resolve `/foo`
            // against the filesystem root — while the caller has already
            // chosen the repo root as the base, which is where a markdown
            // renderer resolves it.
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write(repo: &Path, rel: &str, body: &str) {
        let path = repo.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    fn run_at(repo: &Path) -> CheckCorpusLinksResult {
        run(&CheckCorpusLinksArgs {}, repo).unwrap()
    }

    #[test]
    fn a_link_to_a_missing_sibling_is_reported() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\nSee [041](../041-gone/spec.md).\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.broken.len(), 1, "{:?}", result.broken);
        assert_eq!(result.broken[0].path, "specs/042-demo/spec.md");
        assert_eq!(result.broken[0].line, 3);
        assert_eq!(result.broken[0].target, "../041-gone/spec.md");
        assert!(result.guidance.is_empty());
    }

    #[test]
    fn a_resolving_link_is_silent_and_the_file_is_named_as_examined() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/041-real/spec.md", "# Real\n");
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\nSee [041](../041-real/spec.md).\n",
        );
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty(), "{:?}", result.broken);
        assert!(result.skipped.is_empty());
        assert_eq!(result.examined.len(), 2);
    }

    #[test]
    fn a_depth_error_one_tier_deep_names_the_fix() {
        // The dominant class: a scenario lives one directory deeper than its
        // spec, so a sibling link written with one `../` too few renders fine
        // and resolves to nothing.
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/041-real/spec.md", "# Real\n");
        write(
            tmp.path(),
            "specs/042-demo/scenarios/thing.md",
            "# Thing\n\nSee [041](../041-real/spec.md).\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.broken.len(), 1, "{:?}", result.broken);
        assert!(
            result.broken[0].guidance.contains("one directory up"),
            "{}",
            result.broken[0].guidance
        );
    }

    #[test]
    fn a_link_quoted_in_an_inline_code_span_is_not_a_link() {
        // Load-bearing, not tidy: documentation that discusses linking quotes
        // link syntax constantly, always inside a span.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\nWrite `[label](../NNN-slug/spec.md)` to cite a sibling.\n",
        );
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty(), "{:?}", result.broken);
        // Stripped before matching, so it never reaches the shape counter
        // either — the span is the earlier of the two guards.
        assert_eq!(result.shapes_skipped, 0);
    }

    #[test]
    fn a_link_inside_a_fenced_block_is_not_checked() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n```text\n[gone](../041-gone/spec.md)\n```\n",
        );
        assert!(run_at(tmp.path()).broken.is_empty());
    }

    #[test]
    fn a_blockquoted_broken_link_is_still_broken() {
        // The blockquote exemption belongs to derive-dependencies, where it
        // decides whether a link induces an *edge*. A sunset banner's link
        // still has to resolve, so this scanner must not inherit that skip.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n> **Sunset ([043](../043-gone/spec.md)):** removed.\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.broken.len(), 1, "{:?}", result.broken);
        assert_eq!(result.broken[0].target, "../043-gone/spec.md");
    }

    #[test]
    fn frontmatter_is_skipped_and_line_numbers_still_count_from_the_top() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "---\nstatus: done\n---\n\n# Demo\n\n[gone](../041-gone/spec.md)\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.broken.len(), 1);
        assert_eq!(result.broken[0].line, 7);
    }

    #[test]
    fn an_absent_spec_root_is_guidance_not_a_clean_verdict() {
        // The QUAL-CLAIM-001 case: a check that could not run must never be
        // indistinguishable from one that passed.
        let tmp = tempdir().unwrap();
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty());
        assert!(result.examined.is_empty());
        assert!(
            result.guidance.contains("could not be listed"),
            "{}",
            result.guidance
        );
    }

    #[test]
    fn an_empty_spec_root_is_guidance_not_a_clean_verdict() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty());
        assert!(
            result.guidance.contains("looked at nothing"),
            "{}",
            result.guidance
        );
    }

    #[test]
    fn schemes_and_bare_fragments_are_out_of_scope() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n[a](https://example.com/x.md) [b](mailto:x@example.com) [c](#heading)\n",
        );
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty(), "{:?}", result.broken);
    }

    #[test]
    fn a_fragment_on_a_relative_target_is_stripped_and_the_file_part_checked() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/041-real/spec.md", "# Real\n");
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n[ok](../041-real/spec.md#motivation) [no](../041-gone/spec.md#x)\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.broken.len(), 1, "{:?}", result.broken);
        assert_eq!(result.broken[0].target, "../041-gone/spec.md#x");
    }

    #[test]
    fn a_shape_is_counted_rather_than_tested() {
        // A pattern is not a reference; testing one against the filesystem
        // manufactures findings out of prose.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n[a](../NNN-slug/spec.md) [b](../{feature}/spec.md) [c](../*/spec.md)\n",
        );
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty(), "{:?}", result.broken);
        assert_eq!(result.shapes_skipped, 3);
    }

    #[test]
    fn adopter_templates_are_excluded_by_construction_and_counted() {
        // A template's links resolve in a scaffolded feature directory, not in
        // the template's own, so a broken link there is the correct state.
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/templates/spec.md",
            "# {NNN}\n\n[plan](plan.md)\n",
        );
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty());
        assert_eq!(result.excluded_by_construction, 1);
        assert!(result.examined.is_empty());
        // Zero examined is not a clean corpus, and the result says so.
        assert!(!result.guidance.is_empty());
    }

    #[test]
    fn resolution_is_lexical_so_a_parent_hop_is_textual() {
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/041-real/scenarios/x.md", "# X\n");
        write(
            tmp.path(),
            "specs/042-demo/scenarios/y.md",
            "# Y\n\n[x](../../041-real/scenarios/x.md)\n",
        );
        let result = run_at(tmp.path());
        assert!(result.broken.is_empty(), "{:?}", result.broken);
    }

    #[test]
    fn a_configured_spec_root_is_walked_rather_than_an_assumed_specs() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        fs::write(
            tmp.path().join(".ductus/config.toml"),
            "[paths]\nspecs-root = \"requirements\"\n",
        )
        .unwrap();
        write(
            tmp.path(),
            "requirements/042-demo/spec.md",
            "# Demo\n\n[gone](../041-gone/spec.md)\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.specs_root, "requirements");
        assert_eq!(result.broken.len(), 1, "{:?}", result.broken);
    }

    #[test]
    fn a_root_absolute_target_resolves_against_the_repo_root() {
        // What a markdown renderer does with `/specs/...`. Resolving it
        // against the *filesystem* root instead would report every one of
        // them broken, or — worse on a machine that happens to have the
        // path — report a link to somewhere outside the repo as fine.
        let tmp = tempdir().unwrap();
        write(tmp.path(), "specs/041-real/spec.md", "# Real\n");
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n[ok](/specs/041-real/spec.md) [no](/specs/041-gone/spec.md)\n",
        );
        let result = run_at(tmp.path());
        assert_eq!(result.broken.len(), 1, "{:?}", result.broken);
        assert_eq!(result.broken[0].target, "/specs/041-gone/spec.md");
        // The depth hint is meaningless for a fixed base, so it is not given.
        assert!(
            !result.broken[0].guidance.contains("one directory up"),
            "{}",
            result.broken[0].guidance
        );
    }

    #[test]
    fn a_directory_below_the_depth_cap_is_recorded_as_skipped() {
        // The pathological case the cap exists for is a symlink loop, which
        // `is_dir()` follows forever. Plain nesting reaches the same guard
        // and is the testable form of it: what matters is that the walk says
        // what it did not examine rather than dying, or quietly returning a
        // partial result that reads as clean.
        let tmp = tempdir().unwrap();
        let mut deep = String::from("specs");
        for _ in 0..(super::MAX_WALK_DEPTH + 2) {
            deep.push_str("/d");
        }
        write(tmp.path(), &format!("{deep}/spec.md"), "# Deep\n");
        let result = run_at(tmp.path());
        assert!(
            result
                .skipped
                .iter()
                .any(|s| s.reason == "walk-depth-exceeded"),
            "{:?}",
            result.skipped
        );
    }

    #[test]
    fn a_link_with_no_closing_paren_is_skipped_without_spinning() {
        let tmp = tempdir().unwrap();
        write(
            tmp.path(),
            "specs/042-demo/spec.md",
            "# Demo\n\n[unterminated](../041-gone/spec.md\n",
        );
        assert!(run_at(tmp.path()).broken.is_empty());
    }
}
