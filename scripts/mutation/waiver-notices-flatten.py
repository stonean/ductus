"""Mutation list for 066's `process-waivers-notices-flatten-stored-text`:
each `process-waivers` notice prints a stored waiver's rule, file and reason
flattened to one line, and classification reads them as written
(`runtime/src/primitives/process_waivers.rs`).

Run with `python3 scripts/mutation/run.py scripts/mutation/waiver-notices-flatten.py
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
        'M1 the duplicate notice prints the stored rule and file raw',
        'src/primitives/process_waivers.rs',
        '"duplicate waiver: rule {shown_rule} at {at} — entry [{index}] ignored"',
        '"duplicate waiver: rule {rule} at {file} — entry [{index}] ignored"',
        'process_waivers::tests::notices_flatten_stored_text_and_classification_reads_it_as_written',
    ),
    (
        'M2 the retained notice prints the stored values raw',
        'src/primitives/process_waivers.rs',
        '"waiver retained: rule {rule} at {at} — dimension not evaluated this run ({reason})"',
        '"waiver retained: rule {} at {} — dimension not evaluated this run ({})", anchor.rule, anchor.file, anchor.reason',
        'process_waivers::tests::notices_flatten_stored_text_and_classification_reads_it_as_written',
    ),
    (
        'M3 the final expired notice prints the stored values raw',
        'src/primitives/process_waivers.rs',
        '            out.notices\n                .push(format!("waiver expired: rule {rule} at {at} ({reason})"));\n            out.expired.push(anchor);',
        '            out.notices.push(format!(\n                "waiver expired: rule {} at {} ({})",\n                anchor.rule, anchor.file, anchor.reason\n            ));\n            out.expired.push(anchor);',
        'process_waivers::tests::notices_flatten_stored_text_and_classification_reads_it_as_written',
    ),
    (
        'M4 classification carries the flattened rule instead of the stored one',
        'src/primitives/process_waivers.rs',
        '            let anchor = WaiverRef {\n                rule: rule.clone(),',
        '            let anchor = WaiverRef {\n                rule: shown_rule.clone(),',
        'process_waivers::tests::notices_flatten_stored_text_and_classification_reads_it_as_written',
    ),
    (
        'M5 the departure-expiry notice prints the stored values raw',
        'src/primitives/process_waivers.rs',
        '                && departure_gone(waiver, &rule, &located, spec_path.as_deref(), &mut entries)?\n            {\n                classified.notices.push(format!(\n                    "waiver expired: rule {shown_rule} at {at} ({shown_reason})"\n                ));',
        '                && departure_gone(waiver, &rule, &located, spec_path.as_deref(), &mut entries)?\n            {\n                classified.notices.push(format!(\n                    "waiver expired: rule {rule} at {file} ({reason})"\n                ));',
        'process_waivers::tests::the_departure_expiry_notice_flattens_stored_text',
    ),
    (
        'M6 the retained-unread notice prints the raw why text',
        'src/primitives/process_waivers.rs',
        '— {shown_why} ({reason})"',
        '— {why} ({reason})"',
        'process_waivers::tests::notices_flatten_stored_text_and_classification_reads_it_as_written',
    ),
]
