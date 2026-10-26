//! Shared body scanner for the two frontmatter-index generators.
//!
//! `derive-dependencies` and `derive-references` harvest different link
//! shapes out of the *same* region of a spec: the body, minus frontmatter,
//! minus fenced code, minus blockquote-prefixed lines, minus any `## See also`
//! section. Those four exclusions are the policy — they decide whether a link
//! induces an edge — so they live here once rather than once per primitive.
//!
//! This is the part the shell got wrong by construction. The two generators
//! carried the exclusion walk as two independent `awk` programs, and they
//! drifted: `gen-cross-service-refs.sh` grew the `## See also` opt-out and the
//! blockquote skip that `gen-spec-deps.sh` already had, and the frontmatter
//! rewrite splice diverged between them (one corrupted the block form of a
//! YAML list, the other did not) until a fix landed in one and not the other.
//! One scanner, two matchers, is what stops that recurring.
//!
//! ## The `## See also` opt-out
//!
//! A heading at level 1 or 2 whose text is exactly `See also`
//! (case-insensitively) opens an opt-out region; the next level-1 or level-2
//! heading closes it. Deeper subheadings inherit the region rather than ending
//! it, so a `### Related services` nested under `## See also` stays excluded.
//!
//! `## References` is deliberately **not** an opt-out: it is the formal
//! body-authored dependency section, and links under it are meant to induce
//! edges.

/// Whether `line` is a YAML frontmatter delimiter: `---` starting at column
/// zero, followed by nothing but whitespace.
///
/// The single definition of the fence, shared by the scanner and both
/// frontmatter splices. It exists because those three grew three *different*
/// tests during this change — a trimmed compare, a trim-start compare, and a
/// trim-end compare — each wrong in a different direction. The column-zero
/// anchor is the load-bearing part: an indented `---` inside a YAML block
/// scalar is content, not a delimiter, and treating it as one ends the
/// frontmatter early so a key below it is silently never rewritten. Trailing
/// whitespace, by contrast, *is* tolerated — `---   ` closes the block.
pub(crate) fn is_frontmatter_fence(line: &str) -> bool {
    line.strip_prefix("---")
        .is_some_and(|rest| rest.trim().is_empty())
}

/// Whether `content` opens a frontmatter block it never closes.
///
/// Both frontmatter splices anchor their write inside the block — one replaces
/// a key found there, the other inserts before the closing fence — so an
/// unterminated block leaves each of them with no anchor and nothing written.
/// Detected here rather than in either splice, because a second copy of this
/// test is the drift the shared fence predicate exists to prevent.
///
/// A file with **no** frontmatter at all is not unparseable: there is no block
/// to close and nothing is derived from it, so reporting every such file would
/// bury the signal this exists to raise.
pub(crate) fn has_unterminated_frontmatter(content: &str) -> bool {
    let mut lines = content.lines();
    match lines.next() {
        Some(first) if is_frontmatter_fence(first) => {}
        _ => return false,
    }
    !lines.any(is_frontmatter_fence)
}

/// Whether a frontmatter line continues the value of the key above it rather
/// than starting a new key: an indented line, or a block-sequence entry at
/// column zero (`- item`, or `-` alone with the item on the lines below).
///
/// The single definition of where a key's value ends, shared by both
/// splices. Indentation alone is not enough: YAML lets a block sequence under
/// a mapping key sit at the key's own indentation, and that indentless form is
/// what `PyYAML` and Ruby's Psych emit by default. Under an indent-only rule the
/// splice replaced the key line, stopped at the first `- item`, and left the
/// old items behind the new value, which is invalid YAML. A column-zero `-`
/// cannot start a key of a mapping, so it can only be a sequence entry.
///
/// The frontmatter fence is checked before this in both splices, so `---` is
/// never mistaken for an entry. A blank line is [`continues_key`]'s question,
/// since it belongs to whichever side the next non-blank line is on.
pub(crate) fn is_key_continuation(line: &str) -> bool {
    line.starts_with(' ') || line.starts_with('\t') || line == "-" || line.starts_with("- ")
}

/// Whether `lines[idx]` continues the value of the key above it: a line
/// carrying content that [`is_key_continuation`] accepts, or a line carrying
/// none (empty, whitespace only, or a comment at column zero) that the next
/// line carrying content continues.
///
/// YAML lets blank lines sit inside a block value (`dependencies:`, a blank
/// line, then `  - 002-b`), so a rule that ended the value at the first blank
/// line left the items beyond it behind: `derive-dependencies` wrote them
/// after the new list, which does not parse, and `derive-references` put its
/// block between the key and its items, which parses as a null
/// `dependencies:` and a reference that was a dependency. A blank line the
/// next key follows stays outside the value, so the spacing an author put
/// before that key survives the splice. A comment at column zero between two
/// entries is the same shape: `#` there cannot start a key, so it sits inside
/// the value when an entry follows and before the next key when one does. (An
/// indented comment is indented, so [`is_key_continuation`] already holds it.)
/// `lines` are the file's lines with any `\r` already stripped, as both
/// splices hold them.
pub(crate) fn continues_key(lines: &[&str], idx: usize) -> bool {
    let carries_content = |line: &str| !line.trim().is_empty() && !line.starts_with('#');
    let Some(line) = lines.get(idx) else {
        return false;
    };
    if carries_content(line) {
        return is_key_continuation(line);
    }
    lines
        .iter()
        .skip(idx + 1)
        .find(|next| carries_content(next))
        .is_some_and(|next| is_key_continuation(next))
}

/// Whether the leading frontmatter block of `content` parses as YAML, with
/// the block's bounds found by [`is_frontmatter_fence`], as both splices find
/// them.
///
/// The check a splice's output passes before it is written. A splice edits
/// lines, not YAML, so a value shape its line rules do not anticipate leaves a
/// block that no longer parses: a flow sequence whose closing `]` sits at
/// column zero, say. Writing that
/// block would corrupt a spec while reporting it updated. A file with no
/// frontmatter, or one whose block never closes, has nothing to judge here and
/// is `true`; the unterminated case is [`has_unterminated_frontmatter`]'s.
pub(crate) fn frontmatter_parses(content: &str) -> bool {
    let mut lines = content.lines();
    match lines.next() {
        Some(first) if is_frontmatter_fence(first) => {}
        _ => return true,
    }
    let mut block = String::new();
    for raw in lines {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if is_frontmatter_fence(line) {
            return serde_norway::from_str::<serde_norway::Value>(&block).is_ok();
        }
        block.push_str(line);
        block.push('\n');
    }
    true
}

/// A body line that survived every exclusion, with its 1-based line number.
///
/// The line number is carried because a caller reporting a malformed link
/// needs to name where it is; harvesting alone would not need it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HarvestLine<'a> {
    /// The line's text, verbatim (no trimming — a matcher may care about
    /// leading whitespace).
    pub text: &'a str,
    /// 1-based line number within the whole file, frontmatter included, so a
    /// citation matches what an editor shows.
    pub line: usize,
}

/// Which body region a scan covers.
///
/// The four exclusions differ in *why* they exist, and the difference only
/// becomes visible once a second question is asked of the same text. Removing
/// frontmatter and fenced code is about what is prose at all; removing
/// blockquotes and `## See also` is a policy about which prose links induce an
/// **edge**. A caller asking whether a reader meets a pointer wants the first
/// pair applied and the second pair not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum LinkScope {
    /// Lines eligible to carry an edge-inducing link: frontmatter, fenced
    /// code, blockquote-prefixed lines, and `## See also` regions all
    /// excluded. What `derive-dependencies` and `derive-references` harvest.
    Edges,
    /// Every line a reader meets a pointer on: frontmatter and fenced code
    /// excluded, blockquotes and `## See also` **kept**.
    ///
    /// The pre-`done` gate's cross-spec-impact check reads this scope. A
    /// signpost on a `done` spec is written as a blockquote in this corpus
    /// (`> **Signpost:** …`), which is exactly the artifact
    /// §cross-spec-impact prescribes for recording a cross-spec change — so
    /// harvesting blockquotes out would make the canonical discharge
    /// artifact invisible to the gate built to require it. The edge policy
    /// is right for its own question and wrong for this one, which is why
    /// this is a scope rather than a change to the exclusions.
    Pointers,
}

/// Scan `content` and return the body lines eligible to carry an
/// edge-inducing link. [`LinkScope::Edges`]; see [`scan_body`].
pub(crate) fn harvestable_lines(content: &str) -> Vec<HarvestLine<'_>> {
    scan_body(content, LinkScope::Edges)
}

/// Scan `content` and return every body line a reader meets a pointer on.
/// [`LinkScope::Pointers`]; see [`scan_body`].
pub(crate) fn pointer_lines(content: &str) -> Vec<HarvestLine<'_>> {
    scan_body(content, LinkScope::Pointers)
}

/// Scan `content` and return the body lines inside `scope`.
///
/// Excludes, in order: the leading frontmatter block, fenced code blocks, and
/// — under [`LinkScope::Edges`] only — blockquote-prefixed lines and
/// `## See also` regions. A file with no frontmatter is scanned in full — the
/// absence of frontmatter is a validation concern that `validate-frontmatter`
/// owns, and refusing to harvest here would silently drop every link in the
/// file.
fn scan_body(content: &str, scope: LinkScope) -> Vec<HarvestLine<'_>> {
    let mut out = Vec::new();
    let mut fm_seen = false;
    let mut in_fm = false;
    let mut in_fence = false;
    let mut in_see_also = false;

    for (idx, raw) in content.lines().enumerate() {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        let trimmed = line.trim_start();

        // Frontmatter delimiters. Only a `---` on the very first line opens
        // the block; a later one closes it. A `---` after the block has
        // closed is an ordinary horizontal rule and falls through.
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

        // Fenced code. The toggle line itself is never harvestable.
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        // Blockquote-prefixed lines. Signposts on done specs use blockquotes,
        // and their forward-pointer links are navigation, not dependencies —
        // which is why `Pointers` keeps them: navigation is precisely what
        // that scope is asking about.
        if scope == LinkScope::Edges && trimmed.starts_with('>') {
            continue;
        }

        // `## See also` region toggling, evaluated before the region skip so
        // the heading that *closes* the region is itself outside it. Tracked
        // under both scopes and acted on under `Edges` only, so the region
        // state cannot depend on which caller asked.
        if let Some((level, text)) = heading_parts(line)
            && level <= 2
        {
            in_see_also = text.eq_ignore_ascii_case("see also");
            continue;
        }
        if scope == LinkScope::Edges && in_see_also {
            continue;
        }

        out.push(HarvestLine {
            text: line,
            line: idx + 1,
        });
    }
    out
}

/// Split an ATX heading into `(level, trimmed text)`, or `None` when the line
/// is not a heading.
///
/// Deliberately local rather than reusing `parse_atx_heading`: that helper
/// requires a space after the hashes and returns an owned `String`, while the
/// opt-out rule counts hashes on any `#`-prefixed line and only needs a
/// borrow. Keeping them separate avoids widening a helper five other
/// primitives depend on.
fn heading_parts(line: &str) -> Option<(usize, &str)> {
    let rest = line.strip_prefix('#')?;
    let extra = rest.len() - rest.trim_start_matches('#').len();
    let level = 1 + extra;
    let text = rest[extra..].trim();
    // A bare `#####` with no text is still a heading for level-counting
    // purposes; an empty text simply never matches "see also".
    Some((level, text))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(content: &str) -> Vec<&str> {
        harvestable_lines(content)
            .into_iter()
            .map(|h| h.text)
            .collect()
    }

    #[test]
    fn frontmatter_is_excluded() {
        let content = "---\nstatus: done\ndependencies: [001-a]\n---\nbody link\n";
        assert_eq!(texts(content), vec!["body link"]);
    }

    #[test]
    fn a_horizontal_rule_after_frontmatter_is_not_a_delimiter() {
        // The `---` on line 6 is a rule, not a frontmatter fence. Treating it
        // as one would reopen the block and swallow the rest of the file, so
        // what matters is that `after` survives; the rule line itself stays a
        // body line (it simply never carries a link).
        let content = "---\nstatus: done\n---\nbefore\n\n---\n\nafter\n";
        assert_eq!(texts(content), vec!["before", "", "---", "", "after"]);
    }

    #[test]
    fn fenced_code_is_excluded_including_the_fence_lines() {
        let content = "---\nx: 1\n---\nkeep\n```\n](../001-a/spec.md)\n```\nalso keep\n";
        assert_eq!(texts(content), vec!["keep", "also keep"]);
    }

    #[test]
    fn an_unterminated_fence_excludes_to_end_of_file() {
        let content = "---\nx: 1\n---\nkeep\n```\n](../001-a/spec.md)\n";
        assert_eq!(texts(content), vec!["keep"]);
    }

    #[test]
    fn blockquote_lines_are_excluded() {
        let content = "---\nx: 1\n---\nkeep\n> quoted ](../001-a/spec.md)\n  > indented too\n";
        assert_eq!(texts(content), vec!["keep"]);
    }

    #[test]
    fn see_also_region_is_excluded_and_ends_at_the_next_h2() {
        let content = "\
---
x: 1
---
before

## See also

- ](../001-a/spec.md)

## References

- ](../002-b/spec.md)
";
        let kept = texts(content);
        assert!(kept.contains(&"before"));
        assert!(kept.contains(&"- ](../002-b/spec.md)"));
        assert!(!kept.contains(&"- ](../001-a/spec.md)"));
    }

    #[test]
    fn see_also_matches_case_insensitively_and_at_level_one() {
        let content = "---\nx: 1\n---\n# SEE ALSO\n- ](../001-a/spec.md)\n";
        assert_eq!(texts(content), [] as [&str; 0]);
    }

    #[test]
    fn deeper_subheadings_inherit_the_see_also_region() {
        let content = "\
---
x: 1
---
## See also

### Related services

- ](../001-a/spec.md)
";
        assert!(!texts(content).contains(&"- ](../001-a/spec.md)"));
    }

    #[test]
    fn references_is_not_an_opt_out() {
        let content = "---\nx: 1\n---\n## References\n- ](../001-a/spec.md)\n";
        assert!(texts(content).contains(&"- ](../001-a/spec.md)"));
    }

    #[test]
    fn line_numbers_are_one_based_over_the_whole_file() {
        let content = "---\nx: 1\n---\nfirst\n";
        let lines = harvestable_lines(content);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].line, 4);
    }

    #[test]
    fn a_file_without_frontmatter_is_scanned_in_full() {
        let content = "just a body\n](../001-a/spec.md)\n";
        assert_eq!(texts(content).len(), 2);
    }

    #[test]
    fn an_unterminated_frontmatter_block_is_detected() {
        assert!(has_unterminated_frontmatter("---\nstatus: done\n\nbody\n"));
        assert!(has_unterminated_frontmatter("---\nstatus: done\n"));
    }

    #[test]
    fn a_closed_block_and_a_file_without_frontmatter_are_both_parseable() {
        assert!(!has_unterminated_frontmatter(
            "---\nstatus: done\n---\nbody\n"
        ));
        // No block to close: not unparseable, or every plain markdown file
        // would be reported and bury the real signal.
        assert!(!has_unterminated_frontmatter("just a body\n"));
        assert!(!has_unterminated_frontmatter(""));
    }

    #[test]
    fn an_indented_fence_does_not_close_the_block() {
        // The column-zero anchor matters here too: a `---` inside a block
        // scalar must not make an unterminated block look terminated.
        assert!(has_unterminated_frontmatter(
            "---\ndescription: |\n  ---\n  more\n"
        ));
    }

    #[test]
    fn crlf_lines_are_normalized_before_matching() {
        let content = "---\r\nx: 1\r\n---\r\n## See also\r\n- ](../001-a/spec.md)\r\n";
        assert_eq!(texts(content), [] as [&str; 0]);
    }

    #[test]
    fn indented_lines_and_column_zero_entries_continue_a_key() {
        for line in ["  - 002-b", "\t- 002-b", "    spec: 003-u", "- 002-b", "-"] {
            assert!(is_key_continuation(line), "{line:?} continues the key");
        }
        for line in ["next: 3", "references:", "-x: 1", "--- ", "# note", ""] {
            assert!(!is_key_continuation(line), "{line:?} is not a continuation");
        }
    }

    /// A blank line, or a comment at column zero, belongs to the side the next
    /// line carrying content is on: inside the value when an entry follows,
    /// outside it when a key or the fence does. A whitespace-only line is
    /// blank, not indented.
    #[test]
    fn a_blank_line_continues_a_key_only_when_an_entry_follows_it() {
        let lines = [
            "dependencies:", // 0
            "",              // 1: an indented entry follows
            "  - 002-b",     // 2
            "",              // 3: a column-zero entry follows, past two blanks
            "   ",           // 4
            "- 004-d",       // 5
            "# a note",      // 6: a column-zero comment, an entry follows
            "",              // 7
            "- 005-e",       // 8
            "",              // 9: a key follows
            "  ",            // 10: whitespace only, a key follows
            "# about next",  // 11: a column-zero comment, a key follows
            "next: 3",       // 12
            "",              // 13: the fence follows
            "---",           // 14
            "",              // 15: nothing follows
        ];
        let continues: Vec<bool> = (0..lines.len()).map(|i| continues_key(&lines, i)).collect();
        assert_eq!(
            continues,
            [
                false, true, true, true, true, true, true, true, true, false, false, false, false,
                false, false, false
            ]
        );
        assert!(!continues_key(&lines, lines.len()), "past the end");
    }

    #[test]
    fn frontmatter_parses_judges_the_block_between_the_fences() {
        assert!(frontmatter_parses(
            "---\nstatus: done\ndependencies: [001-a]\n---\nbody: [\n"
        ));
        // The fence rule is the splices': trailing whitespace closes the block,
        // so the body below it, and the rule `---` further down, are not read.
        assert!(frontmatter_parses(
            "---\r\nstatus: done\r\n---   \r\nbody: [\r\n\r\n---\r\n"
        ));
        assert!(!frontmatter_parses(
            "---\nstatus: done\ndependencies: [002-b]\n- 009-old\n---\nbody\n"
        ));
        // Nothing to judge: no block at all.
        assert!(frontmatter_parses("just a body: [\n"));
    }
}
