//! `append-inbox` — append one bullet to `{specs-root}/inbox.md`.
//!
//! The single deterministic surface behind `/ductus:log`, the inbox's only
//! producer (spec 022, scenario scaffolding-primitives). It once also served
//! `/ductus:implement`'s auto-capture and the bootstrap security audit, whose
//! dedup-by-prefix guard it carried; spec 058 removed both writers, and the
//! guard with them.
//!
//! Creation: when `inbox.md` is missing, the file is created from the
//! project inbox template at `framework/templates/project/inbox.md` when
//! that file exists (the ductus source repo, where project templates
//! live), else from a bare `# Inbox` heading. Adopter repos don't carry
//! `framework/templates/project/` — their `inbox.md` was scaffolded at
//! adoption — so the heading fallback is the common adopter-side create.
//!
//! Form: entries are written as checkboxes — `- [ ] {text}` — matching the
//! constitution's §bug-handling ("one a person logs stays a checkbox in
//! `specs/inbox.md` until done, then is removed"). A caller-supplied leading marker is stripped so it cannot
//! double, and removal strips the checkbox marker too, so both forms compare
//! by content.
//!
//! Counting is comment/fence-aware: an older inbox's `<!-- Rules: … -->`
//! guidance embeds `- ` lines that are not items, so `item-count` uses the
//! shared bullet grammar rather than counting list markers.

use std::path::Path;

use crate::primitives::{
    PrimitiveError, Result, SkipScanner, bullet_text, count_inbox_bullets, rel_path,
    strip_bullet_marker, write_atomic,
};
use crate::schema::paths;
use crate::schema::primitives::{AppendInboxArgs, AppendInboxResult};

/// Fallback content for a freshly-created inbox when no project template
/// exists on disk (the adopter-side create).
const FALLBACK_HEADING: &str = "# Inbox\n\n";

/// Repo-relative path of the project inbox template (framework source
/// layout only; see module docs).
const PROJECT_TEMPLATE: &str = "framework/templates/project/inbox.md";

/// Execute the `append-inbox` primitive against the given repo root.
///
/// # Errors
///
/// Returns [`PrimitiveError::InvalidArgument`] when `text` is empty,
/// whitespace-only, or carries an embedded newline (structure injection
/// into `inbox.md`) — the rule `remove-inbox-item` applies too, so every
/// bullet written here can be removed (see [`super::validate_inbox_text`]).
/// Filesystem failures surface as [`PrimitiveError::Io`].
pub fn run(args: &AppendInboxArgs, repo: &Path) -> Result<AppendInboxResult> {
    super::validate_inbox_text("append-inbox", "text", &args.text)?;

    let root = paths::Paths::load(repo).specs_root;
    let inbox_path = repo.join(&root).join("inbox.md");

    let (existing, created) = match std::fs::read_to_string(&inbox_path) {
        Ok(text) => (text, false),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => (creation_base(repo), true),
        Err(source) => {
            return Err(PrimitiveError::Io {
                path: inbox_path,
                source,
            });
        }
    };

    let new_content = append_bullet(&existing, &strip_bullet_marker(&args.text));
    write_atomic(&inbox_path, &new_content)?;

    Ok(AppendInboxResult {
        path: rel_path(&inbox_path, repo),
        created,
        item_count: count_inbox_bullets(&new_content),
    })
}

/// Base content for a freshly-created inbox: the project template's
/// content when it exists on disk, else the bare heading.
fn creation_base(repo: &Path) -> String {
    std::fs::read_to_string(repo.join(PROJECT_TEMPLATE))
        .unwrap_or_else(|_| FALLBACK_HEADING.to_string())
}

/// Append `- [ ] {text}` (the checkbox inbox form) to `content` at a
/// position the comment/fence-aware read side ([`super::iter_bullets`] /
/// [`count_inbox_bullets`]) will count.
///
/// The write side must agree with the read side about what counts as inbox
/// content: were the bullet appended blindly at EOF and `inbox.md` ended
/// inside an *unterminated* `<!--` comment or ` ``` ` fence, the new bullet
/// would land inside that skipped region — invisible to the reader and
/// undercounted. So a trailing unterminated region is split off first and the
/// bullet is inserted before it; the region is preserved after. A well-formed
/// inbox (every comment and fence closed) has no such region and appends at
/// the end exactly as before.
fn append_bullet(content: &str, text: &str) -> String {
    let bullet = format!("- [ ] {text}");
    let boundary = trailing_unterminated_offset(content);
    if boundary >= content.len() {
        return append_after(content, &bullet);
    }
    // `content[..boundary]` is the balanced prefix (it ends at the start of
    // the unterminated region's opener line); `content[boundary..]` is the
    // region that runs to EOF. Insert the bullet into the prefix, then
    // re-attach the region after a blank-line separator.
    let head = append_after(&content[..boundary], &bullet);
    format!("{head}\n{tail}", tail = &content[boundary..])
}

/// Append `bullet` after `content`: a single newline joins onto an existing
/// bullet run, a blank line separates the bullet from any non-list tail
/// (markdownlint's lists-surrounded-by-blanks rule). Output ends with exactly
/// one trailing newline. Operates on the balanced prefix only — comment/fence
/// awareness lives in [`append_bullet`].
fn append_after(content: &str, bullet: &str) -> String {
    // `trimmed` carries the file's own endings while the separator and the
    // bullet below are written with `\n`, so without this the appended line
    // would disagree with every line above it. Normalizing the whole result
    // to the file's ending is what keeps an append from producing a mixed
    // file (spec 051, scenario `rewrites-preserve-line-endings`).
    let ending = super::line_ending_of(content);
    let trimmed = content.trim_end_matches(['\n', '\r']);
    if trimmed.is_empty() {
        return super::with_line_ending(&format!("{bullet}\n"), ending);
    }
    let last_line = trimmed.lines().last().unwrap_or("");
    let sep = if bullet_text(last_line).is_some() {
        "\n"
    } else {
        "\n\n"
    };
    super::with_line_ending(&format!("{trimmed}{sep}{bullet}\n"), ending)
}

/// Byte offset at which a trailing *unterminated* HTML comment or fenced code
/// block begins — the region the comment/fence-aware read side would skip
/// through to EOF. Returns `content.len()` when the document is well-formed
/// (every comment and fence closed), so an append lands at the end.
fn trailing_unterminated_offset(content: &str) -> usize {
    let mut skip = SkipScanner::default();
    let mut region_start: Option<usize> = None;
    let mut offset = 0usize;
    for raw in content.split_inclusive('\n') {
        // Feed `skip` the newline-stripped line, matching how `iter_bullets`
        // drives the scanner via `content.lines()`.
        let line = raw.strip_suffix('\n').unwrap_or(raw);
        let line = line.strip_suffix('\r').unwrap_or(line);
        let was_in_region = skip.in_region();
        skip.skip(line);
        match (was_in_region, skip.in_region()) {
            (false, true) => region_start = Some(offset),
            (true, false) => region_start = None,
            _ => {}
        }
        offset += raw.len();
    }
    region_start.unwrap_or(content.len())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn args(text: &str) -> AppendInboxArgs {
        AppendInboxArgs { text: text.into() }
    }

    fn read_inbox(repo: &Path) -> String {
        fs::read_to_string(repo.join("specs/inbox.md")).unwrap()
    }

    #[test]
    fn appends_bullet_to_existing_inbox() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(
            tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n- [ ] first item\n",
        )
        .unwrap();
        let result = run(&args("second item"), tmp.path()).unwrap();
        assert_eq!(result.path, "specs/inbox.md");
        assert!(!result.created);
        assert_eq!(result.item_count, 2);
        assert_eq!(
            read_inbox(tmp.path()),
            "# Inbox\n\n- [ ] first item\n- [ ] second item\n"
        );
    }

    #[test]
    fn creates_missing_inbox_with_heading_fallback() {
        let tmp = tempdir().unwrap();
        let result = run(&args("first item"), tmp.path()).unwrap();
        assert!(result.created);
        assert_eq!(result.item_count, 1);
        assert_eq!(read_inbox(tmp.path()), "# Inbox\n\n- [ ] first item\n");
    }

    #[test]
    fn creates_missing_inbox_from_project_template_when_present() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("framework/templates/project")).unwrap();
        fs::write(
            tmp.path().join("framework/templates/project/inbox.md"),
            "# Inbox\n\nCapture queue prose.\n\n<!-- Rules -->\n",
        )
        .unwrap();
        let result = run(&args("first item"), tmp.path()).unwrap();
        assert!(result.created);
        assert_eq!(
            read_inbox(tmp.path()),
            "# Inbox\n\nCapture queue prose.\n\n<!-- Rules -->\n\n- [ ] first item\n"
        );
    }

    #[test]
    fn item_count_ignores_bullets_inside_comment_regions() {
        // An older inbox embeds `- ` lines inside a multi-line
        // `<!-- Rules: … -->` comment; those are not real items. Only the
        // one real appended bullet is counted.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(
            tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n<!-- Rules:\n     - not an item\n     - also not an item\n-->\n",
        )
        .unwrap();
        let result = run(&args("real item"), tmp.path()).unwrap();
        assert_eq!(result.item_count, 1, "comment bullets must not be counted");
    }

    #[test]
    fn appends_before_a_trailing_unterminated_comment() {
        // scenarios/append-inbox-comment-aware-write.md: an inbox that ends
        // inside an unclosed `<!--` comment must not swallow the new bullet.
        // It lands before the comment, so the comment/fence-aware read side
        // counts it (the write side agrees with the read side).
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(
            tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n- [ ] first\n<!-- dangling note, never closed\nmore comment text\n",
        )
        .unwrap();
        let result = run(&args("second"), tmp.path()).unwrap();
        assert_eq!(
            result.item_count, 2,
            "the appended bullet must be counted, not swallowed by the comment"
        );
        let content = read_inbox(tmp.path());
        let bullet_pos = content.find("- [ ] second").expect("bullet written");
        let comment_pos = content.find("<!-- dangling").expect("comment preserved");
        assert!(
            bullet_pos < comment_pos,
            "bullet must precede the unterminated comment:\n{content}"
        );
        assert_eq!(
            count_inbox_bullets(&content),
            2,
            "read side counts both bullets"
        );
    }

    #[test]
    fn appends_before_a_trailing_unterminated_fence() {
        // The fence counterpart to the unterminated-comment case: a bullet
        // appended to an inbox ending in an unclosed ``` fence lands before
        // the fence, where the read side counts it.
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(
            tmp.path().join("specs/inbox.md"),
            "# Inbox\n\n- [ ] first\n```\n- [ ] fenced, not an item\n",
        )
        .unwrap();
        let result = run(&args("second"), tmp.path()).unwrap();
        assert_eq!(result.item_count, 2);
        let content = read_inbox(tmp.path());
        assert!(
            content.find("- [ ] second").unwrap() < content.find("```").unwrap(),
            "bullet must precede the unterminated fence:\n{content}"
        );
    }

    #[test]
    fn blank_line_separates_bullet_from_non_list_tail() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(tmp.path().join("specs/inbox.md"), "# Inbox\n").unwrap();
        run(&args("item"), tmp.path()).unwrap();
        assert_eq!(read_inbox(tmp.path()), "# Inbox\n\n- [ ] item\n");
    }

    #[test]
    fn caller_supplied_marker_does_not_double() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();
        fs::write(tmp.path().join("specs/inbox.md"), "# Inbox\n").unwrap();
        run(&args("- [ ] SEC-BE-014: token logging"), tmp.path()).unwrap();
        assert_eq!(
            read_inbox(tmp.path()),
            "# Inbox\n\n- [ ] SEC-BE-014: token logging\n",
            "caller-supplied marker must not double"
        );
    }

    #[test]
    fn rejects_empty_and_multiline_text() {
        let tmp = tempdir().unwrap();
        for bad in ["", "   ", "line one\nline two", "cr\rline"] {
            let err = run(&args(bad), tmp.path()).unwrap_err();
            assert!(
                matches!(err, PrimitiveError::InvalidArgument { .. }),
                "expected InvalidArgument for {bad:?}"
            );
        }
        assert!(!tmp.path().join("specs/inbox.md").exists());
    }

    #[test]
    fn honors_configured_specs_root() {
        let tmp = tempdir().unwrap();
        fs::write(
            tmp.path().join(".govern.toml"),
            "[paths]\nspecs-root = \"governance\"\n",
        )
        .unwrap();
        let result = run(&args("routed item"), tmp.path()).unwrap();
        assert_eq!(result.path, "governance/inbox.md");
        assert!(tmp.path().join("governance/inbox.md").is_file());
        assert!(!tmp.path().join("specs").exists());
    }
}
