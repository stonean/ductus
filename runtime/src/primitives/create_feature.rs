//! `create-feature` — scaffold the next `{specs-root}/{NNN-slug}/`
//! feature directory with a spec-template copy.
//!
//! The deterministic scaffold step of `/ductus:specify` (spec 022, scenario
//! scaffolding-primitives): compute the next feature number, derive the
//! kebab-case slug from the title, create the directory, and copy the
//! spec template into it — atomic and mode-preserving. The LLM fills the
//! spec body afterwards via `writeSpecBody`.
//!
//! Template resolution mirrors `interpreter::payload::load_template`
//! (the `writeSpecBody` request builder): try the installed adopter
//! layout `{specs-root}/templates/spec.md` first, then the framework
//! source layout `framework/templates/spec/spec.md`. The copy goes
//! through [`crate::primitives::write_atomic_bytes`] plus
//! [`crate::primitives::apply_manifest::mirror_source_mode`] — the
//! atomic-write helper lands files at tempfile mode `0600`, so the
//! template's mode is re-applied explicitly (AGENTS.md gotcha,
//! 2026-06-30).

use std::path::Path;

use crate::primitives::apply_manifest::mirror_source_mode;
use crate::primitives::{
    FeatureForm, PrimitiveError, Result, list_feature_dirs, parse_feature_dir, resolve_template,
    write_atomic_bytes,
};
use crate::schema::paths;
use crate::schema::primitives::{CreateFeatureArgs, CreateFeatureResult};

/// Execute the `create-feature` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidArgument`] when `title` derives to an
/// empty slug, [`PrimitiveError::TemplateNotFound`] when no spec template
/// exists at either candidate location, or [`PrimitiveError::Io`] for
/// filesystem failures. An already-existing target directory is the
/// `created: false` **domain outcome** (no overwrite path), not an error.
pub fn run(args: &CreateFeatureArgs, repo: &Path) -> Result<CreateFeatureResult> {
    let slug = derive_slug(&args.title);
    if slug.is_empty() {
        return Err(PrimitiveError::InvalidArgument {
            primitive: "create-feature".into(),
            argument: "title".into(),
            reason: "title derives to an empty slug (no ASCII alphanumeric characters)".into(),
        });
    }
    let branch = resolve_branch_scope(args)?;

    let root = paths::Paths::load(repo).specs_root;
    let specs_dir = repo.join(&root);
    let feature = match &branch {
        None => format!("{:03}-{slug}", next_feature_number(&specs_dir)),
        Some(scope) => {
            let n = next_branch_number(&specs_dir, &scope.identifier);
            format!("{}.{n}-{slug}", scope.identifier)
        }
    };
    let feature_dir = specs_dir.join(&feature);
    let rel_dir = format!("{root}/{feature}");
    let identifier = branch.as_ref().map(|scope| scope.identifier.clone());

    // Refusal domain outcome: the derived directory already exists.
    // The counters make this unreachable for well-formed spec roots (max
    // + 1 exceeds every existing number under the same scheme), but a
    // racing writer or a hand-created directory must never be
    // overwritten. It is also what makes two contributors creating under
    // one identifier at the same moment safe: both compute the same `n`,
    // and the loser is refused rather than clobbering the winner.
    if feature_dir.exists() {
        return Ok(CreateFeatureResult {
            created: false,
            feature,
            path: rel_dir,
            template: None,
            identifier,
        });
    }

    // Resolve the template before creating anything, so a missing
    // template leaves no half-scaffolded directory behind.
    let (template_rel, template_abs) = resolve_template(repo, &root, "spec.md")?;
    let template_bytes = std::fs::read(&template_abs).map_err(|source| PrimitiveError::Io {
        path: template_abs.clone(),
        source,
    })?;
    // Stamp the fold target into the template *before* anything is
    // written, so a template that cannot carry the key is refused with
    // nothing on disk rather than leaving a branch-scoped spec whose
    // upstream home was silently dropped.
    let spec_bytes = match branch.as_ref() {
        None => template_bytes,
        Some(scope) => stamp_fold_target(&template_bytes, &scope.fold_into, &template_rel)?,
    };

    std::fs::create_dir_all(&feature_dir).map_err(|source| PrimitiveError::Io {
        path: feature_dir.clone(),
        source,
    })?;
    let dest = feature_dir.join("spec.md");
    write_atomic_bytes(&dest, &spec_bytes)?;
    mirror_source_mode(&template_abs, &dest)?;

    Ok(CreateFeatureResult {
        created: true,
        feature,
        path: rel_dir,
        template: Some(template_rel),
        identifier,
    })
}

/// A validated branch-scoped creation request.
struct BranchScope {
    /// The sanitized identifier, already in the slug grammar.
    identifier: String,
    /// The upstream spec this spec folds back into.
    fold_into: String,
}

/// Validate the branch-scoped arguments as a group, or confirm that this
/// is an ordinary sequential creation.
///
/// `fold_into` is required with `branch_id`, and refused without it. A
/// branch-scoped spec exists in order to be folded: the number is what
/// keeps the merge clean, and the target is what makes the spec
/// actionable once it lands, so a branch-scoped spec naming no target is
/// not a case this framework has.
fn resolve_branch_scope(args: &CreateFeatureArgs) -> Result<Option<BranchScope>> {
    let invalid = |argument: &str, reason: String| PrimitiveError::InvalidArgument {
        primitive: "create-feature".into(),
        argument: argument.into(),
        reason,
    };

    let Some(raw) = args.branch_id.as_deref() else {
        // A fold target describes a branch-scoped spec, so it is
        // meaningless here: accepting and ignoring it would let a caller
        // believe one was recorded.
        if args.fold_into.is_some() {
            return Err(invalid(
                "fold-into",
                "fold-into requires branch-id: only a branch-scoped spec folds back".into(),
            ));
        }
        return Ok(None);
    };

    let identifier = derive_slug(raw);
    if identifier.is_empty() {
        return Err(invalid(
            "branch-id",
            format!(
                "branch-id {raw:?} sanitizes to an empty identifier \
                 (no ASCII alphanumeric characters)"
            ),
        ));
    }

    let Some(target) = args.fold_into.as_deref() else {
        return Err(invalid(
            "fold-into",
            "branch-scoped creation requires fold-into <feature>: the spec exists in order \
             to be folded into an upstream spec"
                .into(),
        ));
    };
    // Shape only. The target normally lives on the upstream branch and is
    // absent from this tree, so requiring it to resolve would refuse the
    // feature's normal case.
    if !matches!(
        parse_feature_dir(target),
        Some(FeatureForm::Sequential { .. })
    ) {
        return Err(invalid(
            "fold-into",
            format!(
                "fold-into {target:?} is not a sequential feature name (NNN-slug); a \
                 branch-scoped spec cannot fold into another branch-scoped spec"
            ),
        ));
    }

    Ok(Some(BranchScope {
        identifier,
        fold_into: target.to_string(),
    }))
}

/// Insert `folds-into: {target}` just above the frontmatter's closing
/// fence, mirroring how `label-criteria` inserts `next-criterion:`.
///
/// Refuses a template with no frontmatter block: the key has nowhere to
/// live, and silently dropping it would lose the branch-scoped spec's
/// only record of where it belongs.
fn stamp_fold_target(template: &[u8], target: &str, template_rel: &str) -> Result<Vec<u8>> {
    let malformed = || PrimitiveError::InvalidArgument {
        primitive: "create-feature".into(),
        argument: "fold-into".into(),
        reason: format!(
            "spec template {template_rel} has no frontmatter block, so folds-into cannot be \
             recorded"
        ),
    };
    let text = std::str::from_utf8(template).map_err(|_| malformed())?;
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let lines: Vec<&str> = text.split('\n').map(|l| l.trim_end_matches('\r')).collect();
    if lines.first() != Some(&"---") {
        return Err(malformed());
    }
    let close = lines
        .iter()
        .skip(1)
        .position(|l| *l == "---")
        .ok_or_else(malformed)?
        + 1;

    let mut out: Vec<String> = lines.into_iter().map(str::to_string).collect();
    out.insert(close, format!("folds-into: {target}"));
    Ok(out.join(newline).into_bytes())
}

/// The next `{n}` under one branch identifier: the max existing counter
/// for that identifier, plus one.
///
/// Scoped to the identifier, so counters under different identifiers are
/// independent and two branches cannot collide. The rule is `max + 1`,
/// the same one the sequential counter uses — which means a retired
/// number is reusable, accepted because fold-back re-points every in-repo
/// link before a directory is retired.
fn next_branch_number(specs_dir: &Path, identifier: &str) -> u32 {
    list_feature_dirs(specs_dir)
        .iter()
        .filter_map(|name| parse_feature_dir(name))
        .filter_map(|form| match form {
            FeatureForm::BranchScoped { identifier: id, n } if id == identifier => Some(n),
            _ => None,
        })
        .max()
        .unwrap_or(0)
        + 1
}

/// Derive the kebab-case directory slug from a feature title: every ASCII
/// alphanumeric character is lowercased and kept; every run of other
/// characters (spaces, punctuation, non-ASCII) collapses to a single
/// hyphen; leading and trailing hyphens are trimmed.
fn derive_slug(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut pending_hyphen = false;
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_hyphen && !out.is_empty() {
                out.push('-');
            }
            pending_hyphen = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_hyphen = true;
        }
    }
    out
}

/// Compute the next feature number: the max existing three-digit `NNN-`
/// prefix across feature directories, plus one. `1` when the spec root is
/// missing or holds no feature directories. Numbers past 999 render
/// four-digit (the `{:03}` pad only guarantees a minimum width).
///
/// Branch-scoped directories contribute nothing: they carry no sequential
/// number, so a spec root holding `050-a` and `1234.1-b` still yields
/// `051`. The counter is a property of the sequential form alone.
fn next_feature_number(specs_dir: &Path) -> u32 {
    list_feature_dirs(specs_dir)
        .iter()
        .filter_map(|name| parse_feature_dir(name))
        .filter_map(|form| form.sequential_number())
        .max()
        .unwrap_or(0)
        + 1
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const TEMPLATE: &str = "---\nstatus: draft\ndependencies: []\n---\n\n# {Feature}\n";

    fn seed_with_installed_template(repo: &Path) {
        fs::create_dir_all(repo.join("specs/templates")).unwrap();
        fs::write(repo.join("specs/templates/spec.md"), TEMPLATE).unwrap();
    }

    fn args(title: &str) -> CreateFeatureArgs {
        CreateFeatureArgs {
            title: title.into(),
            branch_id: None,
            fold_into: None,
        }
    }

    /// Branch-scoped creation against a stock upstream fold target.
    fn branch_args(title: &str, branch_id: &str) -> CreateFeatureArgs {
        CreateFeatureArgs {
            title: title.into(),
            branch_id: Some(branch_id.into()),
            fold_into: Some("022-upstream".into()),
        }
    }

    #[test]
    fn branch_scoped_creation_counts_within_its_identifier() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());

        let first = run(&branch_args("Staged change", "1234"), tmp.path()).unwrap();
        assert_eq!(first.feature, "1234.1-staged-change");
        assert_eq!(first.identifier.as_deref(), Some("1234"));

        let second = run(&branch_args("Another change", "1234"), tmp.path()).unwrap();
        assert_eq!(second.feature, "1234.2-another-change");

        // A different identifier starts its own counter rather than
        // continuing the first one.
        let other = run(&branch_args("Elsewhere", "5678"), tmp.path()).unwrap();
        assert_eq!(other.feature, "5678.1-elsewhere");
    }

    #[test]
    fn a_branch_scoped_only_root_still_starts_the_sequence_at_001() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        run(&branch_args("Staged change", "1234"), tmp.path()).unwrap();
        run(&branch_args("Another change", "5678"), tmp.path()).unwrap();

        // No sequential directory exists, and the branch-scoped ones
        // contribute nothing, so the sequence begins where it would in an
        // empty root — not at 1235 or 5679.
        let first = run(&args("First sequential"), tmp.path()).unwrap();
        assert_eq!(first.feature, "001-first-sequential");
    }

    /// The counter's doc comment always said numbers past 999 render
    /// four-digit, but `parse_feature_dir` demanded exactly three — so the
    /// 1000th spec was created with a name its own membership predicate
    /// rejected, making it invisible to every corpus reader. The formatter
    /// and the predicate have to agree, and this is where they meet.
    #[test]
    fn the_thousandth_spec_is_created_with_a_name_the_predicate_accepts() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        fs::create_dir_all(tmp.path().join("specs/999-last-three-digit")).unwrap();

        let next = run(&args("Thousandth spec"), tmp.path()).unwrap();
        assert_eq!(next.feature, "1000-thousandth-spec");

        // Visible to the corpus, which is the half the bug broke: the
        // directory existed but no reader could see it.
        assert!(
            list_feature_dirs(&tmp.path().join("specs")).contains(&next.feature),
            "the new directory must be enumerable"
        );
        assert_eq!(
            parse_feature_dir(&next.feature).and_then(|f| f.sequential_number()),
            Some(1000)
        );

        // And the counter keeps going from there rather than restarting.
        let after = run(&args("One after"), tmp.path()).unwrap();
        assert_eq!(after.feature, "1001-one-after");
    }

    #[test]
    fn branch_scoped_creation_leaves_the_sequential_counter_alone() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        fs::create_dir_all(tmp.path().join("specs/050-existing")).unwrap();

        run(&branch_args("Staged change", "1234"), tmp.path()).unwrap();

        // The branch-scoped directory contributes no sequential number,
        // so the next sequential spec is still 051 — not 1235.
        let next = run(&args("Next sequential"), tmp.path()).unwrap();
        assert_eq!(next.feature, "051-next-sequential");
    }

    #[test]
    fn branch_identifier_is_sanitized_and_reported() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());

        // Trackers disagree on shape; both forms are accepted as opaque
        // tokens and lowercased into the directory grammar.
        let jira = run(&branch_args("Ticket work", "PROJ-1111"), tmp.path()).unwrap();
        assert_eq!(jira.feature, "proj-1111.1-ticket-work");
        assert_eq!(jira.identifier.as_deref(), Some("proj-1111"));

        let gitlab = run(&branch_args("Item work", "1111-PROJ"), tmp.path()).unwrap();
        assert_eq!(gitlab.feature, "1111-proj.1-item-work");

        // A dot in the identifier collapses to a hyphen, so it can never
        // be confused with the `.{n}` delimiter.
        let dotted = run(&branch_args("Dotted", "proj.7"), tmp.path()).unwrap();
        assert_eq!(dotted.feature, "proj-7.1-dotted");
    }

    #[test]
    fn identifiers_differing_only_in_case_share_one_namespace() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());

        let first = run(&branch_args("First", "PROJ-1111"), tmp.path()).unwrap();
        assert_eq!(first.feature, "proj-1111.1-first");

        // Not a second namespace starting at .1 — the same one, continuing.
        let second = run(&branch_args("Second", "proj-1111"), tmp.path()).unwrap();
        assert_eq!(second.feature, "proj-1111.2-second");
    }

    #[test]
    fn an_existing_branch_scoped_path_is_refused_not_overwritten() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());

        // The refusal is unreachable through a hand-created *directory*:
        // the counter would see it and hand out the number after it. It
        // is reached the same way the sequential case is — the derived
        // path already exists as a plain file, which `exists()` reports
        // but `list_feature_dirs` does not count.
        fs::write(tmp.path().join("specs/1234.1-taken"), "not a dir\n").unwrap();

        let refused = run(&branch_args("Taken", "1234"), tmp.path()).unwrap();
        assert!(
            !refused.created,
            "refusal is a domain outcome, not an error"
        );
        assert_eq!(refused.feature, "1234.1-taken");
        assert!(refused.template.is_none());
        // The identifier is reported on the refusal too, so the caller
        // can name the namespace that collided.
        assert_eq!(refused.identifier.as_deref(), Some("1234"));
        let body = fs::read_to_string(tmp.path().join("specs/1234.1-taken")).unwrap();
        assert_eq!(body, "not a dir\n", "existing path untouched");
    }

    #[test]
    fn an_explicit_fold_target_is_stamped_into_the_frontmatter() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        let result = run(
            &CreateFeatureArgs {
                title: "Staged change".into(),
                branch_id: Some("1234".into()),
                fold_into: Some("022-deterministic-runtime".into()),
            },
            tmp.path(),
        )
        .unwrap();
        let spec = fs::read_to_string(
            tmp.path()
                .join("specs")
                .join(&result.feature)
                .join("spec.md"),
        )
        .unwrap();
        assert!(
            spec.contains("folds-into: 022-deterministic-runtime"),
            "fold target not recorded:\n{spec}"
        );
        // Inserted inside the frontmatter block, above the closing fence.
        let fm_end = spec.find("\n---\n").unwrap();
        assert!(spec.find("folds-into:").unwrap() < fm_end);
    }

    #[test]
    fn branch_scoped_creation_requires_a_fold_target() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        let err = run(
            &CreateFeatureArgs {
                title: "Staged change".into(),
                branch_id: Some("1234".into()),
                fold_into: None,
            },
            tmp.path(),
        )
        .unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "fold-into"),
            "expected a fold-into refusal, got {err:?}"
        );
        // Nothing was created: the refusal happens before any write.
        assert!(!tmp.path().join("specs/1234.1-staged-change").exists());
    }

    #[test]
    fn branch_scoped_argument_groups_are_validated() {
        let tmp = tempfile::tempdir().unwrap();
        seed_with_installed_template(tmp.path());

        let cases: [(CreateFeatureArgs, &str); 3] = [
            // An identifier with nothing alphanumeric in it.
            (
                CreateFeatureArgs {
                    title: "T".into(),
                    branch_id: Some("!!!".into()),
                    fold_into: Some("022-upstream".into()),
                },
                "branch-id",
            ),
            // A fold target that is not a sequential feature name: a
            // branch-scoped spec cannot fold into another one.
            (
                CreateFeatureArgs {
                    title: "T".into(),
                    branch_id: Some("1234".into()),
                    fold_into: Some("5678.1-other".into()),
                },
                "fold-into",
            ),
            // A fold target without branch-id describes nothing.
            (
                CreateFeatureArgs {
                    title: "T".into(),
                    branch_id: None,
                    fold_into: Some("022-upstream".into()),
                },
                "fold-into",
            ),
        ];

        for (case, expected) in cases {
            let err = run(&case, tmp.path()).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == expected),
                "expected {expected:?} refusal, got {err:?}"
            );
        }
    }

    #[test]
    fn creates_first_feature_as_001() {
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        let result = run(&args("Webhook Delivery"), tmp.path()).unwrap();
        assert!(result.created);
        assert_eq!(result.feature, "001-webhook-delivery");
        assert_eq!(result.path, "specs/001-webhook-delivery");
        assert_eq!(result.template.as_deref(), Some("specs/templates/spec.md"));
        let body =
            fs::read_to_string(tmp.path().join("specs/001-webhook-delivery/spec.md")).unwrap();
        assert_eq!(body, TEMPLATE, "spec.md is a byte copy of the template");
    }

    #[test]
    fn numbers_from_max_existing_prefix_plus_one() {
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        // Gap-tolerant: 003 and 007 exist → next is 008, not 004.
        for existing in ["003-a", "007-b"] {
            fs::create_dir_all(tmp.path().join("specs").join(existing)).unwrap();
        }
        let result = run(&args("next one"), tmp.path()).unwrap();
        assert_eq!(result.feature, "008-next-one");
    }

    #[test]
    fn non_feature_siblings_do_not_affect_numbering() {
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        fs::create_dir_all(tmp.path().join("specs/005-real")).unwrap();
        // `templates/` (already created), a stray file, and a dotdir must
        // not contribute prefixes.
        fs::write(tmp.path().join("specs/inbox.md"), "# Inbox\n").unwrap();
        fs::create_dir_all(tmp.path().join("specs/.cache")).unwrap();
        let result = run(&args("counted right"), tmp.path()).unwrap();
        assert_eq!(result.feature, "006-counted-right");
    }

    #[test]
    fn slug_derivation_collapses_and_trims() {
        assert_eq!(derive_slug("Webhook Delivery"), "webhook-delivery");
        assert_eq!(derive_slug("  Retry!!  Logic  "), "retry-logic");
        assert_eq!(derive_slug("API v2 (draft)"), "api-v2-draft");
        assert_eq!(derive_slug("already-kebab"), "already-kebab");
        assert_eq!(derive_slug("Café Menu"), "caf-menu");
        assert_eq!(derive_slug("!!!"), "");
    }

    #[test]
    fn rejects_title_with_no_alphanumerics() {
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        let err = run(&args("!!! ***"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidArgument { .. }));
    }

    #[test]
    fn falls_back_to_framework_source_template() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("framework/templates/spec")).unwrap();
        fs::write(
            tmp.path().join("framework/templates/spec/spec.md"),
            TEMPLATE,
        )
        .unwrap();
        let result = run(&args("fallback"), tmp.path()).unwrap();
        assert!(result.created);
        assert_eq!(
            result.template.as_deref(),
            Some("framework/templates/spec/spec.md")
        );
    }

    #[test]
    fn installed_template_wins_over_framework_source() {
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        fs::create_dir_all(tmp.path().join("framework/templates/spec")).unwrap();
        fs::write(
            tmp.path().join("framework/templates/spec/spec.md"),
            "# other\n",
        )
        .unwrap();
        let result = run(&args("ordered"), tmp.path()).unwrap();
        assert_eq!(result.template.as_deref(), Some("specs/templates/spec.md"));
        let body = fs::read_to_string(tmp.path().join("specs/001-ordered/spec.md")).unwrap();
        assert_eq!(body, TEMPLATE);
    }

    #[test]
    fn missing_template_errors_without_creating_the_directory() {
        let tmp = tempdir().unwrap();
        let err = run(&args("no template"), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::TemplateNotFound { .. }));
        assert!(
            !tmp.path().join("specs/001-no-template").exists(),
            "a missing template must not leave a half-scaffolded directory"
        );
    }

    #[test]
    fn never_touches_existing_feature_directories() {
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        fs::create_dir_all(tmp.path().join("specs/001-existing")).unwrap();
        fs::write(
            tmp.path().join("specs/001-existing/notes.md"),
            "hands off\n",
        )
        .unwrap();
        let result = run(&args("existing"), tmp.path()).unwrap();
        // max(001) + 1 → a fresh 002 dir; the 001 dir is untouched.
        assert!(result.created);
        assert_eq!(result.feature, "002-existing");
        let notes = fs::read_to_string(tmp.path().join("specs/001-existing/notes.md")).unwrap();
        assert_eq!(notes, "hands off\n");
    }

    #[test]
    fn refusal_branch_reports_domain_outcome() {
        // Directly cover the `feature_dir.exists()` refusal: a dir whose
        // name will be derived next already exists as a *file*-bearing
        // path. Achieved by making the target path exist as a plain file
        // (exists() is true for files too).
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        fs::write(tmp.path().join("specs/001-taken"), "not a dir\n").unwrap();
        let result = run(&args("taken"), tmp.path()).unwrap();
        assert!(!result.created, "refusal is a domain outcome, not an error");
        assert_eq!(result.feature, "001-taken");
        assert_eq!(result.path, "specs/001-taken");
        assert!(result.template.is_none());
        let body = fs::read_to_string(tmp.path().join("specs/001-taken")).unwrap();
        assert_eq!(body, "not a dir\n", "existing path untouched");
    }

    #[cfg(unix)]
    #[test]
    fn copied_spec_mirrors_template_mode() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempdir().unwrap();
        seed_with_installed_template(tmp.path());
        let template = tmp.path().join("specs/templates/spec.md");
        let mut perms = fs::metadata(&template).unwrap().permissions();
        perms.set_mode(0o644);
        fs::set_permissions(&template, perms).unwrap();

        run(&args("mode check"), tmp.path()).unwrap();
        let dest_mode = fs::metadata(tmp.path().join("specs/001-mode-check/spec.md"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(
            dest_mode, 0o644,
            "write_atomic_bytes lands 0600; the template mode must be mirrored"
        );
    }
}
