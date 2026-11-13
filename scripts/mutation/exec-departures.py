"""Mutation list for `ductus exec` analyze counting a departure entry's
findings (spec 073, task 10): `AnalyzeTally` counts `check-departures`' findings
in the tier each carries, in its `tier` field, and its `unexamined` entries by
reason. Each mutation removes one of those, so a kill shows its own test holds
it.

Run with `python3 scripts/mutation/run.py scripts/mutation/exec-departures.py
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
        'M1 a Blocking departure finding is not counted',
        'src/interpreter/analyze_tally.rs',
        'Some(AnalyzeSeverity::Blocking) => self.blocking += 1,\n                        Some(AnalyzeSeverity::Advisory) => self.advisory += 1,\n                        _ => {}\n                    }\n                }\n                for skipped in items("unexamined") {',
        'Some(AnalyzeSeverity::Blocking) => {}\n                        Some(AnalyzeSeverity::Advisory) => self.advisory += 1,\n                        _ => {}\n                    }\n                }\n                for skipped in items("unexamined") {',
        'an_exec_walk_counts_a_malformed_departure_entry_as_blocking each_step_counts_in_the_tier_it_assigns',
    ),
    (
        'M2 a departure finding is read from severity, not tier',
        'src/interpreter/analyze_tally.rs',
        '.get("tier")\n                        .and_then(Value::as_str)\n                        .and_then(|tier| tier.parse().ok())\n                    {\n                        Some(AnalyzeSeverity::Blocking) => self.blocking += 1,',
        '.get("severity")\n                        .and_then(Value::as_str)\n                        .and_then(|tier| tier.parse().ok())\n                    {\n                        Some(AnalyzeSeverity::Blocking) => self.blocking += 1,',
        'an_exec_walk_counts_a_malformed_departure_entry_as_blocking each_step_counts_in_the_tier_it_assigns',
    ),
    (
        'M3 an unjudged departure rule or citation is not recorded unexamined',
        'src/interpreter/analyze_tally.rs',
        'for skipped in items("unexamined") {\n                    if let Some(reason) = skipped.get("reason").and_then(Value::as_str) {\n                        self.record_unexamined(reason, 1);\n                    }\n                }\n            }\n            // Step 6',
        'for skipped in items("unexamined") {\n                    let _ = skipped;\n                }\n            }\n            // Step 6',
        'each_step_counts_in_the_tier_it_assigns',
    ),
    (
        'M4 write-analysis is bound the unexamined list check-departures reports',
        'src/interpreter/mod.rs',
        'bindings.remove("findings");\n        bindings.remove("unexamined");',
        'bindings.remove("findings");',
        'an_exec_walk_counts_a_malformed_departure_entry_as_blocking',
    ),
]
