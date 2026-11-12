"""Mutation list for spec 067 task 29: `check-artifacts` reads no file in a
feature directory through a link that leaves the project. Each mutation drops
one guard at a read the family-wide test could not see: `plan.md` in
`check_plan_consistency`, the `scanned_artifacts` and `body_artifacts` filters
(the fixed names and the scenarios), and the `scenario_slugs` filter. The test
that kills them, `a_linked_body_is_read_by_no_family`, links a body every
reading family would flag and runs every family.

Run with `python3 scripts/mutation/run.py scripts/mutation/outward-link-reads-067.py
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
        'M1 check_plan_consistency reads a plan.md linked outside the project',
        'src/primitives/check_artifacts.rs',
        '    if outward.covers(&plan) {\n        return;\n    }\n    let Ok(text) = read_text(&plan)',
        '    let Ok(text) = read_text(&plan)',
        'check_artifacts::tests::a_linked_body_is_read_by_no_family',
    ),
    (
        'M2 scanned_artifacts lists a fixed-name file linked outside the project',
        'src/primitives/check_artifacts.rs',
        '    let mut out: Vec<PathBuf> = ["spec.md", "plan.md", "tasks.md"]\n        .iter()\n        .map(|name| feature_dir.join(name))\n        .filter(|path| path.is_file() && !outward.covers(path))',
        '    let mut out: Vec<PathBuf> = ["spec.md", "plan.md", "tasks.md"]\n        .iter()\n        .map(|name| feature_dir.join(name))\n        .filter(|path| path.is_file())',
        'check_artifacts::tests::a_linked_body_is_read_by_no_family',
    ),
    (
        'M3 scanned_artifacts lists a scenario linked outside the project',
        'src/primitives/check_artifacts.rs',
        '            .map(|name| scenarios_dir.join(name))\n            .filter(|path| !outward.covers(path)),',
        '            .map(|name| scenarios_dir.join(name)),',
        'check_artifacts::tests::a_linked_body_is_read_by_no_family',
    ),
    (
        'M4 body_artifacts lists a fixed-name file linked outside the project',
        'src/primitives/check_artifacts.rs',
        '    let mut out: Vec<PathBuf> = ["spec.md", "plan.md", "data-model.md"]\n        .iter()\n        .map(|name| feature_dir.join(name))\n        .filter(|path| path.is_file() && !outward.covers(path))',
        '    let mut out: Vec<PathBuf> = ["spec.md", "plan.md", "data-model.md"]\n        .iter()\n        .map(|name| feature_dir.join(name))\n        .filter(|path| path.is_file())',
        'check_artifacts::tests::a_linked_body_is_read_by_no_family',
    ),
    (
        'M5 body_artifacts lists a scenario linked outside the project',
        'src/primitives/check_artifacts.rs',
        '            .map(|name| scenarios.join(name))\n            .filter(|path| !outward.covers(path)),',
        '            .map(|name| scenarios.join(name)),',
        'check_artifacts::tests::a_linked_body_is_read_by_no_family',
    ),
    (
        'M6 scenario_slugs lists a scenario linked outside the project',
        'src/primitives/check_artifacts.rs',
        '        .iter()\n        .filter(|name| !outward.covers(&scenarios_dir.join(name)))\n        .filter_map(|name| {',
        '        .iter()\n        .filter_map(|name| {',
        'check_artifacts::tests::a_linked_body_is_read_by_no_family',
    ),
]
