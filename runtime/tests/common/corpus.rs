//! A generated fixture corpus: a git-initialized project of N specs whose
//! count and per-spec size both scale (spec 080, AC7, AC14, AC24).
//!
//! Both dimensions scale because a result can grow with either: `dashboard`
//! grows with the number of specs, and `read-tasks` with the size of one
//! `tasks.md`. Spec 001 is the subject a per-feature primitive is pointed at,
//! and it is `in-progress`, so the review, analysis and implement primitives
//! see a spec with a window and work in it.
//!
//! History is three commits: every spec at `planned`; then work on each spec
//! that has started, with its status moved to `in-progress` and its source
//! files written; then the `done` specs closed. The subject and the `done`
//! specs have every task ticked, since the review and analysis writers refuse
//! a spec with an unchecked box; the other started specs have half.
//! That gives the history readers (`measure-reopenings`, `diff-cross-spec`,
//! `compute-review-scope`) a window to read, scaled with the count.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use git2::{IndexAddOption, Repository, Signature};

/// How large a generated corpus is.
#[derive(Clone, Copy, Debug)]
pub struct CorpusSize {
    /// Number of specs.
    pub specs: usize,
    /// Acceptance criteria per spec.
    pub criteria: usize,
    /// Numbered tasks per spec.
    pub tasks: usize,
    /// Subtasks per task.
    pub subtasks: usize,
    /// Scenarios per spec.
    pub scenarios: usize,
    /// Items in `specs/inbox.md`.
    pub inbox_items: usize,
    /// Whether each spec carries one markdown-lint violation and one broken
    /// link to a file outside the spec root. A primitive that returns
    /// findings grows with the defects in what it reads, so a corpus with
    /// none could not show that growth (§design-principles: build the
    /// subject that carries the condition).
    pub defects: bool,
    /// `[artifacts] read-size-bytes` in the corpus's config, written before
    /// the first commit so the history is the same either way; `None` leaves
    /// the setting out, so the default applies.
    pub read_size_bytes: Option<usize>,
}

/// The smaller corpus a measurement compares against.
pub const SMALL: CorpusSize = CorpusSize {
    specs: 10,
    criteria: 5,
    tasks: 5,
    subtasks: 3,
    scenarios: 2,
    inbox_items: 5,
    defects: true,
    read_size_bytes: None,
};

/// Ten times [`SMALL`] in every dimension but subtasks per task.
pub const LARGE: CorpusSize = CorpusSize {
    specs: 100,
    criteria: 50,
    tasks: 50,
    subtasks: 3,
    scenarios: 20,
    inbox_items: 50,
    defects: true,
    read_size_bytes: None,
};

/// The subject spec: the one a per-feature primitive is pointed at.
pub const SUBJECT: &str = "001-feature-001";

/// The directory name of spec `n` (1-based).
#[must_use]
pub fn slug(n: usize) -> String {
    format!("{n:03}-feature-{n:03}")
}

/// The status spec `n` ends at. Spec 1 is the in-progress subject; the rest
/// cycle through every status a corpus normally holds.
fn final_status(n: usize) -> &'static str {
    if n == 1 {
        return "in-progress";
    }
    match n % 4 {
        0 => "done",
        1 => "in-progress",
        2 => "planned",
        _ => "clarified",
    }
}

/// Write a generated corpus of `size` into `root`, which must be empty or
/// absent, and commit its history.
///
/// # Panics
///
/// On any filesystem or git failure: this is test support, and a corpus that
/// cannot be built is a broken test, never a skipped one.
pub fn generate(root: &Path, size: &CorpusSize) {
    fs::create_dir_all(root).unwrap();
    let repo = Repository::init(root).unwrap();
    let write = |rel: &str, body: &str| {
        let path = root.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    };

    let mut config = "[project]\nname = \"corpus\"\n".to_string();
    if let Some(bytes) = size.read_size_bytes {
        let _ = write!(config, "\n[artifacts]\nread-size-bytes = {bytes}\n");
    }
    write(".ductus/config.toml", &config);
    write("specs/inbox.md", &inbox(size.inbox_items));
    let rules = Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/rules/quality-cross.md");
    write(
        "specs/rules/quality-cross.md",
        &fs::read_to_string(rules).unwrap(),
    );
    let templates = Path::new(env!("CARGO_MANIFEST_DIR")).join("../framework/templates/spec");
    for entry in fs::read_dir(templates).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        write(
            &format!("specs/templates/{name}"),
            &fs::read_to_string(&path).unwrap(),
        );
    }
    for n in 1..=size.specs {
        write(
            &format!("specs/{}/spec.md", slug(n)),
            &spec(n, "planned", size),
        );
        write(&format!("specs/{}/plan.md", slug(n)), &plan(n, size));
        write(
            &format!("specs/{}/tasks.md", slug(n)),
            &tasks(n, size, Progress::NotStarted),
        );
        for k in 1..=size.scenarios {
            write(
                &format!("specs/{}/scenarios/scenario-{k:03}.md", slug(n)),
                &scenario(n, k),
            );
        }
    }
    for k in 1..=branch_scoped(size) {
        write(
            &format!("specs/{}/spec.md", branch_slug(k)),
            &format!(
                "---\nstatus: planned\ndependencies: []\nfolds-into: {}\n---\n\n# Staged change {k}\n\n\
                 A branch-scoped change that folds into its upstream spec.\n",
                slug((k * 10).min(size.specs))
            ),
        );
    }
    commit(&repo, "plan: every spec planned");

    for n in 1..=size.specs {
        if matches!(final_status(n), "in-progress" | "done") {
            write(
                &format!("specs/{}/spec.md", slug(n)),
                &spec(n, "in-progress", size),
            );
            let progress = if n == 1 || final_status(n) == "done" {
                Progress::Complete
            } else {
                Progress::Half
            };
            write(
                &format!("specs/{}/tasks.md", slug(n)),
                &tasks(n, size, progress),
            );
            for f in 0..source_files(size) {
                write(&source_path(n, f), &format!("// spec {n:03}, file {f}\n"));
            }
        }
    }
    commit(&repo, "feat: work on every started spec");

    for n in 1..=size.specs {
        if final_status(n) == "done" {
            write(
                &format!("specs/{}/spec.md", slug(n)),
                &spec(n, "done", size),
            );
        }
    }
    commit(&repo, "status: close the done specs");
}

/// Commit every file under the work tree.
fn commit(repo: &Repository, message: &str) {
    let mut index = repo.index().unwrap();
    index.add_all(["*"], IndexAddOption::DEFAULT, None).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = Signature::now("Corpus", "corpus@example.com").unwrap();
    let parent = repo.head().ok().map(|head| head.peel_to_commit().unwrap());
    let parents: Vec<&git2::Commit> = parent.iter().collect();
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
        .unwrap();
}

/// Branch-scoped specs (`corpus.{k}-staged-{k}`): one per ten specs, at least
/// one, each folding into a sequential spec, so the staging inventory scales
/// with the corpus too.
fn branch_scoped(size: &CorpusSize) -> usize {
    (size.specs / 10).max(1)
}

/// The directory name of branch-scoped spec `k` (1-based).
#[must_use]
pub fn branch_slug(k: usize) -> String {
    format!("corpus.{k}-staged-{k}")
}

/// Source files per started spec: one per five tasks, at least one.
fn source_files(size: &CorpusSize) -> usize {
    size.tasks / 5 + 1
}

fn source_path(n: usize, f: usize) -> String {
    format!("src/spec{n:03}/file{f:03}.rs")
}

fn inbox(items: usize) -> String {
    let mut out =
        String::from("# Inbox\n\nCapture queue for the todos you record by hand with `/log`.\n\n");
    for k in 1..=items {
        let _ = writeln!(out, "- [ ] Corpus inbox item {k:03}, logged by hand");
    }
    out
}

/// A spec at `status`. Each spec after the first links its predecessor, so
/// the dependency graph, the corpus links and the traversal all scale.
fn spec(n: usize, status: &str, size: &CorpusSize) -> String {
    let deps = if n == 1 {
        "[]".to_string()
    } else {
        format!("[{}]", slug(n - 1))
    };
    let checked = status == "done";
    let mut out = format!(
        "---\nstatus: {status}\ndependencies: {deps}\nnext-criterion: {}\n---\n\n# {n:03} — Feature {n:03}\n\n",
        size.criteria + 1
    );
    if n > 1 {
        let _ = writeln!(
            out,
            "Feature {n:03} builds on [feature {:03}](../{}/spec.md).\n",
            n - 1,
            slug(n - 1)
        );
    }
    out.push_str("## Behavior\n\n");
    for k in 1..=size.criteria {
        let _ = writeln!(
            out,
            "- **Behavior {k}.** The feature handles case {k} the way its criterion states, \
             and reports the case it could not handle rather than skipping it."
        );
    }
    out.push_str("\n## Acceptance Criteria\n\n");
    for k in 1..=size.criteria {
        let mark = if checked { "x" } else { " " };
        let _ = writeln!(
            out,
            "- [{mark}] AC{k}: Case {k} of feature {n:03} is handled, in `{}`.",
            source_path(n, k % source_files(size))
        );
    }
    if size.defects {
        // MD026 (a heading's trailing punctuation) and a relative link to a
        // file that does not exist: neither is a spec link, so neither moves
        // the dependency graph.
        out.push_str("\n## Notes:\n\nSee [the guide](../../docs/missing-guide.md).\n");
    }
    out.push_str("\n## Open Questions\n\n*None — all resolved.*\n\n## Resolved Questions\n\n");
    for k in 1..=size.criteria.div_ceil(5) {
        let _ = writeln!(
            out,
            "- **Does case {k} need its own path?** No: it shares the general one (owner, 2026-11-08)."
        );
    }
    out
}

fn plan(n: usize, size: &CorpusSize) -> String {
    let mut out = format!(
        "# {n:03} — Feature {n:03} Plan\n\nImplements [Feature {n:03}](spec.md).\n\n\
         ## Overview\n\nThe feature is built in one module per source file.\n\n\
         ## Technical Decisions\n\n### One module per file\n\nEach file holds one case group.\n\n\
         ## Affected Files\n\n| File | Action | Purpose |\n| --- | --- | --- |\n"
    );
    for f in 0..source_files(size) {
        let _ = writeln!(out, "| `{}` | Create | Case group {f} |", source_path(n, f));
    }
    out.push_str("\n## Trade-offs\n\n- **One file:** rejected, as it would not scale.\n");
    out
}

/// How far a spec's tasks have got.
#[derive(Clone, Copy)]
enum Progress {
    NotStarted,
    Half,
    Complete,
}

/// `tasks.md` at `progress`.
fn tasks(n: usize, size: &CorpusSize, progress: Progress) -> String {
    let mut out = format!(
        "# {n:03} — Feature {n:03} Tasks\n\nTasks derived from the [plan](plan.md). Complete in order.\n"
    );
    for t in 1..=size.tasks {
        let done = match progress {
            Progress::NotStarted => false,
            Progress::Half => t <= size.tasks / 2,
            Progress::Complete => true,
        };
        let mark = if done { "x" } else { " " };
        let _ = write!(out, "\n## {t}. Handle case group {t}\n\n");
        for s in 1..=size.subtasks {
            let _ = writeln!(
                out,
                "- [{mark}] Write step {s} of case group {t}, with its test"
            );
        }
        let _ = writeln!(
            out,
            "\n- **Done when**: case group {t}'s tests pass and its criterion is ticked."
        );
    }
    out
}

fn scenario(n: usize, k: usize) -> String {
    format!(
        "---\nsection: Behavior\n---\n\n# Feature {n:03} scenario {k:03}\n\n\
         ## Context\n\nCase {k} arrives with an input the general path does not expect.\n\n\
         ## Behavior\n\nThe feature reports the input and handles the rest of the case.\n"
    )
}
