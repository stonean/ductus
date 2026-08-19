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

/// Scan `content` and return the body lines eligible to carry an
/// edge-inducing link.
///
/// Excludes, in order: the leading frontmatter block, fenced code blocks,
/// blockquote-prefixed lines, and `## See also` regions. A file with no
/// frontmatter is scanned in full — the absence of frontmatter is a
/// validation concern that `validate-frontmatter` owns, and refusing to
/// harvest here would silently drop every link in the file.
pub(crate) fn harvestable_lines(content: &str) -> Vec<HarvestLine<'_>> {
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
        // and their forward-pointer links are navigation, not dependencies.
        if trimmed.starts_with('>') {
            continue;
        }

        // `## See also` region toggling, evaluated before the region skip so
        // the heading that *closes* the region is itself outside it.
        if let Some((level, text)) = heading_parts(line)
            && level <= 2
        {
            in_see_also = text.eq_ignore_ascii_case("see also");
            continue;
        }
        if in_see_also {
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
        assert!(texts(content).is_empty());
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
    fn crlf_lines_are_normalized_before_matching() {
        let content = "---\r\nx: 1\r\n---\r\n## See also\r\n- ](../001-a/spec.md)\r\n";
        assert!(texts(content).is_empty());
    }
}
