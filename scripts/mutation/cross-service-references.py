"""Mutation list for spec 030's duplicated-repo report: `resolve-references`
(`runtime/src/primitives/resolve_references.rs`, `run`) returns
`duplicate-repos`, each repo the `[services]` registry holds under two or more
aliases, grouped by `Services::duplicate_repos`
(`runtime/src/schema/services.rs`) under the harvester's own identity,
`normalize_repo`.

Run with `python3 scripts/mutation/run.py
scripts/mutation/cross-service-references.py [ID ...]`; see run.py for the
format. This file is data, read with ast.literal_eval: keep every value a
literal. An id is never reused: a mutation that no longer applies is removed,
and the next one takes the next free number. When the code a mutation anchors
on changes, update the anchor in the same change; `run.py --check` names every
anchor that no longer matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the registry\'s duplicated repos are not reported',
        'src/primitives/resolve_references.rs',
        '    let duplicate_repos = services\n        .duplicate_repos()\n        .into_iter()\n        .map(|(repo, aliases)| DuplicateRepo { repo, aliases })\n        .collect();\n',
        '    let duplicate_repos = Vec::new();\n',
        'resolve_references::tests::a_repo_under_two_aliases_is_reported_with_both',
    ),
    (
        'M2 duplicated repos are grouped by the raw string, not the harvester\'s identity',
        'src/schema/services.rs',
        '                .entry(normalize_repo(&entry.repo))\n',
        '                .entry(entry.repo.clone())\n',
        'resolve_references::tests::a_repo_under_two_aliases_is_reported_with_both',
    ),
]
