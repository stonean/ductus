"""Mutation list for spec 067 task 45's walker half (AC8): an exec `/analyze`
step 4 dispatches `resolve-anchor` through `run_with_scenarios`
(`runtime/src/primitives/resolve_anchor.rs`), which scans the spec and each
`.md` under its `scenarios/`, tags each scenario reference with its `path`,
lists a scenario's unresolved anchor as `<path>: <anchor>`, adds the counts,
reports `scenarios-examined`, and counts a scenario, or a `scenarios/`, it
cannot read in `anchor-target-unreadable` (`dispatch_primitive` in
`runtime/src/interpreter/mod.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/exec-scenario-anchors-067.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the walker dispatches step 4 on spec.md alone',
        'src/interpreter/mod.rs',
        '            let result = primitives::resolve_anchor::run_with_scenarios(&args, repo)?;',
        '            let result = primitives::resolve_anchor::run(&args, repo)?;',
        'exec_step_4_',
    ),
    (
        'M2 a scenarios directory that cannot be listed reads as a spec with none',
        'src/primitives/resolve_anchor.rs',
        '        let Some(scenarios) = scenario_files(repo, &dir) else {',
        '        let Some(scenarios) = scenario_files(repo, &dir).or(Some(Vec::new())) else {',
        'exec_step_4_',
    ),
    (
        'M3 a scenario that cannot be read is not counted',
        'src/primitives/resolve_anchor.rs',
        '                scan.anchor_target_unreadable = scan.anchor_target_unreadable.saturating_add(1);\n                continue;',
        '                continue;',
        'exec_step_4_',
    ),
    (
        'M4 an absent scenarios directory reads as unreadable',
        'src/primitives/resolve_anchor.rs',
        '        return Some(Vec::new());',
        '        return None;',
        'exec_step_4_',
    ),
    (
        'M5 a scenario reference carries no path',
        'src/primitives/resolve_anchor.rs',
        '                    path: Some(path.to_string()),',
        '                    path: None,',
        'exec_step_4_',
    ),
    (
        'M6 a scenario unresolved anchor is listed without its path',
        'src/primitives/resolve_anchor.rs',
        '.map(|a| format!("{path}: {a}"))',
        '.map(|a| a)',
        'exec_step_4_',
    ),
    (
        'M7 a scenarios file that is not markdown is scanned',
        'src/primitives/resolve_anchor.rs',
        '        .filter(|entry| is_markdown(&entry.file_name().to_string_lossy()))\n        .map(|entry| format!("{dir}/{}", entry.file_name().to_string_lossy()))',
        '        .filter(|_| true)\n        .map(|entry| format!("{dir}/{}", entry.file_name().to_string_lossy()))',
        'exec_step_4_',
    ),
    (
        'M8 a scenario counts are not added to the spec',
        'src/primitives/resolve_anchor.rs',
        '        self.intra_document = self.intra_document.saturating_add(scan.intra_document);',
        '        let _ = scan.intra_document;',
        'exec_step_4_',
    ),
]
