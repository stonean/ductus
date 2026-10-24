"""Mutation list for spec 022 task 17's bound on `apply-manifest`'s roots
(the owner's answer (a)): `runtime/src/primitives/apply_manifest.rs` and the
shared `resolve_root` in `runtime/src/primitives/mod.rs`.

Run with `python3 scripts/mutation/run.py scripts/mutation/apply-manifest-roots.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The roots were absolute-capable bases; task 17 bounds them by the working
directory. The behaviour has three cells, each with a test in
apply_manifest.rs that fails when its judgment is removed (see the comment
heading that file's root-bound tests):

  M1  the containment check in `resolve_root`: a root that resolves outside
      the working directory is refused. Killed by every refusal test
      (target absolute outside, target `..`, target through an outward link,
      source at a staging root itself, source `..` into staging).
  M2  `target-root` gets no staging exception. Killed by
      `target_root_in_a_staging_directory_is_refused`.
  M3  `source-root` gets the staging exception (the `{framework-root}`
      `/ductus` passes). Killed by
      `a_staging_source_root_outside_the_working_directory_is_accepted`.

No mutation is an expected survivor: every cell has a portable test.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 root containment check removed (every root resolves "inside")',
        'src/primitives/mod.rs',
        '    if canonical.starts_with(&canonical_repo) {\n        return Ok(resolved);',
        '    if canonical.starts_with(&canonical) {\n        return Ok(resolved);',
        'apply_manifest',
    ),
    (
        'M2 target-root gains the staging exception',
        'src/primitives/apply_manifest.rs',
        '&args.target_root, false)?;',
        '&args.target_root, true)?;',
        'apply_manifest',
    ),
    (
        'M3 source-root loses the staging exception',
        'src/primitives/apply_manifest.rs',
        '&args.source_root, true)?;',
        '&args.source_root, false)?;',
        'apply_manifest',
    ),
]
