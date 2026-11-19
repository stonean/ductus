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
    code_spans(line)
        .into_iter()
        .filter(|span| {
            let content = &line[span.clone()];
            let whole = match content.as_bytes().first() {
                Some(b':') => colon_suffix_end(content, 0),
                Some(b'#') => anchor_suffix_end(content, 0),
                _ => None,
            };
            whole == Some(content.len())
        })
        .collect()
}

/// Each inline code span in `line`, by the range of its content, as Markdown reads one:
/// a run of backticks, then the text up to the next run of exactly as many, with one space
/// taken off each end when the text both begins and ends with one and is not all spaces.
/// A run that no run of its length closes is literal text (067 AC5).
fn code_spans(line: &str) -> Vec<Range<usize>> {
    let bytes = line.as_bytes();
    let run_at = |at: usize| bytes[at..].iter().take_while(|&&byte| byte == b'`').count();
    let mut spans = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] != b'`' {
            at += 1;
            continue;
        }
        let width = run_at(at);
        let start = at + width;
        let mut close = start;
        let mut end = None;
        while close < bytes.len() {
            if bytes[close] != b'`' {
                close += 1;
                continue;
            }
            let run = run_at(close);
            if run == width {
                end = Some(close);
                break;
            }
            close += run;
        }
        let Some(end) = end else {
            at = start;
            continue;
        };
        let text = &line[start..end];
        let padded = text.len() >= 2
            && text.starts_with(' ')
            && text.ends_with(' ')
            && !text.bytes().all(|byte| byte == b' ');
        spans.push(if padded {
            start + 1..end - 1
        } else {
            start..end
        });
        at = end + width;
    }
    spans
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

/// The most citations on one line [`strips_to`] tries one by one, keeping or stripping
/// each; past it only stripping every one counts, which bounds the search at
/// `2^MAX_ONE_BY_ONE` alignments.
const MAX_ONE_BY_ONE: usize = 12;

/// Whether `new` is `old` with the suffixes of one or more of its citations removed and
/// nothing else changed, each citation kept or stripped on its own (066 AC6), so removing
/// one beside a kept one is a strip, an anchor or an en-dash range included. A citation
/// renumbered, moved or added, and any other change, is not.
pub(crate) fn strips_to(old: &str, new: &str) -> bool {
    if old == new {
        return false;
    }
    let citations = find(old);
    if citations.is_empty() {
        return false;
    }
    if citations.len() > MAX_ONE_BY_ONE {
        return strip(old) == new;
    }
    aligns(old, new, &citations, 0, 0)
}

/// Whether `old[old_at..]` becomes `new[new_at..]` with each of `citations`' suffixes
/// either kept as it is or removed.
fn aligns(old: &str, new: &str, citations: &[Citation], old_at: usize, new_at: usize) -> bool {
    let Some((first, rest)) = citations.split_first() else {
        return new.get(new_at..) == old.get(old_at..);
    };
    let before = &old[old_at..first.suffix.start];
    if !new
        .get(new_at..)
        .is_some_and(|tail| tail.starts_with(before))
    {
        return false;
    }
    let new_at = new_at + before.len();
    let suffix = &old[first.suffix.clone()];
    let kept = new
        .get(new_at..)
        .is_some_and(|tail| tail.starts_with(suffix))
        && aligns(old, new, rest, first.suffix.end, new_at + suffix.len());
    kept || aligns(old, new, rest, first.suffix.end, new_at)
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

/// The end of a `#LN` or `#LN-LM` anchor starting at `hash`. A range's second
/// `L` is required, so `#L12-20` holds only the anchor `#L12` (owner decision,
/// 2026-11-18): stripping all of it changes more than a citation.
fn anchor_suffix_end(line: &str, hash: usize) -> Option<usize> {
    if !line[hash..].starts_with("#L") {
        return None;
    }
    let mut end = digits_end(line, hash + 2)?;
    if line[end..].starts_with("-L")
        && let Some(range_end) = digits_end(line, end + 2)
    {
        end = range_end;
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

    /// Any subset of a line's citations may be stripped, each on its own; a
    /// citation renumbered, moved or added, or any other change, is no strip.
    #[test]
    fn a_strip_of_some_citations_is_a_strip() {
        for (old, new) in [
            (
                "See docs/w.md#L10 and a/b.rs:3 here.",
                "See docs/w.md and a/b.rs:3 here.",
            ),
            ("y.md:3–9 and a.rs:1", "y.md and a.rs:1"),
            ("a.rs:1 b.rs:2", "a.rs:1 b.rs"),
            ("a.rs:1 b.rs:2", "a.rs b.rs"),
            ("[w](w.md#L10-L20), `q.md#L3`", "[w](w.md), `q.md#L3`"),
        ] {
            assert!(strips_to(old, new), "{old} -> {new}");
        }
        for (old, new) in [
            ("a.rs:12 b.rs:3", "a.rs:3 b.rs"),
            ("a.rs:12", "a.rs:14"),
            ("a.rs:12", "a.rs:12"),
            ("a.rs", "a.rs:12"),
            ("a.rs:12 then x", "a.rs then y"),
            ("db.internal:5432", "db.internal"),
        ] {
            assert!(!strips_to(old, new), "{old} -> {new}");
        }
    }

    /// A line-range anchor names its second line with an `L`, as `#LN-LM`: in
    /// `#L12-20` only `#L12` is a citation, so stripping the whole of it is no strip.
    #[test]
    fn a_range_anchor_needs_its_second_l() {
        assert_eq!(suffixes("[it](foo.rs#L12-20)"), vec!["#L12"]);
        assert!(!strips_to(
            "See [it](a.rs#L12-20) here.",
            "See [it](a.rs) here."
        ));
        assert!(strips_to(
            "See [it](a.rs#L12-L20) here.",
            "See [it](a.rs) here."
        ));
    }

    /// Past the bound on citations tried one by one, only stripping every one is a
    /// strip, so the search stays bounded.
    #[test]
    fn past_the_bound_only_a_whole_strip_counts() {
        let line = |count: usize, cited: bool| {
            (0..count)
                .map(|n| {
                    if cited {
                        format!("f{n}.rs:{n}")
                    } else {
                        format!("f{n}.rs")
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        let old = line(MAX_ONE_BY_ONE + 1, true);
        assert!(strips_to(&old, &line(MAX_ONE_BY_ONE + 1, false)));
        assert!(!strips_to(&old, &old.replacen("f0.rs:0", "f0.rs", 1)));
        let within = line(MAX_ONE_BY_ONE, true);
        assert!(strips_to(&within, &within.replacen("f0.rs:0", "f0.rs", 1)));
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

    /// A code span of any backtick run is read, as Markdown reads one, its padding
    /// spaces taken off, and a run no run of its length closes is literal (067 AC5).
    #[test]
    fn a_set_apart_suffix_in_a_longer_backtick_run_is_found() {
        assert_eq!(set_apart("see ``:12`` and ``#L4``"), vec![":12", "#L4"]);
        assert_eq!(set_apart("`` :12 ``"), vec![":12"]);
        assert_eq!(set_apart("`:3` then ```:5```"), vec![":3", ":5"]);
        for line in ["``a`:3``", "``:12`", "`` ``", "```:12``"] {
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
