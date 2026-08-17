//! Mechanical-sweep detection — "did this file change by more than spelling?"
//!
//! §spec-lifecycle case (a) calls a uniform token substitution across live
//! artifacts a **mechanical** edit: it is why a rename sweep does not reopen a
//! `done` spec. A review reads contracts, and a contract that changed only in
//! spelling states what it stated before — so the same rule has to hold for
//! review staleness. Without it the two rules disagree, and one repo-wide
//! rename turns every done spec's review stale at once.
//!
//! That is not hypothetical. `/{project}:audit` Family 19 has carried this
//! exemption since it was written; `check-review-gate`'s staleness block did
//! not, and the two answered differently for **19 of this repo's 46 `done`
//! specs** — every one of those 19 a consequence of 049's `govern → ductus`
//! sweep and none a real contract change. The scenario that exposed it was
//! itself a false positive: `017-derive-dont-ask` blocked on three contracts
//! that changed in exactly one commit, the rename.
//!
//! The rule, in three parts, each of which a real edit breaks:
//!
//! 1. **One-for-one lines.** A substitution replaces a run of lines with the
//!    same number of lines; adding or dropping one is structural.
//! 2. **Same token count per line.** A rewrite that changes how many tokens a
//!    line has is not a substitution.
//! 3. **Uniform repo-wide.** A pair must appear in more than one file. This is
//!    the part that keeps the check honest: a one-cell data-model edit
//!    (`| timeout | 30s |` → `| timeout | 60s |`) rewrites one token in one
//!    file and reads as perfectly uniform on its own. Requiring two files
//!    separates a rename from a contract change.
//!
//! Substitutions may **collapse** — 049 sent both `govern` and `gvrn` to
//! `ductus` — so the rewrite need not be invertible. A collapse that is not a
//! rename (every `MUST` and `MAY` rewritten to `SHOULD`) still fails the
//! repo-wide test.
//!
//! Everything here is derived from the diff. A commit trailer or an opt-out
//! flag would make correctness depend on an author remembering to set it,
//! which `AGENTS.md`'s second Design Principle rules out.
//!
//! This is the Rust half of a rule with two enforcement moments. Family 19
//! applies the same rule at release time from `scripts/audit/review-freshness.sh`,
//! which runs in a toolchain-free CI job and so cannot call this code; the
//! `mechanical_sweep_parity` integration test pins the two to agree over the
//! real corpus, so a divergence fails a test rather than going unnoticed.
//!
//! Defined by
//! `specs/022-deterministic-runtime/scenarios/review-staleness-on-done-specs.md`.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use git2::{Delta, DiffOptions, Repository, Tree};

/// One token rewrite: the token as it read before, and as it reads now.
type Pair = (String, String);

/// Per-file token rewrites for one `base..HEAD` window, plus the subset that
/// appears in more than one file.
///
/// A file maps to `None` when its diff is **not** a pure substitution — that
/// file changed structurally and no exemption can apply to it.
pub struct SweepIndex {
    per_file: BTreeMap<String, Option<BTreeSet<Pair>>>,
    repo_wide: BTreeSet<Pair>,
}

impl SweepIndex {
    /// The index for a diff this code could not read: no entries, so every
    /// path reports [`changed_beyond_spelling`] and no exemption is granted.
    ///
    /// Every failure branch in [`build`] returns *this*, so "could not examine
    /// the diff" has one representation rather than one per branch. The
    /// direction is deliberate and is the opposite of the enclosing gate's:
    /// `check-review-gate` fails **open** on what it cannot determine, but an
    /// exemption is a claim that a contract did not really change, and a claim
    /// has to be earned from a diff that was actually read. Failing open here
    /// would mean granting exemptions on the strength of a walk that never
    /// finished (`QUAL-CLAIM-001`).
    ///
    /// [`build`]: SweepIndex::build
    /// [`changed_beyond_spelling`]: SweepIndex::changed_beyond_spelling
    fn unreadable() -> Self {
        Self {
            per_file: BTreeMap::new(),
            repo_wide: BTreeSet::new(),
        }
    }

    /// Build the index from one `git diff --unified=0 base..HEAD -- '*.md'`.
    ///
    /// One diff per base rather than two blob reads per changed file: this
    /// runs inside a release gate, and the blob-per-file shape took a minute
    /// on this repo's history.
    #[must_use]
    pub fn build(repo: &Repository, base: &Tree<'_>, head: &Tree<'_>) -> Self {
        let mut opts = DiffOptions::new();
        opts.context_lines(0).pathspec("*.md");
        let Ok(diff) = repo.diff_tree_to_tree(Some(base), Some(head), Some(&mut opts)) else {
            return Self::unreadable();
        };

        // git2 delivers file / hunk / line callbacks as separate closures, so
        // the shared state they all drive lives in one cell rather than being
        // threaded through three borrows.
        let events: RefCell<Vec<Event>> = RefCell::new(Vec::new());
        let walked = diff.foreach(
            &mut |delta, _| {
                // A deleted file's patch header names no new path
                // (`+++ /dev/null`). Treating it as a path would let its
                // removed lines accumulate against the previous file and mark
                // that file structural — a pure rename reported stale because
                // something unrelated was deleted in the same window.
                let path = if delta.status() == Delta::Deleted {
                    None
                } else {
                    delta
                        .new_file()
                        .path()
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                };
                events.borrow_mut().push(Event::File(path));
                true
            },
            None,
            Some(&mut |_, _| {
                events.borrow_mut().push(Event::Hunk);
                true
            }),
            Some(&mut |_, _, line| {
                let text = String::from_utf8_lossy(line.content())
                    .trim_end_matches(['\n', '\r'])
                    .to_string();
                match line.origin() {
                    '-' => events.borrow_mut().push(Event::Del(text)),
                    '+' => events.borrow_mut().push(Event::Add(text)),
                    _ => {}
                }
                true
            }),
        );

        // A walk that aborted partway still leaves `events` holding the prefix
        // git2 delivered, and folding that prefix is worse than folding
        // nothing: a file whose leading hunk was a rename but whose later
        // hunks were structural would land in `per_file` as a pure
        // substitution and read as **exempt**, so the gate would call the
        // review current on a diff nobody finished reading. Discard the
        // partial index rather than infer from it.
        if walked.is_err() {
            return Self::unreadable();
        }

        let per_file = fold_events(&events.into_inner());
        let mut counts: BTreeMap<&Pair, usize> = BTreeMap::new();
        for pairs in per_file.values().flatten() {
            for pair in pairs {
                *counts.entry(pair).or_insert(0) += 1;
            }
        }
        let repo_wide = counts
            .into_iter()
            .filter(|(_, n)| *n > 1)
            .map(|(pair, _)| pair.clone())
            .collect();

        Self {
            per_file,
            repo_wide,
        }
    }

    /// Whether `path` differs from the base by more than a repo-wide rename.
    ///
    /// A path absent from the index was renamed, added, or deleted — a real
    /// contract change. A path whose diff is not a pure substitution is a real
    /// change. A path with no rewrites at all changed only in ways the diff
    /// does not show as token edits, and is exempt.
    #[must_use]
    pub fn changed_beyond_spelling(&self, path: &str) -> bool {
        let Some(entry) = self.per_file.get(path) else {
            return true;
        };
        let Some(pairs) = entry else {
            return true;
        };
        if pairs.is_empty() {
            return false;
        }
        !pairs
            .iter()
            .all(|pair| self.repo_wide.contains(pair) || self.explained_by(pair))
    }

    /// Whether a file-local rewrite follows from the repo-wide ones.
    ///
    /// A rename produces token variants that occur in a single file — 049's
    /// sweep rewrote `gvrn_` to `ductus_` in exactly one data model — and
    /// those are consequences of the repo-wide rewrite, not separate edits. A
    /// changed table cell (`30s` → `60s`) is derivable from no repo-wide
    /// rewrite, which is what keeps it a finding.
    fn explained_by(&self, pair: &Pair) -> bool {
        let (old, new) = pair;
        let mut rewritten = old.clone();
        // Longest first, so a shorter rewrite cannot pre-empt a longer one.
        let mut ordered: Vec<&Pair> = self.repo_wide.iter().collect();
        ordered.sort_by_key(|(o, _)| std::cmp::Reverse(o.len()));
        for (from, to) in ordered {
            rewritten = rewritten.replace(from.as_str(), to);
        }
        &rewritten == new
    }
}

/// One parsed line of the unified diff, in delivery order.
enum Event {
    File(Option<String>),
    Hunk,
    Del(String),
    Add(String),
}

/// Replay the diff events into per-file rewrite sets.
fn fold_events(events: &[Event]) -> BTreeMap<String, Option<BTreeSet<Pair>>> {
    let mut per_file: BTreeMap<String, Option<BTreeSet<Pair>>> = BTreeMap::new();
    let mut path: Option<String> = None;
    let mut old_run: Vec<&str> = Vec::new();
    let mut new_run: Vec<&str> = Vec::new();

    for event in events {
        match event {
            Event::File(next) => {
                flush(&mut per_file, path.as_deref(), &mut old_run, &mut new_run);
                path.clone_from(next);
                if let Some(p) = &path {
                    per_file
                        .entry(p.clone())
                        .or_insert_with(|| Some(BTreeSet::new()));
                }
            }
            Event::Hunk => flush(&mut per_file, path.as_deref(), &mut old_run, &mut new_run),
            Event::Del(text) => {
                // A new run started, so the previous pairing is closed.
                if !new_run.is_empty() {
                    flush(&mut per_file, path.as_deref(), &mut old_run, &mut new_run);
                }
                old_run.push(text);
            }
            Event::Add(text) => new_run.push(text),
        }
    }
    flush(&mut per_file, path.as_deref(), &mut old_run, &mut new_run);
    per_file
}

/// Fold the accumulated run into `path`'s entry and clear it. A run that is
/// not a pure substitution poisons the file's entry to `None`.
fn flush(
    per_file: &mut BTreeMap<String, Option<BTreeSet<Pair>>>,
    path: Option<&str>,
    old_run: &mut Vec<&str>,
    new_run: &mut Vec<&str>,
) {
    if old_run.is_empty() && new_run.is_empty() {
        return;
    }
    if let Some(path) = path {
        let entry = per_file
            .entry(path.to_string())
            .or_insert_with(|| Some(BTreeSet::new()));
        // A file already known structural stays structural.
        if entry.is_some() {
            match pair_run(old_run, new_run) {
                Some(pairs) => {
                    if let Some(existing) = entry.as_mut() {
                        existing.extend(pairs);
                    }
                }
                None => *entry = None,
            }
        }
    }
    old_run.clear();
    new_run.clear();
}

/// Token rewrites for one removed/added line run, or `None` when the run is
/// not a pure substitution (see the module docs' conditions 1 and 2).
fn pair_run(old_run: &[&str], new_run: &[&str]) -> Option<BTreeSet<Pair>> {
    if old_run.len() != new_run.len() || old_run.is_empty() {
        return None;
    }
    let mut pairs = BTreeSet::new();
    for (old_line, new_line) in old_run.iter().zip(new_run.iter()) {
        let old_toks = tokenize(old_line);
        let new_toks = tokenize(new_line);
        if old_toks.len() != new_toks.len() {
            return None;
        }
        for (a, b) in old_toks.into_iter().zip(new_toks) {
            if a != b {
                pairs.insert((a.to_string(), b.to_string()));
            }
        }
    }
    Some(pairs)
}

/// Characters that form a single word token. Mirrors Family 19's
/// `[A-Za-z0-9_.:/-]+` character class exactly — the two implementations must
/// agree on what a token is, or they agree on nothing downstream.
fn is_token_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | ':' | '/' | '-')
}

/// Split a line into tokens: maximal word runs, maximal whitespace runs, and
/// any other character on its own. The hand-rolled form of Family 19's
/// `[A-Za-z0-9_.:/-]+|\s+|.`.
fn tokenize(line: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let bytes = line.as_bytes();
    let mut start = 0;
    while start < line.len() {
        // `start` always lands on a char boundary: each branch advances by
        // whole chars.
        let first = line[start..].chars().next().unwrap_or('\0');
        let end = if is_token_char(first) {
            advance_while(line, start, is_token_char)
        } else if first.is_whitespace() {
            advance_while(line, start, char::is_whitespace)
        } else {
            start + first.len_utf8()
        };
        tokens.push(&line[start..end]);
        start = end;
        debug_assert!(start <= bytes.len());
    }
    tokens
}

/// Byte offset just past the maximal run of chars satisfying `pred` from `from`.
fn advance_while(line: &str, from: usize, pred: fn(char) -> bool) -> usize {
    let mut end = from;
    for c in line[from..].chars() {
        if !pred(c) {
            break;
        }
        end += c.len_utf8();
    }
    end
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn pairs(old: &[&str], new: &[&str]) -> Option<Vec<Pair>> {
        pair_run(old, new).map(|set| set.into_iter().collect())
    }

    #[test]
    fn tokenize_matches_family_19_classes() {
        assert_eq!(
            tokenize("run `/ductus:review` now"),
            vec!["run", " ", "`", "/ductus:review", "`", " ", "now"]
        );
        // A word token absorbs `_ . : / -`; everything else stands alone.
        assert_eq!(tokenize("a-b.c/d:e_f"), vec!["a-b.c/d:e_f"]);
        assert_eq!(tokenize("x  y"), vec!["x", "  ", "y"]);
        assert_eq!(tokenize(""), Vec::<&str>::new());
        // Non-ASCII stands alone rather than panicking on a byte boundary.
        assert_eq!(tokenize("a — b"), vec!["a", " ", "—", " ", "b"]);
    }

    #[test]
    fn a_uniform_rename_is_a_substitution() {
        let got = pairs(&["Run /govern to sync"], &["Run /ductus to sync"]).unwrap();
        assert_eq!(got, vec![("/govern".to_string(), "/ductus".to_string())]);
    }

    #[test]
    fn a_run_that_adds_or_drops_a_line_is_structural() {
        assert!(pairs(&["one"], &["one", "two"]).is_none());
        assert!(pairs(&[], &[]).is_none());
    }

    #[test]
    fn a_line_whose_token_count_changes_is_structural() {
        // Inserting a word is not a rewrite of an existing token.
        assert!(pairs(&["the timeout is 30s"], &["the default timeout is 30s"]).is_none());
    }

    #[test]
    fn a_changed_table_cell_is_a_substitution_locally_but_not_repo_wide() {
        // Condition 3 is what catches this; locally it looks perfectly uniform.
        let got = pairs(&["| timeout | 30s |"], &["| timeout | 60s |"]).unwrap();
        assert_eq!(got, vec![("30s".to_string(), "60s".to_string())]);
    }

    /// A file's rewrite set in test-literal form: `None` marks a structural
    /// diff, `Some(pairs)` the token rewrites.
    type FileEntry<'a> = (&'a str, Option<&'a [(&'a str, &'a str)]>);

    /// Build an index by hand to exercise the classification rules without a
    /// git fixture; `SweepIndex::build`'s diff walk is covered by the
    /// integration tests and the corpus parity test.
    fn index(per_file: &[FileEntry<'_>], repo_wide: &[(&str, &str)]) -> SweepIndex {
        SweepIndex {
            per_file: per_file
                .iter()
                .map(|(path, pairs)| {
                    let value = pairs.map(|ps| {
                        ps.iter()
                            .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
                            .collect()
                    });
                    ((*path).to_string(), value)
                })
                .collect(),
            repo_wide: repo_wide
                .iter()
                .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
                .collect(),
        }
    }

    #[test]
    fn a_repo_wide_rewrite_is_exempt_and_a_local_one_is_not() {
        let idx = index(
            &[
                ("specs/005/data-model.md", Some(&[("govern", "ductus")])),
                ("specs/006/data-model.md", Some(&[("30s", "60s")])),
            ],
            &[("govern", "ductus")],
        );
        assert!(!idx.changed_beyond_spelling("specs/005/data-model.md"));
        assert!(idx.changed_beyond_spelling("specs/006/data-model.md"));
    }

    #[test]
    fn a_file_local_variant_of_a_repo_wide_rename_is_explained() {
        // 049 rewrote `gvrn_` to `ductus_` in exactly one data model; that is a
        // consequence of the repo-wide rewrite, not a separate edit.
        let idx = index(
            &[(
                "specs/005/data-model.md",
                Some(&[("gvrn_root", "ductus_root")]),
            )],
            &[("gvrn", "ductus")],
        );
        assert!(!idx.changed_beyond_spelling("specs/005/data-model.md"));
    }

    #[test]
    fn collapsing_renames_are_allowed_but_a_non_rename_collapse_is_not() {
        // `govern` and `gvrn` both → `ductus` is what a unifying rename does.
        let collapse = index(
            &[("a.md", Some(&[("govern", "ductus"), ("gvrn", "ductus")]))],
            &[("govern", "ductus"), ("gvrn", "ductus")],
        );
        assert!(!collapse.changed_beyond_spelling("a.md"));

        // MUST/MAY → SHOULD in one file only fails the repo-wide test.
        let meaning = index(
            &[("a.md", Some(&[("MUST", "SHOULD"), ("MAY", "SHOULD")]))],
            &[],
        );
        assert!(meaning.changed_beyond_spelling("a.md"));
    }

    #[test]
    fn structural_and_absent_files_are_never_exempt() {
        let idx = index(
            &[("structural.md", None), ("untouched.md", Some(&[]))],
            &[("govern", "ductus")],
        );
        assert!(idx.changed_beyond_spelling("structural.md"));
        assert!(
            idx.changed_beyond_spelling("renamed-or-added.md"),
            "a path absent from the index is a real change, not an exemption"
        );
        assert!(
            !idx.changed_beyond_spelling("untouched.md"),
            "a file in the diff with no token rewrites changed only in ways \
             the token view does not see"
        );
    }

    #[test]
    fn an_unreadable_diff_grants_no_exemptions() {
        // Both of `build`'s failure branches return this index, so the
        // property is asserted once here rather than per branch. Forcing a
        // real git2 failure is not reachable from a unit test — what is
        // testable, and what actually matters, is that the shape those
        // branches produce cannot hand out an exemption: an empty index has
        // no entry for any path, so every path is a real change and the
        // review stays stale. The alternative — folding a half-delivered
        // diff — would let a file whose structural hunks were dropped read
        // as a pure rename, which is `QUAL-CLAIM-001` in the machinery that
        // enforces it.
        let idx = SweepIndex::unreadable();
        for path in [
            "specs/005/data-model.md",
            "specs/007-gate/scenarios/retry.md",
            "",
        ] {
            assert!(
                idx.changed_beyond_spelling(path),
                "an unreadable diff must not exempt `{path}`"
            );
        }
    }

    #[test]
    fn longest_repo_wide_rewrite_wins_over_a_shorter_prefix() {
        // Applying `gvrn` → `x` before `gvrn_root` → `ductus_root` would
        // produce `x_root` and wrongly report the pair unexplained.
        let idx = index(
            &[("a.md", Some(&[("gvrn_root", "ductus_root")]))],
            &[("gvrn_root", "ductus_root"), ("gvrn", "x")],
        );
        assert!(!idx.changed_beyond_spelling("a.md"));
    }
}
