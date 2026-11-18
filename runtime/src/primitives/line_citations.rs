//! What a line-number citation is, and how one is stripped (specs 066 and 067), for the
//! rule 050's AC24 states.
//!
//! One definition for the readers that must agree on it: `mechanical_sweep` and
//! `classify-spec-edit`, which count a removed citation as a mechanical edit (066 AC6),
//! and `check-artifacts`' `line-number-citations` family, which reports one (067 AC5). A
//! copy per reader would let the check flag a citation whose removal the rule calls a
//! real change.
//!
//! A citation is a suffix after a path token:
//!
//! - `:N`, `:N-M` (a hyphen or an en dash) or `:N:C`, after a token that follows neither
//!   `:` nor `@`, so a URL's host (`https://example.com:8080`) and a `user@host:22` never
//!   match;
//! - `#LN` or `#LN-LM`, a link's line anchor.
//!
//! [`find_set_apart`] reads a third form, which only the check reports: an inline code
//! span holding nothing but such a suffix, a line number set apart from its file.
//!
//! The token must end in an extension, one from [`EXTENSIONS`] when it holds no `/`, so a
//! host and port such as `db.internal:5432` is not a citation (owner decision,
//! 2026-11-17). A host whose last label is also a listed extension (`docs.rs:443`) still
//! reads as one: no list tells a crate-docs host from a Rust file.

use std::borrow::Cow;
use std::ops::Range;

/// The extensions a token with no `/` may end in and still be read as a file path.
///
/// Closed, so a host's last label is not taken for one, and compared without regard to
/// case. A path holding a `/` needs none of these: any extension serves.
pub(crate) const EXTENSIONS: &[&str] = &[
    "adoc", "bash", "bat", "c", "cc", "cfg", "cjs", "clj", "cljs", "cmake", "conf", "cpp", "cs",
    "css", "csv", "cts", "cxx", "dart", "el", "erl", "ex", "exs", "fish", "fs", "go", "gradle",
    "graphql", "groovy", "h", "hcl", "hh", "hpp", "hrl", "hs", "htm", "html", "ini", "ipynb",
    "java", "jl", "js", "json", "jsonc", "jsonl", "jsx", "kt", "kts", "less", "lock", "lua", "m",
    "markdown", "md", "mdx", "mjs", "mk", "ml", "mli", "mm", "mts", "nim", "nix", "php", "pl",
    "pm", "proto", "ps1", "py", "pyi", "r", "rb", "rs", "rst", "sass", "scala", "scss", "sh",
    "sol", "sql", "svelte", "swift", "tex", "tf", "toml", "ts", "tsv", "tsx", "txt", "vb", "vue",
    "xml", "yaml", "yml", "zig", "zsh",
];

/// One citation in a line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Citation {
    /// Byte offset of the path token's first character.
    pub(crate) path_start: usize,
    /// Byte range of the line-number suffix, the `:N…` or `#L…` that makes the token a
    /// citation and that [`strip`] removes.
    pub(crate) suffix: Range<usize>,
}

/// Every citation in `line`, in order.
pub(crate) fn find(line: &str) -> Vec<Citation> {
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if !is_path_byte(bytes[at]) {
            // Every path byte is ASCII, so a byte that is not one, a UTF-8
            // continuation byte included, can be stepped over alone.
            at += 1;
            continue;
        }
        let start = at;
        while at < bytes.len() && is_path_byte(bytes[at]) {
            at += 1;
        }
        let token = &line[start..at];
        if !is_path(token) {
            continue;
        }
        let after_colon_or_at = start > 0 && matches!(bytes[start - 1], b':' | b'@');
        let end = match bytes.get(at) {
            Some(b':') if !after_colon_or_at => colon_suffix_end(line, at),
            Some(b'#') => anchor_suffix_end(line, at),
            _ => None,
        };
        if let Some(end) = end {
            found.push(Citation {
                path_start: start,
                suffix: at..end,
            });
            at = end;
        }
    }
    found
}

/// Each inline code span in `line` whose whole content is a line-number suffix set apart
/// from its file, as in `` `a.rs:12`, `:388` ``, by the range of the span's content. The
/// path-token reading of [`find`] cannot see one, since no path precedes it (owner
/// decision, 2026-11-17). [`strip`] leaves these alone: removing one deletes a span, which
/// is no suffix strip.
pub(crate) fn find_set_apart(line: &str) -> Vec<Range<usize>> {
    let mut found = Vec::new();
    let mut rest = 0;
    while let Some(open) = line[rest..].find('`').map(|offset| rest + offset) {
        let Some(close) = line[open + 1..].find('`').map(|offset| open + 1 + offset) else {
            break;
        };
        let content = &line[open + 1..close];
        let whole = match content.as_bytes().first() {
            Some(b':') => colon_suffix_end(content, 0),
            Some(b'#') => anchor_suffix_end(content, 0),
            _ => None,
        };
        if whole == Some(content.len()) {
            found.push(open + 1..close);
        }
        rest = close + 1;
    }
    found
}

/// `text` with every citation's suffix removed, each path left in place. Borrowed when it
/// holds none.
pub(crate) fn strip(text: &str) -> Cow<'_, str> {
    let mut stripped: Option<String> = None;
    for (index, line) in text.split_inclusive('\n').enumerate() {
        let citations = find(line);
        if citations.is_empty() {
            if let Some(out) = stripped.as_mut() {
                out.push_str(line);
            }
            continue;
        }
        let out = stripped
            .get_or_insert_with(|| text.split_inclusive('\n').take(index).collect::<String>());
        let mut kept = 0;
        for citation in citations {
            out.push_str(&line[kept..citation.suffix.start]);
            kept = citation.suffix.end;
        }
        out.push_str(&line[kept..]);
    }
    stripped.map_or(Cow::Borrowed(text), Cow::Owned)
}

/// A byte a path token is made of: `[A-Za-z0-9_./-]`.
fn is_path_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'/' | b'-')
}

/// Whether `token` names a file: it ends in `.ext`, the extension a letter then letters
/// and digits, and it holds a `/` or the extension is one of [`EXTENSIONS`].
fn is_path(token: &str) -> bool {
    let Some((stem, extension)) = token.rsplit_once('.') else {
        return false;
    };
    let mut chars = extension.chars();
    let shaped = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric());
    shaped
        && !stem.is_empty()
        && (token.contains('/') || EXTENSIONS.iter().any(|e| e.eq_ignore_ascii_case(extension)))
}

/// The end of a `:N`, `:N-M` or `:N:C` suffix starting at `colon`.
fn colon_suffix_end(line: &str, colon: usize) -> Option<usize> {
    let mut end = digits_end(line, colon + 1)?;
    if let Some(dash) = line[end..]
        .chars()
        .next()
        .filter(|c| matches!(c, '-' | '–'))
        && let Some(range_end) = digits_end(line, end + dash.len_utf8())
    {
        end = range_end;
    }
    if line[end..].starts_with(':')
        && let Some(column_end) = digits_end(line, end + 1)
    {
        end = column_end;
    }
    ends_cleanly(line, end).then_some(end)
}

/// The end of a `#LN` or `#LN-LM` anchor starting at `hash`.
fn anchor_suffix_end(line: &str, hash: usize) -> Option<usize> {
    if !line[hash..].starts_with("#L") {
        return None;
    }
    let mut end = digits_end(line, hash + 2)?;
    if line[end..].starts_with('-') {
        let from = end + 1 + usize::from(line[end + 1..].starts_with('L'));
        if let Some(range_end) = digits_end(line, from) {
            end = range_end;
        }
    }
    ends_cleanly(line, end).then_some(end)
}

/// The end of the run of ASCII digits starting at `from`, when there is at least one.
fn digits_end(line: &str, from: usize) -> Option<usize> {
    let run = line
        .get(from..)?
        .bytes()
        .take_while(u8::is_ascii_digit)
        .count();
    (run > 0).then_some(from + run)
}

/// Whether a suffix ending at `end` is not the start of a longer word or path, so
/// `a.rs:12abc` and `host.rs:80/path` are not citations.
fn ends_cleanly(line: &str, end: usize) -> bool {
    !line[end..]
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '/'))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// The suffixes `find` reports in `line`, as text.
    fn suffixes(line: &str) -> Vec<&str> {
        find(line)
            .into_iter()
            .map(|citation| &line[citation.suffix])
            .collect()
    }

    #[test]
    fn each_citation_form_is_found() {
        for (line, suffix) in [
            ("as `parity.rs:460` shows", ":460"),
            ("in src/x.ts:3-9.", ":3-9"),
            ("at a.rs:12:5 here", ":12:5"),
            ("x.rs:1–4 then", ":1–4"),
            ("[it](foo.rs#L12-L20)", "#L12-L20"),
            ("[it](foo.rs#L12)", "#L12"),
            ("[it](https://github.com/o/r/blob/main/a.rs#L7)", "#L7"),
            ("`Cargo.toml:5`", ":5"),
            ("specs/012/plan.md:44,", ":44"),
            ("a.RS:3", ":3"),
        ] {
            assert_eq!(suffixes(line), vec![suffix], "{line}");
        }
    }

    #[test]
    fn hosts_ports_and_other_colons_are_not_citations() {
        for line in [
            "connect to db.internal:5432",
            "https://example.com:8080/x",
            "http://db.internal:5432",
            "ssh user@host.example:22",
            "localhost:3000",
            "v0.59.11 and 12.30:45",
            "a.rs:12abc",
            "host.rs:80/path",
            "src/a.rs alone",
            "§spec-lifecycle at 10:30",
            "a.rs#Lx and a.rs#12",
            ".rs:3",
        ] {
            assert!(find(line).is_empty(), "{line}: {:?}", find(line));
        }
    }

    #[test]
    fn a_line_may_hold_several() {
        assert_eq!(
            suffixes("a.rs:1, b.md:2 and c.py#L3"),
            vec![":1", ":2", "#L3"]
        );
    }

    #[test]
    fn stripping_leaves_each_path() {
        assert_eq!(
            strip("see `a.rs:12` and [b](b.md#L3).\nno citation\nx.rs:1–4\n"),
            "see `a.rs` and [b](b.md).\nno citation\nx.rs\n"
        );
    }

    #[test]
    fn text_without_a_citation_is_borrowed() {
        assert!(matches!(strip("db.internal:5432\n"), Cow::Borrowed(_)));
    }

    /// The set-apart spans `find_set_apart` reports in `line`, as text.
    fn set_apart(line: &str) -> Vec<&str> {
        find_set_apart(line)
            .into_iter()
            .map(|span| &line[span])
            .collect()
    }

    #[test]
    fn a_line_number_set_apart_from_its_file_is_found() {
        assert_eq!(
            set_apart("`a.rs:12`, `:388`, `:12-30`, `:1–4`, `:7:2` and `#L3`."),
            vec![":388", ":12-30", ":1–4", ":7:2", "#L3"]
        );
        for line in [
            "`:x`",
            "`a:12`",
            "`:8080x`",
            "`::`",
            "an unclosed `:12",
            "`/ductus:review`",
            "no span :12",
        ] {
            assert!(find_set_apart(line).is_empty(), "{line}");
        }
    }

    #[test]
    fn stripping_leaves_a_set_apart_span() {
        assert_eq!(strip("`a.rs:1`, `:2`\n"), "`a.rs`, `:2`\n");
    }

    #[test]
    fn multibyte_text_around_a_citation_is_kept() {
        assert_eq!(strip("— a.rs:3 — é\n"), "— a.rs — é\n");
    }
}
