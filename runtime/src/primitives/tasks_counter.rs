//! The `next-task:` counter in a `tasks.md`'s frontmatter (spec 041, scenario
//! `a-pruned-task-number-is-never-reissued`).
//!
//! The number the next appended task takes. `prune-tasks` writes it past the
//! highest number in the file before removing sections, and `append-task`
//! advances it, so a number a prune removed is never assigned again. Absent
//! means "highest present plus one", the numbering a never-pruned file has
//! always had. Every other `tasks.md` reader scans for headings and checkboxes
//! and never sees a frontmatter line, so the block is invisible to them.

use std::path::Path;

use crate::primitives::{PrimitiveError, Result, TaskNumber, iter_task_numbers_at_levels};

const FIELD: &str = "next-task:";

/// The byte offset just past the frontmatter block's closing fence line, or
/// `None` when the file does not open with `---`.
fn frontmatter_end(content: &str, path: &Path) -> Result<Option<usize>> {
    if !(content.starts_with("---\n") || content.starts_with("---\r\n")) {
        return Ok(None);
    }
    let mut offset = 0;
    for (idx, line) in content.split_inclusive('\n').enumerate() {
        offset += line.len();
        if idx > 0 && line.trim_end() == "---" {
            return Ok(Some(offset));
        }
    }
    Err(PrimitiveError::MalformedTasks {
        path: path.to_path_buf(),
        reason: "the frontmatter block is never closed by a `---` line".to_string(),
    })
}

/// The frontmatter block `content` opens with, fences included, `""` when it
/// has none.
///
/// # Errors
///
/// [`PrimitiveError::MalformedTasks`] when the block never closes.
pub(crate) fn frontmatter<'a>(content: &'a str, path: &Path) -> Result<&'a str> {
    Ok(frontmatter_end(content, path)?.map_or("", |end| &content[..end]))
}

/// The counter `content` carries, `None` when it has none.
///
/// # Errors
///
/// [`PrimitiveError::MalformedTasks`] when the frontmatter never closes or the
/// stored value is not a positive integer: a counter that cannot be read must
/// not look like an absent one, since the pass would then reissue a number.
pub(crate) fn read(content: &str, path: &Path) -> Result<Option<u32>> {
    let Some(end) = frontmatter_end(content, path)? else {
        return Ok(None);
    };
    for line in content[..end].lines().skip(1) {
        let Some(rest) = line.trim_start().strip_prefix(FIELD) else {
            continue;
        };
        return match rest.trim().parse::<u32>() {
            Ok(value) if value >= 1 => Ok(Some(value)),
            _ => Err(PrimitiveError::MalformedTasks {
                path: path.to_path_buf(),
                reason: format!("`next-task: {}` is not a positive integer", rest.trim()),
            }),
        };
    }
    Ok(None)
}

/// The highest task number's digits present in `content` at either task
/// level, `0` when there is none.
pub(crate) fn highest_present(content: &str) -> u32 {
    iter_task_numbers_at_levels(content, &[2, 3])
        .map(TaskNumber::digits)
        .max()
        .unwrap_or(0)
}

/// `content` with its counter set to `value`: the existing line replaced, a
/// line added to existing frontmatter, or a block created above the file.
///
/// # Errors
///
/// [`PrimitiveError::MalformedTasks`] when the frontmatter never closes.
pub(crate) fn set(content: &str, value: u32, path: &Path) -> Result<String> {
    let line = format!("{FIELD} {value}");
    let Some(end) = frontmatter_end(content, path)? else {
        return Ok(format!("---\n{line}\n---\n\n{content}"));
    };
    let newline = if content.starts_with("---\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut out = String::with_capacity(content.len() + line.len() + 2);
    let mut offset = 0;
    let mut written = false;
    for (idx, text) in content[..end].split_inclusive('\n').enumerate() {
        let closing = idx > 0 && offset + text.len() == end;
        if idx > 0 && !closing && text.trim_start().starts_with(FIELD) {
            out.push_str(&line);
            out.push_str(newline);
            written = true;
        } else {
            if closing && !written {
                out.push_str(&line);
                out.push_str(newline);
            }
            out.push_str(text);
        }
        offset += text.len();
    }
    out.push_str(&content[end..]);
    Ok(out)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn p() -> &'static Path {
        Path::new("specs/x/tasks.md")
    }

    #[test]
    fn reads_the_counter_and_treats_none_as_absent() {
        assert_eq!(read("# T\n", p()).unwrap(), None);
        assert_eq!(
            read("---\nnext-task: 7\n---\n\n# T\n", p()).unwrap(),
            Some(7)
        );
        assert_eq!(read("---\nother: 1\n---\n\n# T\n", p()).unwrap(), None);
    }

    #[test]
    fn an_unreadable_counter_is_an_error_not_an_absent_one() {
        for bad in [
            "---\nnext-task: soon\n---\n\n# T\n",
            "---\nnext-task: 0\n---\n\n# T\n",
            "---\nnext-task: 3\n\n# T\n",
        ] {
            assert!(
                matches!(read(bad, p()), Err(PrimitiveError::MalformedTasks { .. })),
                "{bad}"
            );
        }
    }

    #[test]
    fn set_creates_replaces_and_adds() {
        assert_eq!(
            set("# T\n", 4, p()).unwrap(),
            "---\nnext-task: 4\n---\n\n# T\n"
        );
        assert_eq!(
            set("---\nnext-task: 2\n---\n\n# T\n", 9, p()).unwrap(),
            "---\nnext-task: 9\n---\n\n# T\n"
        );
        assert_eq!(
            set("---\nother: x\n---\n\n# T\n", 9, p()).unwrap(),
            "---\nother: x\nnext-task: 9\n---\n\n# T\n"
        );
        assert_eq!(
            set("---\r\nnext-task: 2\r\n---\r\n\r\n# T\r\n", 3, p()).unwrap(),
            "---\r\nnext-task: 3\r\n---\r\n\r\n# T\r\n"
        );
    }

    #[test]
    fn highest_present_reads_both_levels_and_suffixes() {
        assert_eq!(highest_present("# T\n\n## 2. a\n\n### 11a. b\n"), 11);
        assert_eq!(highest_present("# T\n"), 0);
    }
}
