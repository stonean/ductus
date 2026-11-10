//! The pipeline view held to `[artifacts] read-size-bytes` on disk, through
//! both surfaces that serialize it (spec 080, AC24): `render-status`'s result,
//! which the model path reads, and `ductus hook status`'s block response,
//! which the hook path prints. Each is driven through the binary on a
//! generated fixture corpus past the bound, and measured as the bytes it
//! prints, since those are what reach a host.

#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;

use common::corpus::{self, CorpusSize};
use common::ductus_command;
use serde_json::Value;

/// 120 small specs, a quarter of them `done`: the view grows with the count,
/// not with any one spec.
const CORPUS: CorpusSize = CorpusSize {
    specs: 120,
    criteria: 1,
    tasks: 1,
    subtasks: 1,
    scenarios: 0,
    inbox_items: 1,
    defects: false,
    read_size_bytes: None,
};

fn set_read_size(root: &Path, bytes: usize) {
    fs::write(
        root.join(".ductus/config.toml"),
        format!("[host]\nproject = \"corpus\"\n\n[artifacts]\nread-size-bytes = {bytes}\n"),
    )
    .unwrap();
}

/// Run `ductus {args}` at `root`, with `input` on stdin, and return what it
/// printed on stdout, without the closing newline.
fn printed(root: &Path, args: &[&str], input: &str) -> String {
    let mut child = ductus_command(env!("CARGO_BIN_EXE_ductus"))
        .args(args)
        .current_dir(root)
        .env("CLAUDE_PROJECT_DIR", root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("binary runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "ductus {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut text = String::from_utf8(output.stdout).unwrap();
    if text.ends_with('\n') {
        text.pop();
    }
    text
}

/// Past the rows alone, and past the next-action names too, each surface
/// prints no more than the read size. `render-status`'s list file holds the
/// whole view, and each line of its view standing for a cut names that file.
/// The hook path writes no file, so its view names none.
#[test]
fn both_surfaces_hold_the_whole_result_to_the_read_size_on_disk() {
    let tmp = tempfile::tempdir().unwrap();
    corpus::generate(tmp.path(), &CORPUS);
    set_read_size(tmp.path(), 100_000_000);
    let text = printed(tmp.path(), &["render-status"], "");
    let whole: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(whole["form"], "inline", "{text}");
    let whole_view = whole["rendered-markdown"].as_str().unwrap().to_string();
    for bound in [text.len() / 2, 1_600] {
        set_read_size(tmp.path(), bound);
        let text = printed(tmp.path(), &["render-status"], "");
        assert!(
            text.len() <= bound,
            "render-status printed {} bytes past {bound}:\n{text}",
            text.len()
        );
        let result: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(result["form"], "file", "{text}");
        let file = result["lists-file"].as_str().unwrap();
        assert!(
            file.starts_with(".ductus/results/render-status/all-specs/"),
            "{file}"
        );
        let held: Value =
            serde_json::from_str(&fs::read_to_string(tmp.path().join(file)).unwrap()).unwrap();
        let lines: Vec<&str> = held["view"]
            .as_array()
            .unwrap()
            .iter()
            .map(|line| line.as_str().unwrap())
            .collect();
        assert_eq!(
            lines.join("\n"),
            whole_view,
            "the list file holds the whole view"
        );
        let view = result["rendered-markdown"].as_str().unwrap();
        assert!(
            view.contains(&format!(
                "not shown — the view is over the read size; the whole view is in {file}"
            )),
            "{view}"
        );

        let response = printed(tmp.path(), &["hook", "status"], "{}");
        assert!(
            response.len() <= bound,
            "hook status printed {} bytes past {bound}:\n{response}",
            response.len()
        );
        let parsed: Value = serde_json::from_str(&response).unwrap();
        let reason = parsed["reason"].as_str().unwrap();
        assert!(
            reason.contains("not shown — the view is over the read size"),
            "{reason}"
        );
        assert!(
            !reason.contains(".ductus/results/"),
            "the hook path writes no file: {reason}"
        );
    }
}
