//! The keys `ductus` recognizes in each section of the project config file it
//! owns, and the report of every other key found there (spec 019's
//! `an-unknown-config-key-is-named`, 022's `config-readers-name-unknown-keys`,
//! `CFG-ENV-012`).
//!
//! Each reader deserializes only the section it needs, and serde drops every
//! key the reader's struct does not name, so a mistyped key silently takes its
//! default. This module is the one declaration of what each `ductus`-owned
//! section may hold. Readers report a key outside it rather than failing,
//! because teammates can run different `ductus` versions against one committed
//! file, and a key a newer version added is not an error for an older one.
//!
//! The check reads key names only. A value of the wrong type is its own
//! reader's to report, so it never stops another section's report, and a
//! document that does not parse reports nothing here. Whether a reader
//! reports the parse error is its own contract: most halt or warn, while
//! `derive-references` and `check-orphaned-references` read an unparseable
//! file as an empty registry and an absent watermark. Sections `ductus` does
//! not own are never examined, and neither is the session file, whose top
//! level is the exec walker's open seed.

use std::collections::BTreeSet;
use std::fmt::{self, Write as _};
use std::io::Write as _;
use std::path::Path;
use std::sync::{Mutex, PoisonError};

use crate::primitives::is_line_hazard;

/// A `ductus`-owned section of the config file and what it may hold.
struct Section {
    /// The top-level table name.
    name: &'static str,
    /// The section's plain keys — or, for an [`Self::entries`] section, the
    /// fields of each named entry.
    keys: &'static [&'static str],
    /// The section is a table of named entries (`[constitutions.<alias>]`)
    /// whose names are the project's own; only their fields are checked.
    entries: bool,
    /// Keys holding an array of tables, each with the fields its entries may
    /// carry.
    tables: &'static [(&'static str, &'static [&'static str])],
}

impl Section {
    const fn keys(name: &'static str, keys: &'static [&'static str]) -> Self {
        Self {
            name,
            keys,
            entries: false,
            tables: &[],
        }
    }

    const fn entries(name: &'static str, fields: &'static [&'static str]) -> Self {
        Self {
            name,
            keys: fields,
            entries: true,
            tables: &[],
        }
    }
}

/// Every `ductus`-owned section, with the reader of each key. The installer's
/// §Project Configuration (`framework/bootstrap/ductus.md`) documents the same
/// set, and `the_installer_documents_the_registry` holds the two together.
const SECTIONS: &[Section] = &[
    // `project`: `Host::load` and `/ductus`. `cli-config-dir`: `Host::load`'s
    // fallback for a project predating its move to the session file.
    Section::keys("host", &["project", "cli-config-dir"]),
    // `/ductus` (spec 061).
    Section::keys("source", &["ref"]),
    // `/ductus` (§Collect Project Inputs).
    Section::keys("project", &["name", "description", "languages"]),
    // `discover-rule-files` and `/ductus` (spec 033).
    Section::keys("rules", &["surfaces"]),
    // `Paths::load` and `/ductus` (spec 040).
    Section::keys("paths", &["specs-root"]),
    // `/ductus`.
    Section::keys("pinned", &["files"]),
    // `/ductus` and `check-orphaned-references`.
    Section::keys("migrations", &["last_applied"]),
    // `check-artifact-size` (spec 063).
    Section::keys("artifacts", &["read-size-bytes"]),
    // `tech-stack-verified`: `/{project}:review` by hand (spec 020).
    // `disabled-rule-files`: `discover-rule-files` and `dashboard` (spec 025).
    Section {
        name: "review",
        keys: &["tech-stack-verified"],
        entries: false,
        tables: &[("disabled-rule-files", &["file", "reason"])],
    },
    // `/ductus` (spec 048).
    Section::keys("runtime", &["path"]),
    // `resolve-constitutions` (spec 055).
    Section::entries("constitutions", &["repo", "path", "description"]),
    // `resolve-references` and `derive-references` (spec 030).
    Section::entries("services", &["repo", "path", "description"]),
    // Retired by spec 043, whose `workflows-sunset` migration removes it, so
    // anything still in it is reported.
    Section::keys("workflows", &[]),
];

/// A key in a `ductus`-owned section that no reader of the section reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownKey {
    /// Where the key sits, as TOML names it: `[rules]`, `[constitutions.acme]`,
    /// or `[[review.disabled-rule-files]] entry [0]`.
    pub location: String,
    /// The key as TOML writes it (see [`key_text`]).
    pub key: String,
}

impl fmt::Display for UnknownKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.location, self.key)
    }
}

impl UnknownKey {
    /// The one-line report naming the key, its section and `file`.
    #[must_use]
    pub fn notice(&self, file: &str) -> String {
        format!("unknown config key: {self} ({file}) — this version of ductus does not read it")
    }
}

/// The unknown keys in every `ductus`-owned section of `config`, in registry
/// order and then key order.
#[must_use]
pub fn unknown_keys(config: &toml::Table) -> Vec<UnknownKey> {
    let mut found = Vec::new();
    for section in SECTIONS {
        report_section(config, section, &mut found);
    }
    found
}

/// The unknown keys in the named `sections` of `config` — the sections one
/// reader reads.
#[must_use]
pub fn unknown_keys_in(config: &toml::Table, sections: &[&str]) -> Vec<UnknownKey> {
    let mut found = Vec::new();
    for section in SECTIONS.iter().filter(|s| sections.contains(&s.name)) {
        report_section(config, section, &mut found);
    }
    found
}

/// The unknown keys in the named `sections` of the config document
/// `content`; none when it does not parse.
#[must_use]
pub fn unknown_keys_in_text(content: &str, sections: &[&str]) -> Vec<UnknownKey> {
    toml::from_str::<toml::Table>(content)
        .map(|config| unknown_keys_in(&config, sections))
        .unwrap_or_default()
}

/// Write each unknown key in `sections` of the config document `content`,
/// read from `path`, to stderr — the channel for a reader whose result has no
/// notices. Each line is written once per process, however often the reader
/// runs. A document that does not parse writes nothing.
///
/// The record of what was written is process-wide because the readers that
/// call this share no context object to carry it. It is a `Mutex`, held only
/// for the check-and-insert, which `BTreeSet::insert` makes one operation;
/// the document is parsed before the lock and the lines written after it.
/// A write that fails — stderr closed, or a pipe whose reader has gone — is
/// dropped rather than panicking as `eprintln!` would, because a warning
/// must never fail the run it warns about.
pub fn warn_unknown_keys(path: &Path, content: &str, sections: &[&str]) {
    static WARNED: Mutex<BTreeSet<String>> = Mutex::new(BTreeSet::new());
    let fresh = first_seen(
        warning_lines(path, content, sections),
        &mut WARNED.lock().unwrap_or_else(PoisonError::into_inner),
    );
    let mut stderr = std::io::stderr().lock();
    for line in fresh {
        let _ = writeln!(stderr, "ductus: {line}");
    }
}

/// The lines [`warn_unknown_keys`] writes for `content`, read from `path`,
/// before the once-per-process filter.
fn warning_lines(path: &Path, content: &str, sections: &[&str]) -> Vec<String> {
    let file = path_text(path);
    unknown_keys_in_text(content, sections)
        .iter()
        .map(|key| key.notice(&file))
        .collect()
}

/// The `lines` `warned` does not already hold, each recorded there as it is
/// returned.
fn first_seen(lines: Vec<String>, warned: &mut BTreeSet<String>) -> Vec<String> {
    lines
        .into_iter()
        .filter(|line| warned.insert(line.clone()))
        .collect()
}

/// Append `section`'s unknown keys in `config` to `found`. A section that is
/// absent, or is not a table, holds nothing to check.
fn report_section(config: &toml::Table, section: &Section, found: &mut Vec<UnknownKey>) {
    let Some(toml::Value::Table(body)) = config.get(section.name) else {
        return;
    };
    if section.entries {
        for (name, entry) in body {
            if let toml::Value::Table(fields) = entry {
                let location = format!("[{}.{}]", section.name, key_text(name));
                report_fields(fields, section.keys, &location, found);
            }
        }
        return;
    }
    let location = format!("[{}]", section.name);
    for (key, value) in body {
        if let Some((_, fields)) = section.tables.iter().find(|(name, _)| name == key) {
            let toml::Value::Array(items) = value else {
                continue;
            };
            for (index, item) in items.iter().enumerate() {
                if let toml::Value::Table(entry) = item {
                    let location = format!("[[{}.{key}]] entry [{index}]", section.name);
                    report_fields(entry, fields, &location, found);
                }
            }
        } else if !section.keys.contains(&key.as_str()) {
            found.push(UnknownKey {
                location: location.clone(),
                key: key_text(key),
            });
        }
    }
}

/// Append each key of `table` outside `known` to `found`, at `location`.
fn report_fields(table: &toml::Table, known: &[&str], location: &str, found: &mut Vec<UnknownKey>) {
    for key in table.keys() {
        if !known.contains(&key.as_str()) {
            found.push(UnknownKey {
                location: location.to_owned(),
                key: key_text(key),
            });
        }
    }
}

/// `key` as TOML writes it: bare when every character may appear in a bare
/// key, otherwise a basic string with `"`, `\`, every line hazard, a tab, and
/// every bidirectional formatting control escaped. The operator sees exactly
/// which key it is: a key holding a newline or a terminal escape cannot forge
/// a second line or restyle the terminal (`BE-INPUT-011`), one holding a
/// right-to-left override cannot reorder the rest of the line as displayed,
/// and a tab is not shown as the space it resembles. Escaping rather than
/// flattening keeps two keys that differ only in such a character
/// distinguishable.
fn key_text(key: &str) -> String {
    let bare = !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
    if bare {
        return key.to_owned();
    }
    let mut text = String::from('"');
    for c in key.chars() {
        match c {
            '"' => text.push_str("\\\""),
            '\\' => text.push_str("\\\\"),
            c if is_display_hazard(c) => push_escaped(&mut text, c),
            c => text.push(c),
        }
    }
    text.push('"');
    text
}

/// `path` as the stderr report names it, with every character
/// [`key_text`] escapes for display escaped the same way. The path can come
/// from committed config — a registered service checkout's `path` — so a
/// newline or a terminal escape in it must not forge a line (`BE-INPUT-011`).
/// A backslash is left alone: on Windows it is the path separator.
fn path_text(path: &Path) -> String {
    let mut text = String::new();
    for c in path.display().to_string().chars() {
        if is_display_hazard(c) {
            push_escaped(&mut text, c);
        } else {
            text.push(c);
        }
    }
    text
}

/// A character shown escaped rather than raw: a line hazard, a tab, or a
/// bidirectional formatting control.
fn is_display_hazard(c: char) -> bool {
    c == '\t' || is_line_hazard(c) || is_bidi_control(c)
}

/// Append `c` as a TOML `\uXXXX` escape; every display hazard is in the
/// Basic Multilingual Plane.
fn push_escaped(text: &mut String, c: char) {
    let _ = write!(text, "\\u{:04X}", u32::from(c));
}

/// The Unicode bidirectional formatting controls (UAX #9): the Arabic letter
/// mark, the left-to-right and right-to-left marks, the embeddings and
/// overrides, and the isolates.
fn is_bidi_control(c: char) -> bool {
    matches!(c, '\u{061C}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
}

/// Whether the registry recognizes `key` at `at`: `[]` asks about a section
/// name, `[section]` a key of a plain section, `[section, entry]` a field of
/// a named entry, or of the entries of the array of tables `entry` names.
#[cfg(test)]
pub(crate) fn recognizes(at: &[&str], key: &str) -> bool {
    let section = |name: &str| SECTIONS.iter().find(|s| s.name == name);
    match at {
        [] => section(key).is_some(),
        [name] => section(name).is_some_and(|s| {
            !s.entries && (s.keys.contains(&key) || s.tables.iter().any(|(t, _)| *t == key))
        }),
        [name, entry] => section(name).is_some_and(|s| {
            if s.entries {
                s.keys.contains(&key)
            } else {
                s.tables
                    .iter()
                    .any(|(t, fields)| t == entry && fields.contains(&key))
            }
        }),
        _ => false,
    }
}

/// The keys of `fields` the registry does not recognize at `at` — empty when
/// a reader's every key is declared here.
#[cfg(test)]
pub(crate) fn unrecognized(at: &[&str], fields: &[&str]) -> Vec<String> {
    fields
        .iter()
        .filter(|key| !recognizes(at, key))
        .map(|key| (*key).to_owned())
        .collect()
}

/// The field names a `Deserialize` struct reads, as serde names them after
/// every `rename`. Lets each reader's test compare the keys it actually reads
/// with the registry, so a field added to a reader without a registry entry
/// fails a test rather than being reported to every operator as unknown.
#[cfg(test)]
pub(crate) fn fields_of<T: serde::de::DeserializeOwned>() -> &'static [&'static str] {
    use serde::de::Visitor;

    #[derive(Debug)]
    struct Stop;
    impl fmt::Display for Stop {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("probe")
        }
    }
    impl std::error::Error for Stop {}
    impl serde::de::Error for Stop {
        fn custom<M: fmt::Display>(_: M) -> Self {
            Self
        }
    }

    /// A deserializer that records the field list a struct asks for, then
    /// stops.
    struct Probe<'a>(&'a mut &'static [&'static str]);
    impl<'de> serde::Deserializer<'de> for Probe<'_> {
        type Error = Stop;
        fn deserialize_any<V: Visitor<'de>>(self, _: V) -> Result<V::Value, Stop> {
            Err(Stop)
        }
        fn deserialize_struct<V: Visitor<'de>>(
            self,
            _: &'static str,
            fields: &'static [&'static str],
            _: V,
        ) -> Result<V::Value, Stop> {
            *self.0 = fields;
            Err(Stop)
        }
        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
            bytes byte_buf option unit unit_struct newtype_struct seq tuple
            tuple_struct map enum identifier ignored_any
        }
    }

    let mut fields: &'static [&'static str] = &[];
    let _ = T::deserialize(Probe(&mut fields));
    fields
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn table(text: &str) -> toml::Table {
        toml::from_str(text).unwrap()
    }

    fn shown(keys: &[UnknownKey]) -> Vec<String> {
        keys.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn a_mistyped_key_in_an_owned_section_is_named() {
        let config = table("[rules]\nsurface = [\"backend\"]\n[paths]\nspecs-root = \"specs\"\n");
        assert_eq!(shown(&unknown_keys(&config)), ["[rules] surface"]);
    }

    #[test]
    fn every_recognized_key_reports_nothing() {
        let config = table(
            "[host]\nproject = \"p\"\ncli-config-dir = \".claude\"\n\
             [source]\nref = \"main\"\n\
             [project]\nname = \"p\"\ndescription = \"d\"\nlanguages = [\"Rust\"]\n\
             [rules]\nsurfaces = []\n[paths]\nspecs-root = \"specs\"\n\
             [pinned]\nfiles = []\n[migrations]\nlast_applied = \"x\"\n\
             [artifacts]\nread-size-bytes = 1\n\
             [review]\ntech-stack-verified = true\n\
             [[review.disabled-rule-files]]\nfile = \"a.md\"\nreason = \"r\"\n\
             [runtime]\npath = \"bin\"\n\
             [constitutions.acme]\nrepo = \"https://x/y\"\npath = \"../g\"\ndescription = \"d\"\n\
             [services.api]\nrepo = \"https://x/z\"\npath = \"../api\"\n",
        );
        assert_eq!(unknown_keys(&config), []);
    }

    #[test]
    fn a_section_ductus_does_not_own_is_not_examined() {
        let config = table("[tool.custom]\nanything = 1\n[team]\nowner = \"x\"\n");
        assert_eq!(unknown_keys(&config), []);
    }

    #[test]
    fn a_field_on_a_named_entry_is_named_with_its_entry() {
        let config =
            table("[constitutions.acme]\nrepo = \"https://x/y\"\npath = \"../g\"\nurl = \"u\"\n");
        assert_eq!(shown(&unknown_keys(&config)), ["[constitutions.acme] url"]);
    }

    #[test]
    fn an_adopter_field_on_an_array_entry_is_named_with_its_index() {
        let config = table(
            "[[review.disabled-rule-files]]\nfile = \"a.md\"\nreason = \"r\"\n\
             [[review.disabled-rule-files]]\nfile = \"b.md\"\nreason = \"r\"\napproved-by = \"ops\"\n",
        );
        assert_eq!(
            shown(&unknown_keys(&config)),
            ["[[review.disabled-rule-files]] entry [1] approved-by"]
        );
    }

    #[test]
    fn a_retired_section_reports_every_key() {
        let config = table("[workflows]\ndeclined_categories = [\"Linting\"]\n");
        assert_eq!(
            shown(&unknown_keys(&config)),
            ["[workflows] declined_categories"]
        );
    }

    #[test]
    fn a_value_of_the_wrong_type_is_left_to_its_reader() {
        // A non-table section, a non-table entry, and a non-array table key
        // are each their reader's to report; the other sections still are.
        let config = table(
            "artifacts = 5\n[constitutions]\nacme = \"x\"\n\
             [review]\ndisabled-rule-files = \"x\"\n[rules]\nsurface = 1\n",
        );
        assert_eq!(shown(&unknown_keys(&config)), ["[rules] surface"]);
    }

    #[test]
    fn only_the_named_sections_are_examined() {
        let config = table("[rules]\nsurface = 1\n[paths]\nroot = \"x\"\n");
        assert_eq!(
            shown(&unknown_keys_in(&config, &["paths"])),
            ["[paths] root"]
        );
    }

    #[test]
    fn a_key_that_needs_quoting_is_shown_quoted_and_escaped() {
        let config = table("[rules]\n\"a b\" = 1\n\"x\\ny\\u001b[31m\" = 2\n\"q\\\"\\\\\" = 3\n");
        let names: Vec<String> = unknown_keys(&config).into_iter().map(|k| k.key).collect();
        assert!(names.contains(&"\"a b\"".to_string()), "{names:?}");
        assert!(
            names.contains(&"\"x\\u000Ay\\u001B[31m\"".to_string()),
            "{names:?}"
        );
        assert!(names.contains(&"\"q\\\"\\\\\"".to_string()), "{names:?}");
        for name in &names {
            assert!(!name.chars().any(is_line_hazard), "{name:?}");
        }
        // A tab and a right-to-left override are escaped too, so neither
        // passes for a space nor reorders the line it is shown in.
        let config = table("[rules]\n\"a\\tb\" = 1\n\"r\\u202Eevil\" = 2\n");
        let names: Vec<String> = unknown_keys(&config).into_iter().map(|k| k.key).collect();
        assert_eq!(names, ["\"a\\u0009b\"", "\"r\\u202Eevil\""]);
    }

    #[test]
    fn each_warning_is_written_once_however_often_its_section_is_read() {
        let mut warned = BTreeSet::new();
        let line = |key: &str| format!("unknown config key: [paths] {key} (f)");
        assert_eq!(first_seen(vec![line("root")], &mut warned), [line("root")]);
        assert_eq!(
            first_seen(vec![line("root")], &mut warned),
            [] as [String; 0]
        );
        // A different key warns on its own first appearance, beside a repeat.
        assert_eq!(
            first_seen(vec![line("root"), line("base")], &mut warned),
            [line("base")]
        );
    }

    #[test]
    fn a_path_from_committed_config_is_shown_with_its_hazards_escaped() {
        // A service checkout's path comes from `[services] path`, so it can
        // carry what a key can; a backslash stays, as a Windows separator.
        let path = Path::new("../svc\nfake\u{1b}[31m\u{202E}\\x/.ductus/config.toml");
        let shown = path_text(path);
        assert_eq!(
            shown,
            "../svc\\u000Afake\\u001B[31m\\u202E\\x/.ductus/config.toml"
        );
        assert!(!shown.chars().any(is_display_hazard), "{shown:?}");
        // The warning names the file by that escaped form.
        assert_eq!(
            warning_lines(Path::new("svc\n/c.toml"), "[paths]\nroot = 1\n", &["paths"]),
            [
                "unknown config key: [paths] root (svc\\u000A/c.toml) — this version of ductus does not read it"
            ]
        );
    }

    #[test]
    fn the_notice_names_the_key_the_section_and_the_file() {
        let key = UnknownKey {
            location: "[rules]".into(),
            key: "surface".into(),
        };
        assert_eq!(
            key.notice(".ductus/config.toml"),
            "unknown config key: [rules] surface (.ductus/config.toml) — this version of ductus does not read it"
        );
    }

    #[test]
    fn the_registry_answers_for_sections_keys_entries_and_tables() {
        assert!(recognizes(&[], "review"));
        assert!(!recognizes(&[], "tool"));
        assert!(recognizes(&["review"], "disabled-rule-files"));
        assert!(!recognizes(&["review"], "file"));
        assert!(recognizes(&["review", "disabled-rule-files"], "reason"));
        assert!(!recognizes(
            &["review", "disabled-rule-files"],
            "approved-by"
        ));
        assert!(recognizes(&["services", "api"], "repo"));
        assert!(!recognizes(&["services"], "repo"));
    }

    #[test]
    fn the_probe_reads_a_struct_s_field_names_after_renames() {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "kebab-case")]
        #[allow(dead_code)]
        struct Shape {
            plain: u8,
            two_words: u8,
            #[serde(rename = "renamed")]
            other: u8,
        }
        assert_eq!(fields_of::<Shape>(), ["plain", "two-words", "renamed"]);
    }

    /// The installer's §Project Configuration documents every section and
    /// key the registry recognizes, retired sections and `[host]`'s legacy
    /// `cli-config-dir` aside, and nothing it does not — so a host reading
    /// the file by hand on the markdown-only path reports what the runtime
    /// reports.
    #[test]
    fn the_installer_documents_the_registry() {
        let installer = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/bootstrap/ductus.md"),
        )
        .unwrap();
        let section = installer
            .split("\n## Project Configuration\n")
            .nth(1)
            .and_then(|rest| rest.split("```toml\n").nth(1))
            .and_then(|block| block.split("\n```").next())
            .expect("§Project Configuration holds a toml block");
        // Uncomment the schema-reference lines: a commented table header or
        // `key = value` line becomes live; prose comments stay comments.
        let live: String = section
            .lines()
            .map(|line| {
                let rest = line.strip_prefix("# ").unwrap_or(line);
                let is_schema = (rest.starts_with('[') && rest.ends_with(']'))
                    || rest
                        .split_once(" = ")
                        .is_some_and(|(k, _)| !k.is_empty() && !k.contains(' '));
                if line.starts_with("# ") && is_schema {
                    rest
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let config: toml::Table = toml::from_str(&live).unwrap();
        assert_eq!(shown(&unknown_keys(&config)), [] as [String; 0]);
        // Every section documented is one the registry owns, so none of
        // them is skipped as unowned.
        for name in config.keys() {
            assert!(
                recognizes(&[], name),
                "[{name}] is documented but not owned"
            );
        }
        for section in SECTIONS.iter().filter(|s| !s.keys.is_empty()) {
            let Some(toml::Value::Table(body)) = config.get(section.name) else {
                panic!("§Project Configuration omits [{}]", section.name);
            };
            let documented: BTreeSet<&str> = if section.entries {
                body.values()
                    .filter_map(toml::Value::as_table)
                    .flat_map(|entry| entry.keys().map(String::as_str))
                    .collect()
            } else {
                body.keys().map(String::as_str).collect()
            };
            for key in section
                .keys
                .iter()
                .chain(section.tables.iter().map(|(t, _)| t))
            {
                let legacy = section.name == "host" && *key == "cli-config-dir";
                assert!(
                    legacy || documented.contains(key),
                    "§Project Configuration omits [{}] {key}",
                    section.name
                );
            }
            for (table, fields) in section.tables {
                let Some(toml::Value::Array(entries)) = body.get(*table) else {
                    continue;
                };
                let documented: BTreeSet<&str> = entries
                    .iter()
                    .filter_map(toml::Value::as_table)
                    .flat_map(|entry| entry.keys().map(String::as_str))
                    .collect();
                for field in *fields {
                    assert!(
                        documented.contains(field),
                        "§Project Configuration omits [[{}.{table}]] {field}",
                        section.name
                    );
                }
            }
        }
    }
}
