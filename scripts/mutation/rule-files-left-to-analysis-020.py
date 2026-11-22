"""Mutation list for spec 020's AC17: a review leaves out each rule file whose
header names `/{project}:analyze` and not `/{project}:review`
(`leave_to_analysis` and `header_leaves_to_analysis` in
`runtime/src/primitives/discover_rule_files.rs`), after the
disabled-rule-files filter, keeping a file it cannot read, and an analysis
(`all`) loads every file.

Run with `python3 scripts/mutation/run.py scripts/mutation/rule-files-left-to-analysis-020.py
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
        'M1 a review loads a file whose header gives it to analysis',
        'src/primitives/discover_rule_files.rs',
        '            leave_to_analysis(&dir, &mut selected, &mut notices);',
        '            let _ = &dir;',
        'discover_rule_files::',
    ),
    (
        'M2 an analysis leaves a file out as a review does',
        'src/primitives/discover_rule_files.rs',
        '        if !args.all {\n            leave_to_analysis(&dir, &mut selected, &mut notices);',
        '        if true {\n            leave_to_analysis(&dir, &mut selected, &mut notices);',
        'discover_rule_files::',
    ),
    (
        'M3 the header test runs before the disabled-rule-files filter',
        'src/primitives/discover_rule_files.rs',
        '        apply_disabled_filter(&disabled, &all, &mut selected, &mut notices, config_name);\n        if !args.all {\n            leave_to_analysis(&dir, &mut selected, &mut notices);\n        }',
        '        if !args.all {\n            leave_to_analysis(&dir, &mut selected, &mut notices);\n        }\n        apply_disabled_filter(&disabled, &all, &mut selected, &mut notices, config_name);',
        'discover_rule_files::',
    ),
    (
        'M4 a file that cannot be read is left out',
        'src/primitives/discover_rule_files.rs',
        'let left = read_text(&dir.join(name)).is_ok_and(|text| header_leaves_to_analysis(&text));',
        'let left = read_text(&dir.join(name)).map_or(true, |text| header_leaves_to_analysis(&text));',
        'discover_rule_files::',
    ),
    (
        'M5 a header naming both commands is left to analysis',
        'src/primitives/discover_rule_files.rs',
        '    analyze && !review\n}',
        '    analyze || review && false\n}',
        'discover_rule_files::',
    ),
    (
        'M6 a command named in the body below the header counts',
        'src/primitives/discover_rule_files.rs',
        '        .take_while(|line| !line.starts_with("##"))\n        .collect();\n    let (mut review, mut analyze) = (false, false);',
        '        .take_while(|_| true)\n        .collect();\n    let (mut review, mut analyze) = (false, false);',
        'discover_rule_files::',
    ),
]
