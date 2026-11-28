//! `register-service` — add one `[services.<alias>]` table to the project
//! config: `framework/commands/link.md`'s **Additive write**, which a host
//! performed with its file tools until spec 030's scenario
//! `registration-writes-through-a-primitive` made it a primitive, so a host
//! walking `/{project}:link` and `ductus exec link` make one write.
//!
//! The table is appended after the file's last byte, so every other byte
//! stays as it was, and the result is read back through the registry's one
//! parser before anything is written: a config the table would leave
//! unparseable, or that would not read back as the registry plus this entry,
//! is refused. The write is atomic and refused when the file changed since
//! the read (`BE-TXN-002`).
//!
//! The `path` is recorded as given, never run through
//! [`super::validate_no_traversal`]: this primitive transcribes an operator's
//! answer, which keeps `[services]` on the config-sourced side of the boundary
//! that function's doc states.

use std::fmt::Write as _;
use std::path::Path;

use crate::primitives::{
    PrimitiveError, ReadSnapshot, Result, guarded, rel_path, resolve_path, resolve_references,
};
use crate::schema::paths;
use crate::schema::primitives::{RegisterServiceArgs, RegisterServiceResult};
use crate::schema::services::{ServiceEntry, Services, normalize_repo};

const PRIMITIVE: &str = "register-service";

/// Execute the `register-service` primitive against the project root `repo`.
///
/// # Errors
///
/// [`PrimitiveError::InvalidArgument`] for an `alias`, `repo` or `path` the
/// registration flow rejects, an `alias` already registered among them;
/// [`PrimitiveError::Toml`] and [`PrimitiveError::InvalidServiceEntry`] for a
/// config the registry's reader refuses; [`PrimitiveError::ServicesNotExtensible`]
/// for a config the appended table would break; [`PrimitiveError::ChangedSinceRead`]
/// when the config changed between the read and the write. Nothing is written
/// in any of them.
pub fn run(args: &RegisterServiceArgs, repo: &Path) -> Result<RegisterServiceResult> {
    validate_alias(&args.alias)?;
    validate_repo(&args.repo)?;
    validate_path(&args.path)?;
    let description = args
        .description
        .as_deref()
        .filter(|text| !text.trim().is_empty());

    let config = paths::config_path_for_write(repo);
    let rel = rel_path(&config, repo);
    let snapshot = ReadSnapshot::read(repo, &rel)?;
    let existing = snapshot.text()?;
    let before = match existing {
        Some(text) => resolve_references::parse_services(&config, text)?,
        None => Services::default(),
    };
    if before.0.contains_key(&args.alias) {
        return Err(invalid(
            "alias",
            format!(
                "[services.{}] is already registered in {rel}: an existing entry is changed by hand",
                args.alias
            ),
        ));
    }

    let entry = ServiceEntry {
        repo: args.repo.clone(),
        path: args.path.clone(),
        description: description.map(str::to_string),
    };
    let ending = existing.map_or("\n", super::line_ending_of);
    let block = render_block(&args.alias, &entry, ending);
    let content = append(existing.unwrap_or(""), &block, ending);

    let mut expected = before.clone();
    expected.0.insert(args.alias.clone(), entry);
    match Services::from_toml_str(&content) {
        Ok(after) if after == expected => {}
        Ok(_) => {
            return Err(not_extensible(
                &rel,
                &args.alias,
                "the appended table does not read back as the registry plus this entry",
            ));
        }
        Err(err) => return Err(not_extensible(&rel, &args.alias, &err.to_string())),
    }

    let warnings = collect_warnings(repo, &before, args);
    guarded::write_guarded(&snapshot, &content)?;
    Ok(RegisterServiceResult {
        path: rel,
        created: existing.is_none(),
        block,
        warnings,
    })
}

/// The alias is a bare TOML key, so the table header needs no quoting: the
/// rule `link.md`'s **Validation** states.
fn validate_alias(alias: &str) -> Result<()> {
    if alias.is_empty() {
        return Err(invalid("alias", "is empty".to_string()));
    }
    if !alias
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(invalid(
            "alias",
            format!(
                "`{}` is not a bare TOML key: use letters, digits, `-` and `_` only",
                alias.escape_debug()
            ),
        ));
    }
    Ok(())
}

/// URL-shaped: a scheme, `://` and a host, with no whitespace or control
/// character anywhere. Recorded verbatim and never fetched.
fn validate_repo(repo: &str) -> Result<()> {
    let shaped = repo.split_once("://").is_some_and(|(scheme, rest)| {
        let mut chars = scheme.chars();
        let scheme_ok = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
            && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
        let host = rest.split(['/', '?', '#']).next().unwrap_or("");
        scheme_ok && !host.is_empty()
    });
    if !shaped || repo.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(invalid(
            "repo",
            format!(
                "`{}` is not URL-shaped: give a scheme and a host, such as https://github.com/acme/api",
                repo.escape_debug()
            ),
        ));
    }
    Ok(())
}

/// Every reader of the registry halts on an empty `path`, which would resolve
/// to this repository, so an empty one is refused here rather than written.
fn validate_path(path: &str) -> Result<()> {
    if path.trim().is_empty() {
        return Err(invalid(
            "path",
            "is empty: it names the service's local checkout, and an empty value resolves to \
             this repository"
                .to_string(),
        ));
    }
    Ok(())
}

/// The `[services.<alias>]` table, one line per key, each ending in `ending`.
fn render_block(alias: &str, entry: &ServiceEntry, ending: &str) -> String {
    let mut lines = vec![
        format!("[services.{alias}]"),
        format!("repo = {}", basic_string(&entry.repo)),
        format!("path = {}", basic_string(&entry.path)),
    ];
    if let Some(description) = &entry.description {
        lines.push(format!("description = {}", basic_string(description)));
    }
    super::join_lines_terminated(&lines, ending)
}

/// `existing` with `block` after it: a line ending closes an unterminated
/// last line, and a blank line separates the block from what came before,
/// unless the file already ends in one or holds nothing.
fn append(existing: &str, block: &str, ending: &str) -> String {
    let mut out = existing.to_string();
    if !out.is_empty() && !out.ends_with('\n') {
        out.push_str(ending);
    }
    let blank_already = out.trim_end_matches(['\r', '\n']).is_empty()
        || out.ends_with("\n\n")
        || out.ends_with("\n\r\n");
    if !blank_already {
        out.push_str(ending);
    }
    out.push_str(block);
    out
}

/// `value` as a one-line TOML basic string: `"` and `\` escaped, and every
/// control character written as an escape, so a value holding a quote, a
/// backslash or a line break reads back as given and the line stays one line,
/// whatever line ending the file uses.
fn basic_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04X}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The registration's warnings, which never refuse it: a repo another alias
/// already registers, compared as resolution compares it, and a path that
/// resolves to no directory, the valid `not-checked-out` state.
fn collect_warnings(repo: &Path, before: &Services, args: &RegisterServiceArgs) -> Vec<String> {
    let mut out = Vec::new();
    let identity = normalize_repo(&args.repo);
    let sharing: Vec<String> = before
        .0
        .iter()
        .filter(|(_, entry)| normalize_repo(&entry.repo) == identity)
        .map(|(alias, _)| format!("[services.{alias}]"))
        .collect();
    if !sharing.is_empty() {
        out.push(format!(
            "repo {} is already registered as {}: a reference to it then resolves through one \
             alias only, a duplicate repo the registry's readers report",
            args.repo,
            sharing.join(", ")
        ));
    }
    if !resolve_path(repo, &args.path).is_dir() {
        out.push(format!(
            "path {} resolves to no directory: references to the service read as not checked \
             out until it is cloned there",
            args.path.escape_debug()
        ));
    }
    out
}

fn invalid(argument: &str, reason: String) -> PrimitiveError {
    PrimitiveError::InvalidArgument {
        primitive: PRIMITIVE.to_string(),
        argument: argument.to_string(),
        reason,
    }
}

fn not_extensible(rel: &str, alias: &str, reason: &str) -> PrimitiveError {
    PrimitiveError::ServicesNotExtensible {
        path: rel.into(),
        alias: alias.to_string(),
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn args(alias: &str, repo: &str, path: &str, description: Option<&str>) -> RegisterServiceArgs {
        RegisterServiceArgs {
            alias: alias.into(),
            repo: repo.into(),
            path: path.into(),
            description: description.map(str::to_string),
        }
    }

    fn api() -> RegisterServiceArgs {
        args("api", "https://github.com/acme/api", "../api", None)
    }

    fn config(repo: &Path) -> String {
        fs::read_to_string(repo.join(".ductus/config.toml")).unwrap()
    }

    fn write_config(repo: &Path, body: &str) {
        fs::create_dir_all(repo.join(".ductus")).unwrap();
        fs::write(repo.join(".ductus/config.toml"), body).unwrap();
    }

    #[test]
    fn appends_the_table_and_keeps_every_other_byte() {
        let tmp = tempdir().unwrap();
        let before = "# Project config\n[paths]\nspecs-root = \"specs\"  # trailing comment\n\n[review]\nre-reads = 2\n";
        write_config(tmp.path(), before);
        let result = run(
            &args(
                "api",
                "https://github.com/acme/api",
                "../api",
                Some("owns the data models"),
            ),
            tmp.path(),
        )
        .unwrap();
        let block = "[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"../api\"\ndescription = \"owns the data models\"\n";
        assert_eq!(config(tmp.path()), format!("{before}\n{block}"));
        assert_eq!(result.block, block);
        assert_eq!(result.path, ".ductus/config.toml");
        assert!(!result.created);
    }

    #[test]
    fn creates_the_config_when_none_exists() {
        let tmp = tempdir().unwrap();
        let result = run(&api(), tmp.path()).unwrap();
        assert!(result.created);
        assert_eq!(
            config(tmp.path()),
            "[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"../api\"\n"
        );
    }

    /// A legacy file stays the active one: no partial `.ductus/config.toml`
    /// is created beside it (`link.md`'s **Additive write**).
    #[test]
    fn writes_the_legacy_file_when_only_it_exists() {
        let tmp = tempdir().unwrap();
        fs::write(
            tmp.path().join(".govern.toml"),
            "[paths]\nspecs-root = \"specs\"\n",
        )
        .unwrap();
        let result = run(&api(), tmp.path()).unwrap();
        assert_eq!(result.path, ".govern.toml");
        assert!(!tmp.path().join(".ductus/config.toml").exists());
        assert!(
            fs::read_to_string(tmp.path().join(".govern.toml"))
                .unwrap()
                .ends_with("\n\n[services.api]\nrepo = \"https://github.com/acme/api\"\npath = \"../api\"\n")
        );
    }

    #[test]
    fn an_unterminated_last_line_is_closed_before_the_blank_line() {
        let tmp = tempdir().unwrap();
        write_config(tmp.path(), "[paths]\nspecs-root = \"specs\"");
        run(&api(), tmp.path()).unwrap();
        assert!(
            config(tmp.path()).starts_with("[paths]\nspecs-root = \"specs\"\n\n[services.api]\n")
        );
    }

    #[test]
    fn a_file_ending_in_a_blank_line_gains_no_second_one() {
        let tmp = tempdir().unwrap();
        write_config(tmp.path(), "[paths]\nspecs-root = \"specs\"\n\n");
        run(&api(), tmp.path()).unwrap();
        assert!(
            config(tmp.path()).starts_with("[paths]\nspecs-root = \"specs\"\n\n[services.api]\n")
        );
    }

    #[test]
    fn a_crlf_config_gets_a_crlf_table() {
        let tmp = tempdir().unwrap();
        write_config(tmp.path(), "[paths]\r\nspecs-root = \"specs\"\r\n");
        let result = run(&api(), tmp.path()).unwrap();
        let block =
            "[services.api]\r\nrepo = \"https://github.com/acme/api\"\r\npath = \"../api\"\r\n";
        assert_eq!(result.block, block);
        assert_eq!(
            config(tmp.path()),
            format!("[paths]\r\nspecs-root = \"specs\"\r\n\r\n{block}")
        );
    }

    #[test]
    fn an_empty_description_writes_none() {
        let tmp = tempdir().unwrap();
        let result = run(
            &args("api", "https://github.com/acme/api", "../api", Some("  ")),
            tmp.path(),
        )
        .unwrap();
        assert!(!result.block.contains("description"), "{}", result.block);
    }

    #[test]
    fn a_registered_alias_is_refused_before_any_write() {
        let tmp = tempdir().unwrap();
        let before =
            "[services.api]\nrepo = \"https://github.com/acme/other\"\npath = \"../other\"\n";
        write_config(tmp.path(), before);
        let err = run(&api(), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == "alias"),
            "{err:?}"
        );
        assert_eq!(config(tmp.path()), before);
    }

    #[test]
    fn a_rejected_field_is_refused_naming_it() {
        let tmp = tempdir().unwrap();
        let cases = [
            (args("", "https://h/o/r", "../r", None), "alias"),
            (args("a.b", "https://h/o/r", "../r", None), "alias"),
            (args("a b", "https://h/o/r", "../r", None), "alias"),
            (args("\"api\"", "https://h/o/r", "../r", None), "alias"),
            (args("api", "github.com/acme/api", "../r", None), "repo"),
            (args("api", "https://", "../r", None), "repo"),
            (args("api", "://h/o/r", "../r", None), "repo"),
            (args("api", "1http://h/o/r", "../r", None), "repo"),
            (args("api", "https://h/o r", "../r", None), "repo"),
            (args("api", "https://h/o/r\n", "../r", None), "repo"),
            (args("api", "https://h/o/r", "", None), "path"),
            (args("api", "https://h/o/r", "  ", None), "path"),
        ];
        for (call, field) in cases {
            let err = run(&call, tmp.path()).unwrap_err();
            assert!(
                matches!(&err, PrimitiveError::InvalidArgument { argument, .. } if argument == field),
                "{call:?}: {err:?}"
            );
        }
        assert!(
            !tmp.path().join(".ductus/config.toml").exists(),
            "nothing written"
        );
    }

    #[test]
    fn url_shapes_the_flow_accepts_pass() {
        let tmp = tempdir().unwrap();
        for (alias, repo) in [
            ("a", "https://github.com/acme/api"),
            ("b", "ssh://git@github.com/acme/b.git"),
            ("c", "git+https://example.com/c"),
            ("d", "https://example.com"),
        ] {
            run(&args(alias, repo, "../x", None), tmp.path()).unwrap();
        }
    }

    #[test]
    fn values_are_escaped_and_read_back_as_given() {
        let tmp = tempdir().unwrap();
        let path = "C:\\checkouts\\\"api\"";
        let description = "line one\nline two\ttabbed\u{7}bell";
        run(
            &args("api", "https://h/o/api", path, Some(description)),
            tmp.path(),
        )
        .unwrap();
        let text = config(tmp.path());
        assert_eq!(text.lines().count(), 4, "one line per key: {text}");
        let read = Services::from_toml_str(&text).unwrap();
        let entry = &read.0["api"];
        assert_eq!(entry.path, path);
        assert_eq!(entry.description.as_deref(), Some(description));
    }

    #[test]
    fn a_config_that_does_not_parse_is_refused_naming_the_file() {
        let tmp = tempdir().unwrap();
        write_config(tmp.path(), "[paths\n");
        let err = run(&api(), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::Toml { path, .. } if path.ends_with(".ductus/config.toml")),
            "{err:?}"
        );
        assert_eq!(config(tmp.path()), "[paths\n");
    }

    #[test]
    fn an_entry_the_reader_refuses_is_refused() {
        let tmp = tempdir().unwrap();
        let before = "[services.web]\nrepo = \"https://h/o/web\"\n";
        write_config(tmp.path(), before);
        let err = run(&api(), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::InvalidServiceEntry { alias, .. } if alias == "web"),
            "{err:?}"
        );
        assert_eq!(config(tmp.path()), before);
    }

    /// An inline-table `services` cannot be extended by a `[services.<alias>]`
    /// header, so the appended text would not parse: refused, nothing written.
    #[test]
    fn an_inline_table_services_is_refused() {
        let tmp = tempdir().unwrap();
        let before = "services = { web = { repo = \"https://h/o/web\", path = \"../web\" } }\n";
        write_config(tmp.path(), before);
        let err = run(&api(), tmp.path()).unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::ServicesNotExtensible { alias, .. } if alias == "api"),
            "{err:?}"
        );
        assert_eq!(config(tmp.path()), before);
    }

    /// Dotted keys define `services` in a form a header can still extend, so
    /// the table is written and both entries read back.
    #[test]
    fn dotted_key_services_are_extended() {
        let tmp = tempdir().unwrap();
        write_config(
            tmp.path(),
            "services.web.repo = \"https://h/o/web\"\nservices.web.path = \"../web\"\n",
        );
        run(&api(), tmp.path()).unwrap();
        let read = Services::from_toml_str(&config(tmp.path())).unwrap();
        assert_eq!(read.0.keys().collect::<Vec<_>>(), ["api", "web"]);
    }

    #[test]
    fn a_duplicate_repo_and_a_missing_checkout_warn_without_refusing() {
        let tmp = tempdir().unwrap();
        write_config(
            tmp.path(),
            "[services.old]\nrepo = \"https://github.com/acme/api.git\"\npath = \"../old\"\n",
        );
        let result = run(&api(), tmp.path()).unwrap();
        assert_eq!(result.warnings.len(), 2, "{:?}", result.warnings);
        assert!(
            result.warnings[0].contains("[services.old]"),
            "{:?}",
            result.warnings
        );
        assert!(
            result.warnings[1].contains("no directory"),
            "{:?}",
            result.warnings
        );
        assert!(config(tmp.path()).contains("[services.api]"));
    }

    #[test]
    fn an_existing_checkout_and_a_new_repo_warn_nothing() {
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("checkouts/api")).unwrap();
        let result = run(
            &args("api", "https://github.com/acme/api", "checkouts/api", None),
            tmp.path(),
        )
        .unwrap();
        assert!(result.warnings.is_empty(), "{:?}", result.warnings);
    }

    /// A change another writer lands between the read and the write is kept:
    /// the write is refused, never made over it (`BE-TXN-002`).
    #[test]
    fn a_config_changed_after_the_read_is_refused_and_kept() {
        let tmp = tempdir().unwrap();
        write_config(tmp.path(), "[paths]\nspecs-root = \"specs\"\n");
        let file = tmp.path().join(".ductus/config.toml");
        let err = guarded::between_read_and_write(guarded::theirs_lands(&file), || {
            run(&api(), tmp.path())
        })
        .unwrap_err();
        assert!(
            matches!(&err, PrimitiveError::ChangedSinceRead { path } if path == Path::new(".ductus/config.toml")),
            "{err:?}"
        );
        assert_eq!(
            config(tmp.path()),
            "[paths]\nspecs-root = \"specs\"\n\nTheirs.\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_config_that_is_a_link_leaving_the_project_is_refused() {
        let outside = tempdir().unwrap();
        fs::write(outside.path().join("config.toml"), "").unwrap();
        let tmp = tempdir().unwrap();
        fs::create_dir_all(tmp.path().join(".ductus")).unwrap();
        std::os::unix::fs::symlink(
            outside.path().join("config.toml"),
            tmp.path().join(".ductus/config.toml"),
        )
        .unwrap();
        let err = run(&api(), tmp.path()).unwrap_err();
        assert!(matches!(err, PrimitiveError::InvalidPath { .. }), "{err:?}");
        assert_eq!(
            fs::read_to_string(outside.path().join("config.toml")).unwrap(),
            ""
        );
    }
}
