"""Mutation list for spec 022 task 43 (the review round of 2026-11-17):
the agent-reach guards judge every case (O30, O31, O32), the canonical allow set
retires four entries (O100, O106, O112, O117), and the containment audit's
tests for `enforce-manifest` (BE-INPUT-004).

Run with `python3 scripts/mutation/run.py scripts/mutation/review-022-task-43.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

  M6  check_no_protected ends at the first config-directory refusal, as it did.
      Killed by host_config's and merge_permissions' O30 tests.
  M7  check_no_protected refuses a non-row under a config directory. Killed by
      merge_permissions::tests::a_config_dir_settings_file_stays_allowed.
  M8  validate_staging_outside admits a dest that contains the working
      directory. Killed by extract_archive::tests::a_dest_containing_the_working_directory_is_refused.
  M10 step 4's four new entries leave RETIRED_ALLOW. Killed by merge_permissions'
      step 4, count and run-or-write tests.
  M11 git push returns to the canonical set. Killed by merge_permissions'
      ceiling and count tests.
  M14 enforce-manifest's recursive walk follows links.
  M15 enforce-manifest's flat walk reads a link's target's type. M14 and M15 are
      killed by enforce_manifest::tests::a_link_inside_the_directory_is_not_followed_by_either_walk.

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M6 check_no_protected ends at the first config-directory refusal',
        'src/primitives/host_config.rs',
        '    judge_destination(repo, dest, shown, Scope::ProtectedOnly).map(|_| ())',
        '    match check_destination(repo, dest, shown) {\n        Ok(_) => Ok(()),\n        Err(PrimitiveError::InvalidPath { reason, .. })\n            if reason.contains("configuration directory `") =>\n        {\n            Ok(())\n        }\n        Err(other) => Err(other),\n    }',
        'host_config::tests::a_config_directory_verdict_the_call_allows_does_not_end_the_judgment merge_permissions::tests::a_link_under_a_config_directory_into_a_protected_one_is_refused',
    ),
    (
        'M7 check_no_protected refuses a non-row under a config directory',
        'src/primitives/host_config.rs',
        '        None if scope == Scope::ProtectedOnly => Ok(None),\n',
        '',
        'merge_permissions::tests::a_config_dir_settings_file_stays_allowed host_config::tests::a_config_directory_verdict_the_call_allows_does_not_end_the_judgment',
    ),
    (
        'M8 validate_staging_outside admits a dest that contains the working directory',
        'src/primitives/mod.rs',
        '    if canonical.starts_with(&canonical_repo) || canonical_repo.starts_with(&canonical) {',
        '    if canonical.starts_with(&canonical_repo) {',
        'extract_archive::tests::a_dest_containing_the_working_directory_is_refused',
    ),
    (
        'M10 the four entries task 43 retires leave RETIRED_ALLOW',
        'src/primitives/merge_permissions.rs',
        '    "Bash(curl *)",\n    "Bash(git commit *)",\n    "Bash(git push *)",\n    "Bash(gh api *)",\n    "Bash(git config --unset core.hooksPath)",\n];',
        '    "Bash(curl *)",\n];',
        'merge_permissions::tests::the_sets_hold_the_counts_task_43_left merge_permissions::tests::the_retired_set_is_configure_claudes_step_4 merge_permissions::tests::the_run_or_write_entries_are_retired_not_granted',
    ),
    (
        'M11 git push returns to the canonical set',
        'src/primitives/merge_permissions.rs',
        '    "Bash(git config --get core.hooksPath)",\n];',
        '    "Bash(git config --get core.hooksPath)",\n    "Bash(git push *)",\n];',
        'merge_permissions::tests::the_sets_hold_the_counts_task_43_left merge_permissions::tests::the_ceiling_is_configure_claudes_canonical_allow_set merge_permissions::tests::the_run_or_write_entries_are_retired_not_granted',
    ),
    (
        'M14 enforce-manifest recursive walk follows links',
        'src/primitives/enforce_manifest.rs',
        '.min_depth(1).follow_links(false)',
        '.min_depth(1).follow_links(true)',
        'enforce_manifest::tests::a_link_inside_the_directory_is_not_followed_by_either_walk',
    ),
    (
        'M15 enforce-manifest flat walk reads a link\'s target type',
        'src/primitives/enforce_manifest.rs',
        '            if ft.is_file() {',
        '            if ft.is_file() || path.is_file() {',
        'enforce_manifest::tests::a_link_inside_the_directory_is_not_followed_by_either_walk',
    ),
]
