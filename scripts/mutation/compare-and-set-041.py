"""Mutation list for spec 041 task 27 (`BE-TXN-002`): the shared compare-before-
persist guard, `ReadSnapshot` in `runtime/src/primitives/mod.rs`, and the three
prune applies that write through it. Each mutation removes one half of the
guard, or makes an apply write without it; the helper's tests and one test per
prune primitive kill them.

Run with `python3 scripts/mutation/run.py scripts/mutation/compare-and-set-041.py
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
        'M1 a file changed since its read is written over',
        'src/primitives/mod.rs',
        '        if read_observed(&path)? != self.observed {\n            return Err(PrimitiveError::ChangedSinceRead {',
        '        if false && read_observed(&path)? != self.observed {\n            return Err(PrimitiveError::ChangedSinceRead {',
        'primitives::tests::a_file_changed_since_its_read_is_refused_and_the_other_writers_bytes_survive',
    ),
    (
        'M2 a file absent at its read is written over when it now exists',
        'src/primitives/mod.rs',
        '        if read_observed(&path)? != self.observed {\n            return Err(PrimitiveError::ChangedSinceRead {',
        '        if self.observed.is_some() && read_observed(&path)? != self.observed {\n            return Err(PrimitiveError::ChangedSinceRead {',
        'primitives::tests::a_file_absent_at_its_read_must_still_be_absent_at_its_write',
    ),
    (
        'M3 a file deleted since its read is recreated',
        'src/primitives/mod.rs',
        '        if read_observed(&path)? != self.observed {\n            return Err(PrimitiveError::ChangedSinceRead {',
        '        if read_observed(&path)?.is_some_and(|now| Some(now) != self.observed) {\n            return Err(PrimitiveError::ChangedSinceRead {',
        'primitives::tests::a_file_deleted_since_its_read_is_refused',
    ),
    (
        'M4 the compare and the rename hold no lock',
        'src/primitives/mod.rs',
        '        let _lock = crate::session::lock_compare_and_set(&self.repo, timeout)?;\n',
        '        let _lock = timeout;\n',
        'primitives::tests::two_runtime_writers_cannot_both_pass_the_compare',
    ),
    (
        'M5 the re-read skips the containment check',
        'src/primitives/mod.rs',
        '        hook(Stage::BeforeReread);\n        validate_contained(&self.repo, &self.rel)?;\n',
        '        hook(Stage::BeforeReread);\n',
        'primitives::tests::the_write_is_refused_through_a_link_leaving_the_project',
    ),
    (
        'M6 a rewrite drops the destination mode',
        'src/primitives/mod.rs',
        '        hook(Stage::BeforeRename);\n        write_atomic_bytes(&path, content)\n',
        '        hook(Stage::BeforeRename);\n        std::fs::remove_file(&path).ok();\n        write_atomic_bytes(&path, content)\n',
        'primitives::tests::an_unchanged_file_is_replaced_and_keeps_its_mode',
    ),
    (
        'M7 prune-tasks writes its reduction over a changed file',
        'src/primitives/prune_tasks.rs',
        '            self.snapshot.write_if_unchanged(&self.new_content)?;\n',
        '            crate::primitives::write_atomic(&self.snapshot.repo.join(&self.snapshot.rel), &self.new_content)?;\n',
        'prune_tasks::tests::an_apply_refuses_a_file_changed_after_it_was_read',
    ),
    (
        'M8 a prune-tasks walk stops at a refused spec instead of listing it',
        'src/primitives/prune_tasks.rs',
        '            Err(PrimitiveError::ChangedSinceRead { .. }) => skipped.push(SkippedFeature {',
        '            Err(PrimitiveError::ChangedSinceRead { .. }) if false => skipped.push(SkippedFeature {',
        'prune_tasks::tests::a_walk_refuses_a_changed_feature_and_writes_the_others',
    ),
    (
        'M9 prune-plan writes its removal over a changed plan',
        'src/primitives/prune_plan.rs',
        '        snapshot.write_if_unchanged(&after)?;\n',
        '        crate::primitives::write_atomic(&snapshot.repo.join(&snapshot.rel), &after)?;\n',
        'prune_plan::tests::an_apply_refuses_a_plan_changed_after_it_was_read',
    ),
    (
        'M10 prune-project writes a project file over a changed one',
        'src/primitives/prune_project.rs',
        '        match snapshot.write_if_unchanged(rewrite(content, segments, &edits)) {\n',
        '        match crate::primitives::write_atomic(&snapshot.repo.join(&snapshot.rel), &rewrite(content, segments, &edits)) {\n',
        'prune_project::tests::a_file_changed_after_the_apply_read_it_is_refused_and_the_rest_is_written',
    ),
    (
        'M11 prune-project stops writing at the first refused file',
        'src/primitives/prune_project.rs',
        '                outcome.refused.push((*rel).to_string());\n            }\n',
        '                outcome.refused.push((*rel).to_string());\n                break;\n            }\n',
        'prune_project::tests::a_file_changed_after_the_apply_read_it_is_refused_and_the_rest_is_written',
    ),
    (
        'M12 prune-project reports an apply that refused a file as applied',
        'src/primitives/prune_project.rs',
        '    outcome.applied = outcome.refused.is_empty();\n',
        '    outcome.applied = true;\n',
        'prune_project::tests::a_file_changed_after_the_apply_read_it_is_refused_and_the_rest_is_written',
    ),
    (
        'M13 prune-project replaces a home another writer created',
        'src/primitives/prune_project.rs',
        '    if !write_new_atomic(path, body)? {\n',
        '    if !crate::primitives::write_atomic(path, body).map(|()| true)? {\n',
        'prune_project::tests::a_home_another_writer_created_is_kept_and_never_replaced',
    ),
    (
        'M14 the skip reason a host matches on is renamed on the wire',
        'src/schema/primitives.rs',
        '    /// (`BE-TXN-002`, `BE-TXN-004`).\n    ChangedSinceRead,\n',
        '    /// (`BE-TXN-002`, `BE-TXN-004`).\n    #[serde(rename = "changed")]\n    ChangedSinceRead,\n',
        'schema::primitives::tests::prune_plan_round_trip',
    ),
]
