"""Mutation list for `ductus exec` analyze counting a departure entry's
findings (spec 073, task 10): `AnalyzeTally` counts `check-departures`' findings
in the tier each carries, in its `tier` field, and counts none of its
`unexamined` entries, which step 4's and step 5's records already hold (task
11). Each mutation breaks one of those, so a kill shows its own test holds it.

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
        'Some(AnalyzeSeverity::Blocking) => self.blocking += 1,\n                        Some(AnalyzeSeverity::Advisory) => self.advisory += 1,\n                        _ => {}\n                    }\n                }\n            }\n            // Step 6',
        'Some(AnalyzeSeverity::Blocking) => {}\n                        Some(AnalyzeSeverity::Advisory) => self.advisory += 1,\n                        _ => {}\n                    }\n                }\n            }\n            // Step 6',
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
        'M3 a departure result\'s unexamined entries are recorded again',
        'src/interpreter/analyze_tally.rs',
        '                        _ => {}\n                    }\n                }\n            }\n            // Step 6: drift',
        '                        _ => {}\n                    }\n                }\n                for skipped in items("unexamined") {\n                    if let Some(reason) = skipped.get("reason").and_then(Value::as_str) {\n                        self.record_unexamined(reason, 1);\n                    }\n                }\n            }\n            // Step 6: drift',
        'each_step_counts_in_the_tier_it_assigns',
    ),
    (
        'M4 write-analysis is bound the unexamined list check-departures reports',
        'src/interpreter/mod.rs',
        'bindings.remove("findings");\n        bindings.remove("unexamined");',
        'bindings.remove("findings");',
        'an_exec_walk_counts_a_malformed_departure_entry_as_blocking',
    ),
    (
        'M5 the walker does not restore check-departures\' file form before the tally reads it',
        'src/interpreter/payload.rs',
        '    if map.get("form").and_then(Value::as_str) != Some("file") {\n        return Ok(Vec::new());\n    }\n    let file = map',
        '    if map.get("form").and_then(Value::as_str) != Some("file") || primitive == "check-departures" {\n        return Ok(Vec::new());\n    }\n    let file = map',
        'an_exec_walk_counts_a_departure_finding_held_in_the_file_form',
    ),
]
