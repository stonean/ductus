"""Mutation list for spec 078's AC16: every fix-round brief states the
reachable-trigger bar, so a re-read raises a finding only on a case whose
trigger is shown possible in the project (`SEVERITY_BAR` in
`runtime/src/primitives/brief_re_read.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/reachable-trigger-bar-078.py
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
        'M1 the brief drops the reachable-trigger clause from its bar',
        'src/primitives/brief_re_read.rs',
        'Raise a finding of any severity only on a case whose trigger is shown possible in this project, from its code, configuration, history or deployment shape. A case shown only by building it, an input or a layout the project cannot produce, is a false positive: leave it out of the result file.\n\n',
        '',
        'brief_re_read::',
    ),
]
