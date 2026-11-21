"""Mutation list for spec 067 task 44 (AC6): `write-analysis`
(`refuse_design_edit_deferrals` in `runtime/src/primitives/write_analysis.rs`)
refuses a live hard-fail or blocking finding at a design subject of the spec
recorded deferred to the spec's own `tasks.md`, and one recorded
undispositioned at a design subject that a finding recorded `live: false`
also cites (a fix of the run edited it), and accepts every other record.

Run with `python3 scripts/mutation/run.py scripts/mutation/analysis-design-fix-067.py
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
        'M1 neither refusal runs',
        'src/primitives/write_analysis.rs',
        '    refuse_design_edit_deferrals(args, &root)?;\n',
        '',
        'a_gating_design_finding_deferred a_gating_finding_left_undispositioned',
    ),
    (
        'M2 a deferral to another spec is refused as one to the own tasks.md',
        'src/primitives/write_analysis.rs',
        '.is_some_and(|target| target.strip_prefix("./").unwrap_or(target) == own_tasks)',
        '.is_some()',
        'a_design_finding_deferred_elsewhere',
    ),
    (
        'M3 a deferral to the own tasks.md is accepted',
        'src/primitives/write_analysis.rs',
        '.is_some_and(|target| target.strip_prefix("./").unwrap_or(target) == own_tasks)',
        '.is_some_and(|target| target.strip_prefix("./").unwrap_or(target) == "")',
        'a_gating_design_finding_deferred',
    ),
    (
        'M4 an advisory finding is refused as a gating one',
        'src/primitives/write_analysis.rs',
        'AnalysisTier::HardFail | AnalysisTier::Blocking\n        ) {\n            continue;',
        'AnalysisTier::HardFail | AnalysisTier::Blocking | AnalysisTier::Advisory\n        ) {\n            continue;',
        'a_design_finding_deferred_elsewhere a_gating_finding_is_recorded_where',
    ),
    (
        'M5 a finding at a file that is no design subject is refused',
        'src/primitives/write_analysis.rs',
        '        .filter(|inside| crate::primitives::analyze_subjects::is_design_subject(inside))',
        '        .filter(|_| true)',
        'a_design_finding_deferred_elsewhere a_gating_finding_is_recorded_where',
    ),
    (
        'M6 a repo-relative path names no design subject',
        'src/primitives/write_analysis.rs',
        '    if let Some(inside) = path.strip_prefix(own.as_str()) {\n        return Some(inside);\n    }\n',
        '',
        'a_gating_design_finding_deferred a_gating_finding_left_undispositioned',
    ),
    (
        'M7 a path relative to the feature directory names no design subject',
        'src/primitives/write_analysis.rs',
        '.then_some(path)',
        '.then_some("")',
        'a_gating_design_finding_deferred a_gating_finding_left_undispositioned',
    ),
    (
        'M8 a gating finding left undispositioned at an edited subject is accepted',
        'src/primitives/write_analysis.rs',
        'DispositionOutcome::Undispositioned if edited.contains(subject) => {',
        'DispositionOutcome::Undispositioned if false && edited.contains(subject) => {',
        'a_gating_finding_left_undispositioned',
    ),
    (
        'M9 any fix of the run counts as an edit of every subject',
        'src/primitives/write_analysis.rs',
        'DispositionOutcome::Undispositioned if edited.contains(subject) => {',
        'DispositionOutcome::Undispositioned if !edited.is_empty() => {',
        'a_gating_finding_is_recorded_where',
    ),
    (
        'M10 a finding still firing counts as one a fix removed',
        'src/primitives/write_analysis.rs',
        '        .filter(|finding| !finding.live)\n        .filter_map(|finding| design_subject(finding, root, &args.feature))',
        '        .filter(|finding| finding.live)\n        .filter_map(|finding| design_subject(finding, root, &args.feature))',
        'a_gating_finding_is_recorded_where',
    ),
]
