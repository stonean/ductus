"""Mutation list for spec 057 task 27 (`BE-TXN-002`): `relocate-audit-records`
writes `review.md`, `analysis.md` and `spec.md` through the shared
compare-before-persist guard (`ReadSnapshot`, `runtime/src/primitives/mod.rs`),
and reports a relocation left half done by name. Each mutation removes the
guard from one write, or the half-done report from one refusal; the tests in
`relocate_audit_records.rs` kill them. The helper's own mutants are
`compare-and-set-041.py`.

Run with `python3 scripts/mutation/run.py scripts/mutation/compare-and-set-057.py
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
        'M1 an artifact is written over a file changed since its read',
        'src/primitives/relocate_audit_records.rs',
        '            match artifact.snapshot.write_if_unchanged(&artifact.rendered) {\n',
        '            match crate::primitives::write_atomic(&artifact.snapshot.repo.join(&artifact.snapshot.rel), &artifact.rendered) {\n',
        'relocate_audit_records::tests::an_artifact_changed_since_its_read_is_refused_and_nothing_is_written',
    ),
    (
        'M2 spec.md is rewritten over a file changed since its read',
        'src/primitives/relocate_audit_records.rs',
        '        match self.spec_snapshot.write_if_unchanged(&self.rewritten) {\n',
        '        match crate::primitives::write_atomic(&self.spec_snapshot.repo.join(&self.spec_snapshot.rel), &self.rewritten) {\n',
        'relocate_audit_records::tests::a_spec_changed_since_its_read_leaves_a_named_half_done_relocation',
    ),
    (
        'M3 a refused spec.md after the artifacts were written reads as a plain refusal',
        'src/primitives/relocate_audit_records.rs',
        '            Err(PrimitiveError::ChangedSinceRead { path }) => {\n                return Err(PrimitiveError::RelocationIncomplete {\n                    written,\n                    refused: path,\n                });\n            }\n            Err(err) => return Err(err),\n        }\n        Ok(RelocateAuditRecordsResult {',
        '            Err(err) => return Err(err),\n        }\n        Ok(RelocateAuditRecordsResult {',
        'relocate_audit_records::tests::a_spec_changed_since_its_read_leaves_a_named_half_done_relocation',
    ),
    (
        'M4 a refused later artifact after an earlier write reads as a plain refusal',
        'src/primitives/relocate_audit_records.rs',
        '                Err(PrimitiveError::ChangedSinceRead { path }) if !written.is_empty() => {',
        '                Err(PrimitiveError::ChangedSinceRead { path }) if false && !written.is_empty() => {',
        'relocate_audit_records::tests::a_later_artifact_changed_since_its_read_leaves_a_named_half_done_relocation',
    ),
    (
        'M5 the half-done error forgets which files were written',
        'src/primitives/relocate_audit_records.rs',
        '                    return Err(PrimitiveError::RelocationIncomplete {\n                        written,\n                        refused: path,\n                    });',
        '                    return Err(PrimitiveError::RelocationIncomplete {\n                        written: Vec::new(),\n                        refused: path,\n                    });',
        'relocate_audit_records::tests::a_later_artifact_changed_since_its_read_leaves_a_named_half_done_relocation',
    ),
    (
        'M6 spec.md is written before the artifacts, so a refused artifact drops its block',
        'src/primitives/relocate_audit_records.rs',
        '        let mut written: Vec<PathBuf> = Vec::new();\n        for artifact in &self.artifacts {',
        '        let mut written: Vec<PathBuf> = Vec::new();\n        self.spec_snapshot.write_if_unchanged(&self.rewritten)?;\n        for artifact in &self.artifacts {',
        'relocate_audit_records::tests::an_artifact_changed_since_its_read_is_refused_and_nothing_is_written',
    ),
]
