"""Mutation list for spec 022 task 17 question 2 (the owner's answer (a)): the
archive primitives' write destination must lie outside the working directory,
`validate_staging_outside` in `runtime/src/primitives/mod.rs`, used by
`extract-archive` (`dest`) and `fetch-archive` (`archive`, its output).

Run with `python3 scripts/mutation/run.py scripts/mutation/archive-staging-outside.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

One judgment, one mutation:

  M1  the outside-the-working-directory check in `validate_staging_outside`:
      a staging path that resolves inside the working directory is refused.
      Removing it (the condition can never hold) makes both primitives accept
      a dest inside a project that sits under a staging root. Killed by the
      two refusal tests, `a_dest_inside_the_working_directory_is_refused`
      (extract-archive) and `an_output_inside_the_working_directory_is_refused`
      (fetch-archive). The acceptance test
      (`a_staging_dest_outside_the_working_directory_is_accepted`, the
      `/ductus` shape) is the regression guard and needs no mutation of its
      own: M1 cannot make it fail.

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 outside-working-directory check removed (a staging dest inside the project passes)',
        'src/primitives/mod.rs',
        '    if canonical.starts_with(&canonical_repo) {\n        return Err(PrimitiveError::InvalidPath {',
        '    if false {\n        return Err(PrimitiveError::InvalidPath {',
        'a_dest_inside_the_working_directory_is_refused an_output_inside_the_working_directory_is_refused',
    ),
]
