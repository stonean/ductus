//! `derive-references` — regenerate every spec's frontmatter `references:`
//! from the cross-service spec links in its body.
//!
//! The companion to [`super::derive_dependencies`], replacing
//! `.ductus/scripts/gen-cross-service-refs.sh`. Both harvest links out of the
//! same body region (the shared [`super::spec_links`] scanner); they differ in
//! what they match and what they write.
//!
//! ## Strictly distinct from `dependencies:`
//!
//! A reference is *informative cross-service navigation* and never enters the
//! blocking dependency graph (spec 030). This primitive never reads or writes
//! `dependencies:`, and sibling `../NNN-slug/` links are relative, never match
//! the absolute-URL predicate, and stay `derive-dependencies`' exclusive
//! domain.
//!
//! ## Absent when empty
//!
//! A spec with no cross-service links carries **no** `references:` key, and a
//! stale block is removed when its last link goes. This is deliberately unlike
//! `derive-dependencies`' `dependencies: []`. Unifying the two rules would
//! rewrite frontmatter across every spec in a corpus.
//!
//! ## One extra exclusion
//!
//! On top of the four the shared scanner applies, a link inside an
//! inline-code span is skipped: backticked text renders as literal, not as a
//! clickable link, so by the spec it is an illustrative example rather than a
//! reference. `derive-dependencies` has no such rule — the two generators
//! genuinely differ here, and the difference is load-bearing.
//!
//! ## The registry is a second input, and it changes what `--staged` means
//!
//! A `dependencies:` edge is a pure function of the spec body, so it can only
//! drift when that spec is edited — which stages it, which makes it visible to
//! a staged-mode run. A `references:` entry is **not**: it is a function of the
//! body *and* the `[services]` registry, because [`harvest`] resolves each
//! link's repo URL through that registry to produce the `service:` alias.
//!
//! So renaming a service alias drifts every spec referencing it while leaving
//! those specs untouched — and an untouched spec is never staged. An earlier
//! version narrowed the *enumeration* to the staged set under `--staged`,
//! which made that entire class structurally invisible: an adopter carried
//! dead references for nine commits while the pre-commit hook reported the
//! tree in sync every time.
//!
//! The walk therefore spans every tracked spec and `--staged` filters only the
//! write, with drifted-but-unstaged specs reported in `unwritten`. This
//! primitive needs the full walk *more* than `derive-dependencies` does, not
//! less — the opposite of what the original rationale concluded. See
//! `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md`.
//!
//! ## Root-aware matching
//!
//! The spec-root segment of a referenced URL is **not** hardcoded to `specs`:
//! a referenced service may rename its own root (spec 040). Two tiers:
//!
//! * **Checkout reachable** — the service is registered and its local `path`
//!   resolves, so the matcher reads that checkout's own `[paths] specs-root`
//!   and accepts only that exact segment.
//! * **Checkout unreachable** — a registered service that is not checked out,
//!   or an unregistered repo, has an unknowable root, so any single
//!   `[A-Za-z0-9_-]` segment is accepted. The `/spec.md` anchor is what keeps
//!   an `owner/repo` pair that looks like `NNN-slug` from false-matching.
//!
//! An unregistered repo is still harvested, with `service: null` — the
//! `unregistered` outcome surfaces later, at resolution time, rather than the
//! reference silently dropping from the index.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::primitives::spec_links::{
    continues_key, frontmatter_parses, harvestable_lines, has_unterminated_frontmatter,
    is_frontmatter_fence,
};
use crate::primitives::{Result, inline_code_spans, read_text, write_atomic};
use crate::schema::config_keys;
use crate::schema::paths;
use crate::schema::primitives::{DeriveReferencesArgs, DeriveReferencesResult};
use crate::schema::services::{self, Services};

/// One harvested reference: the resolved service alias (`None` for an
/// unregistered repo) and the referenced `NNN-slug`.
type Reference = (Option<String>, String);

/// A registered service as the matcher needs it: alias, plus the spec-root
/// segment to require (`None` when the checkout is unreachable).
struct RegisteredService {
    alias: String,
    specs_root: Option<String>,
}

/// Execute the `derive-references` primitive.
///
/// # Errors
///
/// Returns [`super::PrimitiveError::Io`] when a spec cannot be read or the
/// rewrite cannot be persisted, and [`super::PrimitiveError::Git`] when a
/// repository contains the project but its index or status cannot be read —
/// the listing of which specs to derive then has no answer, and guessing one
/// would rewrite drafts or skip specs silently.
pub fn run(args: &DeriveReferencesArgs, repo: &Path) -> Result<DeriveReferencesResult> {
    let specs_root = paths::Paths::load(repo).specs_root;
    let registry = load_registry(repo);

    // Every tracked spec is enumerated, exactly as `derive-dependencies` does,
    // and `--staged` filters only the *write*. The enumeration is deliberately
    // NOT narrowed to the staged set: a reference is derived from the body
    // **and** the `[services]` registry, so renaming a service alias drifts
    // every spec that references it while leaving those specs untouched — and
    // an untouched spec is never staged. Narrowing the walk made that entire
    // class structurally invisible to the pre-commit hook, for any number of
    // commits (scenario `derive-references-unstaged-drift-is-reported`).
    let tracked = super::list_tracked_specs(repo, &specs_root)?;
    let untracked = super::list_untracked_specs(repo, &specs_root)?;
    let staged = if args.staged {
        Some(super::list_staged_specs(repo, &specs_root)?)
    } else {
        None
    };

    let mut updated = Vec::new();
    let mut unwritten = Vec::new();
    let mut unparseable = Vec::new();
    let mut absent = Vec::new();
    let mut read_count: usize = 0;
    for spec in &tracked {
        let path = repo.join(spec);
        if !path.is_file() {
            // Tracked in the index but not on disk — deleted without the
            // deletion being staged. Nothing can be derived from it, and it is
            // neither drift nor "in sync", so it is named rather than dropped:
            // counting it in `examined` would assert a subject that was never
            // read (`QUAL-CLAIM-001`).
            absent.push(spec.clone());
            continue;
        }
        read_count += 1;
        let content = read_text(&path)?;
        if has_unterminated_frontmatter(&content) {
            unparseable.push(spec.clone());
            continue;
        }
        let records = harvest(&content, &registry);
        let rewritten = splice_references(&content, &records);
        if rewritten == content {
            continue;
        }
        if !frontmatter_parses(&rewritten) {
            // The splice's output is not YAML: a value shape its line rules
            // do not cover. Never written, and named as an unterminated block
            // is, before the staged filter, for `derive-dependencies`' reason.
            unparseable.push(spec.clone());
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

    Ok(DeriveReferencesResult {
        drift: !updated.is_empty(),
        updated,
        unwritten,
        examined: u32::try_from(read_count).unwrap_or(u32::MAX),
        untracked_skipped: untracked,
        unparseable,
        absent,
        registered_services: u32::try_from(registry.len()).unwrap_or(u32::MAX),
        specs_root,
        wrote: args.write,
    })
}

/// Build the repo→service registry, resolving each service's own spec-root
/// from its local checkout when that checkout is reachable.
///
/// A missing, unreadable, or unparseable config yields an empty registry —
/// every reference then resolves to `unregistered` and is still harvested,
/// rather than the run failing over an unrelated config error.
fn load_registry(repo: &Path) -> BTreeMap<String, RegisteredService> {
    let config_path = paths::config_path(repo);
    let Ok(content) = std::fs::read_to_string(&config_path) else {
        return BTreeMap::new();
    };
    config_keys::warn_unknown_keys(&config_path, &content, &["services"]);
    let Ok(services) = Services::from_toml_str(&content) else {
        return BTreeMap::new();
    };

    let mut registry = BTreeMap::new();
    for (alias, entry) in services.0 {
        if entry.repo.is_empty() {
            continue;
        }
        let checkout = if entry.path.is_empty() {
            None
        } else {
            let candidate = Path::new(&entry.path);
            let resolved = if candidate.is_absolute() {
                candidate.to_path_buf()
            } else {
                repo.join(candidate)
            };
            resolved.is_dir().then_some(resolved)
        };
        // Reachable checkout → require that checkout's own root. Unreachable
        // → `None`, the permissive tier.
        let specs_root = checkout.map(|dir| paths::Paths::load(&dir).specs_root);
        registry.insert(
            services::normalize_repo(&entry.repo),
            RegisteredService { alias, specs_root },
        );
    }
    registry
}

/// Drop a trailing `/blob/<ref>`, `/tree/<ref>`, or `/-/blob|tree/<ref>`
/// segment so branch-ref variations collapse to the same repo identity. The
/// branch is never part of a reference's identity.
fn strip_branch_ref(before: &str) -> &str {
    let trimmed = before.trim_end_matches('/');
    // Walk back over `<ref>`, then the `blob|tree` keyword, then an optional
    // GitLab-style `-` segment.
    let Some((head, last)) = trimmed.rsplit_once('/') else {
        return trimmed;
    };
    if last.is_empty() {
        return trimmed;
    }
    let Some((head2, keyword)) = head.rsplit_once('/') else {
        return trimmed;
    };
    if keyword != "blob" && keyword != "tree" {
        return trimmed;
    }
    match head2.rsplit_once('/') {
        Some((head3, "-")) => head3,
        _ => head2,
    }
}

/// Locate the `/<root>/NNN-slug/spec(-and-plan).md` segment in a URL.
///
/// Returns `(byte offset of the segment, root segment, slug)`. Leftmost match
/// wins, matching the shell's `match()` semantics. Content after `.md` (an
/// anchor, a query) is permitted and ignored.
fn find_spec_segment(url: &str) -> Option<(usize, String, String)> {
    for (idx, _) in url.char_indices().filter(|&(_, c)| c == '/') {
        let rest = &url[idx + 1..];
        // <root>/
        let root_end = rest.find('/')?;
        let root_seg = &rest[..root_end];
        if root_seg.is_empty()
            || !root_seg
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            continue;
        }
        let after_root = &rest[root_end + 1..];
        // NNN-slug/
        let Some(slug_end) = after_root.find('/') else {
            continue;
        };
        let slug = &after_root[..slug_end];
        if !is_spec_slug(slug) {
            continue;
        }
        let tail = &after_root[slug_end + 1..];
        // Longest filename alternative first, mirroring awk's leftmost-longest.
        if tail.starts_with("spec-and-plan.md") || tail.starts_with("spec.md") {
            return Some((idx, root_seg.to_string(), slug.to_string()));
        }
    }
    None
}

/// Whether `slug` names a feature directory in either form spec 051
/// defines.
///
/// Delegates to [`super::parse_feature_dir`] rather than carrying a second
/// copy of the grammar. This was an independent re-derivation of the same
/// exactly-three-digits rule `derive_dependencies` carried, with the same
/// consequence: a cross-service reference to a branch-scoped or four-digit
/// spec was silently not harvested.
///
/// The charset test is kept local for the reason `leading_slug` documents
/// at length: the shared grammar deliberately leaves a sequential
/// directory's trailing slug unconstrained, because it recognizes
/// directories that exist on disk — while this value is parsed out of a
/// link target and rendered verbatim as `spec: {slug}` into frontmatter.
fn is_spec_slug(slug: &str) -> bool {
    super::is_feature_slug(slug) && slug.split('.').all(super::is_slug_grammar)
}

/// Harvest the sorted, deduplicated cross-service references from a body.
fn harvest(content: &str, registry: &BTreeMap<String, RegisteredService>) -> Vec<Reference> {
    let mut seen: BTreeSet<Reference> = BTreeSet::new();
    for line in harvestable_lines(content) {
        let scrubbed = strip_inline_code(line.text);
        for url in absolute_link_urls(&scrubbed) {
            let Some((offset, root_seg, slug)) = find_spec_segment(url) else {
                continue;
            };
            let repo = services::normalize_repo(strip_branch_ref(&url[..offset]));
            let service = match registry.get(&repo) {
                Some(entry) => {
                    // Reachable checkout: only that service's real root
                    // matches. Unreachable: any segment.
                    if let Some(required) = &entry.specs_root
                        && *required != root_seg
                    {
                        continue;
                    }
                    Some(entry.alias.clone())
                }
                // Unregistered: root unknowable, accept any segment. Still
                // harvested, so it never silently drops from the index.
                None => None,
            };
            seen.insert((service, slug));
        }
    }
    // Sort by slug, then service — the shell's ordering.
    let mut records: Vec<Reference> = seen.into_iter().collect();
    records.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));
    records
}

/// Blank out inline-code spans so a backticked link cannot be harvested,
/// preserving byte offsets so nothing else shifts.
///
/// Borrows on the common path: most body lines carry no code span, and this
/// runs over every harvestable line of every spec on every commit.
fn strip_inline_code(line: &str) -> Cow<'_, str> {
    let spans = inline_code_spans(line);
    if spans.is_empty() {
        return Cow::Borrowed(line);
    }
    let mut out = line.as_bytes().to_vec();
    for span in spans {
        for byte in &mut out[span] {
            *byte = b' ';
        }
    }
    String::from_utf8(out).map_or(Cow::Borrowed(line), Cow::Owned)
}

/// Every `](<absolute-url>)` target on a line.
fn absolute_link_urls(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut cursor = 0;
    while let Some(pos) = line[cursor..].find("](") {
        cursor += pos + 2;
        let rest = &line[cursor..];
        if !rest.starts_with("http://") && !rest.starts_with("https://") {
            continue;
        }
        // Run of non-`)`, non-whitespace characters, which must terminate at
        // a closing paren for the link to be well-formed.
        let end = rest
            .find(|c: char| c == ')' || c.is_whitespace())
            .unwrap_or(rest.len());
        if rest.as_bytes().get(end) == Some(&b')') && end > 0 {
            out.push(&rest[..end]);
        }
    }
    out
}

/// Render the `references:` block, or `None` when there is nothing to write.
fn build_block(records: &[Reference]) -> Option<Vec<String>> {
    if records.is_empty() {
        return None;
    }
    let mut lines = vec!["references:".to_string()];
    for (service, slug) in records {
        match service {
            Some(alias) => lines.push(format!("  - service: {alias}")),
            None => lines.push("  - service: null".to_string()),
        }
        lines.push(format!("    spec: {slug}"));
    }
    Some(lines)
}

/// Strip any existing `references:` block from the frontmatter and re-insert
/// the desired one after the `dependencies:` entry, or before the closing
/// fence when there is no `dependencies:` line.
///
/// **After the entry, not after its key line.** A block-form value
/// (`dependencies:` then `  - 002-b`, or the indentless `- 002-b`) continues
/// on the lines below the key, and inserting straight after the key line put
/// `references:` between the key and its items: `dependencies:` became null
/// and each dependency an item of the references list, which still parses,
/// so nothing downstream noticed. The block goes before the first line that
/// is not a continuation ([`continues_key`], which counts a blank line an
/// entry follows as one), the same rule that decides where a stale
/// `references:` block ends.
///
/// Absent-when-empty: with no records the block is removed and not replaced.
fn splice_references(content: &str, records: &[Reference]) -> String {
    let block = build_block(records);
    let line_ending = super::line_ending_of(content);

    let mut out: Vec<String> = Vec::new();
    let mut fm_seen = false;
    let mut in_fm = false;
    let mut skipping = false;
    let mut inserted = false;
    // Inside the `dependencies:` entry, waiting for its last continuation
    // line to pass before the block goes in.
    let mut pending = false;
    let lines: Vec<&str> = content
        .lines()
        .map(|raw| raw.strip_suffix('\r').unwrap_or(raw))
        .collect();

    for (idx, &line) in lines.iter().enumerate() {
        if is_frontmatter_fence(line) {
            if !fm_seen && idx == 0 {
                fm_seen = true;
                in_fm = true;
                out.push(line.to_string());
                continue;
            }
            if in_fm {
                // Closing fence: last chance to place the block.
                if let Some(block_lines) = &block
                    && !inserted
                {
                    out.extend(block_lines.iter().cloned());
                    inserted = true;
                }
                in_fm = false;
                skipping = false;
                pending = false;
                out.push(line.to_string());
                continue;
            }
            out.push(line.to_string());
            continue;
        }

        if in_fm {
            if skipping {
                if continues_key(&lines, idx) {
                    continue;
                }
                skipping = false;
            }
            if pending {
                if continues_key(&lines, idx) {
                    out.push(line.to_string());
                    continue;
                }
                // The `dependencies:` entry ended on the line above.
                if let Some(block_lines) = &block {
                    out.extend(block_lines.iter().cloned());
                }
                inserted = true;
                pending = false;
            }
            if line.starts_with("references:") {
                skipping = true;
                continue;
            }
            if !inserted && block.is_some() && line.starts_with("dependencies:") {
                out.push(line.to_string());
                pending = true;
                continue;
            }
        }
        out.push(line.to_string());
    }

    let mut joined = out.join(line_ending);
    if content.ends_with('\n') {
        joined.push_str(line_ending);
    }
    joined
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn registry(entries: &[(&str, &str, Option<&str>)]) -> BTreeMap<String, RegisteredService> {
        entries
            .iter()
            .map(|(repo, alias, root)| {
                (
                    services::normalize_repo(repo),
                    RegisteredService {
                        alias: (*alias).to_string(),
                        specs_root: root.map(str::to_string),
                    },
                )
            })
            .collect()
    }

    fn spec(body: &str) -> String {
        format!("---\nstatus: done\ndependencies: []\n---\n\n{body}\n")
    }

    /// [`crate::primitives::derive_dependencies`]'s subdirectory case, for
    /// the other generator on the same listings: with the project in a
    /// subdirectory of its repository, `--write` rewrote the untracked draft,
    /// nothing was reported untracked, and `--staged` rewrote nothing
    /// (spec 059).
    #[test]
    fn a_subdirectory_project_derives_from_its_index() {
        use crate::primitives::git_fixture;
        const LINK: &str = "[x](https://github.com/other/t/specs/003-user/spec.md)";
        let tmp = tempfile::tempdir().unwrap();
        let (repository, project) = git_fixture::subdirectory_project(tmp.path());
        let a = project.join("specs/001-a/spec.md");
        let b = project.join("specs/002-b/spec.md");
        git_fixture::write(&a, &spec("nothing"));
        git_fixture::write(&b, &spec("nothing"));
        git_fixture::commit_all(&repository, "feat: specs");
        let draft_path = project.join("specs/004-draft/spec.md");
        let draft = spec(LINK);
        git_fixture::write(&draft_path, &draft);

        let write = DeriveReferencesArgs {
            write: true,
            staged: false,
        };
        let result = run(&write, &project).unwrap();
        assert_eq!(std::fs::read_to_string(&draft_path).unwrap(), draft);
        assert_eq!(result.examined, 2);
        assert_eq!(result.untracked_skipped, vec!["specs/004-draft/spec.md"]);

        // Both tracked specs drift; only the first is staged.
        git_fixture::write(&a, &spec(LINK));
        git_fixture::write(&b, &spec(LINK));
        let mut index = repository.index().unwrap();
        index
            .add_path(Path::new("proj/specs/001-a/spec.md"))
            .unwrap();
        index.write().unwrap();
        let staged = DeriveReferencesArgs {
            write: true,
            staged: true,
        };
        let result = run(&staged, &project).unwrap();
        assert_eq!(result.updated, vec!["specs/001-a/spec.md"]);
        assert_eq!(result.unwritten, vec!["specs/002-b/spec.md"]);
        assert!(std::fs::read_to_string(&a).unwrap().contains("references:"));
        assert!(!std::fs::read_to_string(&b).unwrap().contains("references:"));
    }

    #[test]
    fn normalizes_trailing_slash_and_git_suffix() {
        assert_eq!(services::normalize_repo("https://h/o/r/"), "https://h/o/r");
        assert_eq!(
            services::normalize_repo("https://h/o/r.git"),
            "https://h/o/r"
        );
        assert_eq!(services::normalize_repo("https://h/o/r"), "https://h/o/r");
    }

    #[test]
    fn strips_branch_refs_in_all_three_shapes() {
        assert_eq!(strip_branch_ref("https://h/o/r/blob/main"), "https://h/o/r");
        assert_eq!(strip_branch_ref("https://h/o/r/tree/v2"), "https://h/o/r");
        assert_eq!(
            strip_branch_ref("https://h/o/r/-/blob/feature-x"),
            "https://h/o/r"
        );
        // Not a branch segment: left alone.
        assert_eq!(strip_branch_ref("https://h/o/r"), "https://h/o/r");
        assert_eq!(strip_branch_ref("https://h/o/r/docs"), "https://h/o/r/docs");
    }

    #[test]
    fn a_branch_ref_does_not_change_identity() {
        let reg = registry(&[("https://h/o/api", "api", None)]);
        let with_branch = spec("[x](https://h/o/api/blob/main/specs/003-user/spec.md)");
        let without = spec("[x](https://h/o/api/specs/003-user/spec.md)");
        assert_eq!(harvest(&with_branch, &reg), harvest(&without, &reg));
        assert_eq!(
            harvest(&without, &reg),
            vec![(Some("api".to_string()), "003-user".to_string())]
        );
    }

    #[test]
    fn an_unregistered_repo_harvests_with_a_null_service() {
        let reg = registry(&[]);
        let content = spec("[x](https://h/o/other/specs/003-user/spec.md)");
        assert_eq!(
            harvest(&content, &reg),
            vec![(None, "003-user".to_string())]
        );
    }

    #[test]
    fn a_reachable_checkout_requires_its_own_spec_root() {
        // Registered with a known root of `governance`: a `specs/` URL for
        // that service is not a reference to it.
        let reg = registry(&[("https://h/o/api", "api", Some("governance"))]);
        let matching = spec("[x](https://h/o/api/governance/003-user/spec.md)");
        assert_eq!(
            harvest(&matching, &reg),
            vec![(Some("api".to_string()), "003-user".to_string())]
        );
        let wrong_root = spec("[x](https://h/o/api/specs/003-user/spec.md)");
        assert_eq!(
            harvest(&wrong_root, &reg),
            [] as [(Option<String>, String); 0]
        );
    }

    #[test]
    fn an_unreachable_checkout_accepts_any_root_segment() {
        let reg = registry(&[("https://h/o/api", "api", None)]);
        for root in ["specs", "governance", "anything_1"] {
            let content = spec(&format!("[x](https://h/o/api/{root}/003-user/spec.md)"));
            assert_eq!(
                harvest(&content, &reg),
                vec![(Some("api".to_string()), "003-user".to_string())],
                "root segment {root} should have matched"
            );
        }
    }

    #[test]
    fn a_backticked_link_is_an_example_not_a_reference() {
        let reg = registry(&[]);
        let content = spec("Use `[x](https://h/o/other/specs/003-user/spec.md)` in your body.");
        assert_eq!(harvest(&content, &reg), [] as [(Option<String>, String); 0]);
    }

    #[test]
    fn relative_sibling_links_are_never_references() {
        let reg = registry(&[]);
        let content = spec("[b](../002-b/spec.md) and [c](specs/003-c/spec.md)");
        assert_eq!(harvest(&content, &reg), [] as [(Option<String>, String); 0]);
    }

    #[test]
    fn an_owner_repo_pair_shaped_like_a_slug_does_not_false_match() {
        // The `/spec.md` anchor is what prevents this.
        let reg = registry(&[]);
        let content = spec("[x](https://h/o/123-thing/README.md)");
        assert_eq!(harvest(&content, &reg), [] as [(Option<String>, String); 0]);
    }

    #[test]
    fn spec_and_plan_is_matched_too() {
        let reg = registry(&[]);
        let content = spec("[x](https://h/o/other/specs/003-user/spec-and-plan.md)");
        assert_eq!(
            harvest(&content, &reg),
            vec![(None, "003-user".to_string())]
        );
    }

    #[test]
    fn an_anchor_after_the_filename_is_ignored() {
        let reg = registry(&[]);
        let content = spec("[x](https://h/o/other/specs/003-user/spec.md#goals)");
        assert_eq!(
            harvest(&content, &reg),
            vec![(None, "003-user".to_string())]
        );
    }

    #[test]
    fn records_sort_by_slug_then_service_and_dedupe() {
        let reg = registry(&[("https://h/o/api", "api", None)]);
        let content = spec(
            "[b](https://h/o/api/specs/004-b/spec.md) \
             [a](https://h/o/api/specs/003-a/spec.md) \
             [a again](https://h/o/api/blob/main/specs/003-a/spec.md)",
        );
        assert_eq!(
            harvest(&content, &reg),
            vec![
                (Some("api".to_string()), "003-a".to_string()),
                (Some("api".to_string()), "004-b".to_string()),
            ]
        );
    }

    #[test]
    fn block_is_inserted_after_dependencies() {
        let content = "---\nstatus: done\ndependencies: [001-a]\nnext-criterion: 2\n---\nbody\n";
        let out = splice_references(content, &[(Some("api".to_string()), "003-u".to_string())]);
        let expected = "---\nstatus: done\ndependencies: [001-a]\nreferences:\n  - service: api\n    spec: 003-u\nnext-criterion: 2\n---\nbody\n";
        assert_eq!(out, expected);
    }

    /// A block-form `dependencies:` keeps its items: the references block goes
    /// after the entry's last continuation line, never between the key and
    /// its items, for the indented and the column-zero form alike. The
    /// `dependencies:` lines stay byte-identical (spec 030).
    #[test]
    fn block_is_inserted_after_a_block_form_dependencies_entry() {
        let records = [(Some("api".to_string()), "003-u".to_string())];
        for items in ["  - 002-b\n  - 004-d\n", "- 002-b\n- 004-d\n"] {
            let content =
                format!("---\nstatus: done\ndependencies:\n{items}next-criterion: 2\n---\nbody\n");
            let expected = format!(
                "---\nstatus: done\ndependencies:\n{items}references:\n  - service: api\n    spec: 003-u\nnext-criterion: 2\n---\nbody\n"
            );
            assert_eq!(splice_references(&content, &records), expected, "{items:?}");
        }
        // The entry runs to the closing fence: the block goes just before it.
        let content = "---\nstatus: done\ndependencies:\n  - 002-b\n---\nbody\n";
        assert_eq!(
            splice_references(content, &records),
            "---\nstatus: done\ndependencies:\n  - 002-b\nreferences:\n  - service: api\n    spec: 003-u\n---\nbody\n"
        );
    }

    /// Blank lines and column-zero comments inside a block-form
    /// `dependencies:` belong to it: the references block goes after the last
    /// entry, never between the key and the items below such a line, and a blank line before the next key
    /// stays before it. A stale block with a blank line inside is removed
    /// whole.
    #[test]
    fn blank_lines_inside_a_block_stay_with_their_key() {
        let records = [(Some("api".to_string()), "003-u".to_string())];
        let block = "references:\n  - service: api\n    spec: 003-u\n";
        for items in [
            "\n  - 002-b\n\n  - 004-d\n",
            "\n- 002-b\n\n\n- 004-d\n",
            "- 002-b\n# a note\n- 004-d\n",
        ] {
            let content = format!(
                "---\nstatus: done\ndependencies:\n{items}\nnext-criterion: 2\n---\nbody\n"
            );
            assert_eq!(
                splice_references(&content, &records),
                format!(
                    "---\nstatus: done\ndependencies:\n{items}{block}\nnext-criterion: 2\n---\nbody\n"
                ),
                "{items:?}"
            );
        }
        let stale = "---\nstatus: done\ndependencies: []\nreferences:\n\n  - service: null\n    spec: 003-old\n\nnext-criterion: 2\n---\nbody\n";
        assert_eq!(
            splice_references(stale, &records),
            format!("---\nstatus: done\ndependencies: []\n{block}\nnext-criterion: 2\n---\nbody\n")
        );
    }

    /// A stale block in the indentless form (`references:` then `- service:`
    /// at column zero) is removed whole; an indent-only skip left its entries
    /// behind the new block as invalid YAML.
    #[test]
    fn a_column_zero_references_block_is_replaced_whole() {
        let content = "---\nstatus: done\ndependencies: []\nreferences:\n- service: null\n  spec: 003-old\nnext-criterion: 2\n---\nbody\n";
        assert_eq!(
            splice_references(content, &[(None, "004-new".to_string())]),
            "---\nstatus: done\ndependencies: []\nreferences:\n  - service: null\n    spec: 004-new\nnext-criterion: 2\n---\nbody\n"
        );
        assert_eq!(
            splice_references(content, &[]),
            "---\nstatus: done\ndependencies: []\nnext-criterion: 2\n---\nbody\n"
        );
    }

    /// A stale block whose rewrite would not parse — a flow sequence closing
    /// at column zero, which the line rules do not cover — is reported
    /// `unparseable` and never written, on a dry run and a write alike.
    #[test]
    fn a_splice_that_would_not_parse_is_reported_and_never_written() {
        use crate::primitives::git_fixture;
        let tmp = tempfile::tempdir().unwrap();
        let repository = git2::Repository::init(tmp.path()).unwrap();
        let original = "---\nstatus: done\ndependencies: []\nreferences: [\n  {service: null, spec: 003-old}\n]\n---\n\n[x](https://github.com/other/t/specs/004-new/spec.md)\n";
        assert!(frontmatter_parses(original), "the fixture starts as YAML");
        let path = tmp.path().join("specs/001-a/spec.md");
        git_fixture::write(&path, original);
        git_fixture::commit_all(&repository, "feat: spec");
        for write in [false, true] {
            let result = run(
                &DeriveReferencesArgs {
                    write,
                    staged: false,
                },
                tmp.path(),
            )
            .unwrap();
            assert_eq!(
                result.unparseable,
                vec!["specs/001-a/spec.md"],
                "write {write}"
            );
            assert_eq!(result.updated, [] as [String; 0], "write {write}");
            assert!(!result.drift, "write {write}");
            assert_eq!(result.examined, 1, "write {write}");
            assert_eq!(
                std::fs::read_to_string(&path).unwrap(),
                original,
                "write {write}"
            );
        }
    }

    #[test]
    fn block_goes_before_the_closing_fence_when_there_is_no_dependencies_key() {
        let content = "---\nstatus: done\n---\nbody\n";
        let out = splice_references(content, &[(None, "003-u".to_string())]);
        let expected =
            "---\nstatus: done\nreferences:\n  - service: null\n    spec: 003-u\n---\nbody\n";
        assert_eq!(out, expected);
    }

    #[test]
    fn an_empty_derivation_removes_a_stale_block_entirely() {
        let content = "---\nstatus: done\ndependencies: []\nreferences:\n  - service: api\n    spec: 003-u\nnext-criterion: 2\n---\nbody\n";
        let out = splice_references(content, &[]);
        assert_eq!(
            out,
            "---\nstatus: done\ndependencies: []\nnext-criterion: 2\n---\nbody\n"
        );
        assert!(!out.contains("references:"), "stale block survived");
    }

    #[test]
    fn an_empty_derivation_on_a_spec_without_a_block_is_a_no_op() {
        let content = "---\nstatus: done\ndependencies: []\n---\nbody\n";
        assert_eq!(splice_references(content, &[]), content);
    }

    #[test]
    fn splice_is_idempotent() {
        let records = vec![(Some("api".to_string()), "003-u".to_string())];
        let content = "---\nstatus: done\ndependencies: []\n---\nbody\n";
        let once = splice_references(content, &records);
        assert_eq!(splice_references(&once, &records), once);
    }

    #[test]
    fn splice_only_touches_frontmatter() {
        let content = "---\nstatus: done\ndependencies: []\n---\nreferences: not frontmatter\n";
        let out = splice_references(content, &[]);
        assert!(out.contains("references: not frontmatter"));
    }

    #[test]
    fn splice_preserves_crlf() {
        let content = "---\r\nstatus: done\r\ndependencies: []\r\n---\r\nbody\r\n";
        let out = splice_references(content, &[(None, "003-u".to_string())]);
        assert!(out.contains("references:\r\n"));
        assert!(!out.contains("references:\n\n"));
    }

    #[test]
    fn the_shared_exclusions_apply() {
        let reg = registry(&[]);
        let content = spec(
            "```\n[fenced](https://h/o/other/specs/003-a/spec.md)\n```\n\n\
             > [quoted](https://h/o/other/specs/004-b/spec.md)\n\n\
             ## See also\n\n[nav](https://h/o/other/specs/005-c/spec.md)\n",
        );
        assert_eq!(harvest(&content, &reg), [] as [(Option<String>, String); 0]);
    }
}
