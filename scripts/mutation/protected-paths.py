"""Mutation list for spec 022 task 20 (the owner's answer (a)): the
host-configuration destination check extended to Claude Code's protected set
(`runtime/src/primitives/host_config.rs`) and the six writers routed through
it. `.githooks` is deliberately out of scope (the host does not protect it).

Run with `python3 scripts/mutation/run.py scripts/mutation/protected-paths.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused; when the
code a mutation anchors on changes, update the anchor in the same change
(`run.py --check`).

The judgments and the tests that hold them:

  M1  judge_names refuses a protected directory (the full check, used by
      apply-manifest / enforce-manifest / merge-managed-block and, through
      check_no_destination, append-task / create-scenario). Killed by
      host_config::a_protected_destination_is_refused.
  M2  judge_names refuses a protected file. Killed by the same test.
  M3  merge-permissions routes its path through check_no_protected. Killed by
      merge_permissions::a_protected_path_is_refused.
  M4  append-task routes its feature path through check_no_destination. Killed
      by append_task::a_feature_path_under_a_config_or_protected_directory_is_refused.
  M5  create-scenario routes its feature path through check_no_destination.
      Killed by create_scenario::refuses_a_feature_path_under_a_config_or_protected_directory.
  M6  check_no_protected refuses a protected directory (merge-permissions'
      own path, a judgment distinct from judge_names'). Killed by the
      merge_permissions test (`.git/config`).
  M7  check_no_protected refuses a protected file. Killed by that test
      (`.npmrc`).
  M8  check_no_protected refuses an MCP/settings file. Killed by that test
      (`.mcp.json`).

No mutation is an expected survivor.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 judge_names protected-directory refusal removed',
        'src/primitives/host_config.rs',
        '    if let Some(seq) = protected_dir_in(names) {\n        return Err(refuse_protected_dir(shown, seq));\n    }',
        '    if let Some(seq) = protected_dir_in(names) {\n        let _ = seq;\n    }',
        'host_config',
    ),
    (
        'M2 judge_names protected-file refusal removed',
        'src/primitives/host_config.rs',
        '    if let Some(last) = names.last()\n        && let Some(&file) = PROTECTED_FILES.iter().find(|f| fold(last) == **f)\n    {\n        return Err(refuse_protected_file(shown, file));\n    }',
        '    if let Some(last) = names.last()\n        && let Some(&file) = PROTECTED_FILES.iter().find(|f| fold(last) == **f)\n    {\n        let _ = file;\n    }',
        'host_config',
    ),
    (
        'M3 merge-permissions protected-set check removed',
        'src/primitives/merge_permissions.rs',
        '    super::host_config::check_no_protected(repo, &target_path, &args.path)?;',
        '    let _ = &target_path;',
        'merge_permissions',
    ),
    (
        'M4 append-task destination check removed',
        'src/primitives/append_task.rs',
        '    super::host_config::check_no_destination(\n        repo,\n        &repo.join(&args.feature_path),\n        &args.feature_path,\n    )?;',
        '    let _ = &args.feature_path;',
        'append_task',
    ),
    (
        'M5 create-scenario destination check removed',
        'src/primitives/create_scenario.rs',
        '    super::host_config::check_no_destination(repo, &feature_dir, &args.feature_path)?;',
        '    let _ = &feature_dir;',
        'create_scenario',
    ),
    (
        'M6 check_no_protected protected-directory refusal removed',
        'src/primitives/host_config.rs',
        '        if let Some(seq) = protected_dir_in(names) {\n            return Err(refuse_protected_dir(shown, seq));\n        }',
        '        if let Some(seq) = protected_dir_in(names) {\n            let _ = seq;\n        }',
        'merge_permissions',
    ),
    (
        'M7 check_no_protected protected-file refusal removed',
        'src/primitives/host_config.rs',
        '            if let Some(&file) = PROTECTED_FILES.iter().find(|f| folded == **f) {\n                return Err(refuse_protected_file(shown, file));\n            }',
        '            if let Some(&file) = PROTECTED_FILES.iter().find(|f| folded == **f) {\n                let _ = file;\n            }',
        'merge_permissions',
    ),
    (
        'M8 check_no_protected MCP/settings-file refusal removed',
        'src/primitives/host_config.rs',
        '            if let Some(&file) = CONFIG_FILES.iter().find(|f| folded == **f) {\n                return Err(refuse_file(shown, file));\n            }',
        '            if let Some(&file) = CONFIG_FILES.iter().find(|f| folded == **f) {\n                let _ = file;\n            }',
        'merge_permissions',
    ),
]
