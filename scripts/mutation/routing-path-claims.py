"""Mutation list for spec 022 task 62 (scenario `routing-ranks-by-claimed-path`):
`derive-routing-candidates`' path-claim source offers each path a description
names to every spec whose plan's Affected Files claims it, an exact path, then
a glob, then a directory, then by spec number; reports a named path no plan
claims as `unclaimed-paths`, and none while a plan is unread; and the
`routeInboxItem` request carries those paths.

Run with `python3 scripts/mutation/run.py scripts/mutation/routing-path-claims.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

The killing tests are in runtime/src/primitives/derive_routing_candidates.rs
(M1-M10, M12, M13) and runtime/src/interpreter/payload.rs (M11). M6's test is Unix
only, so on Windows M6 survives.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the path-claim source reads no path',
        'src/primitives/derive_routing_candidates.rs',
        '    if named.is_empty() {\n',
        '    if true {\n',
        'primitives::derive_routing_candidates::tests::path_claims_rank_exact_then_glob_then_directory_then_by_number',
    ),
    (
        'M2 path claims keep the listing order, not the claim rank',
        'src/primitives/derive_routing_candidates.rs',
        '    claims.sort_by_key(|(rank, ..)| *rank);\n',
        '',
        'primitives::derive_routing_candidates::tests::path_claims_rank_exact_then_glob_then_directory_then_by_number',
    ),
    (
        'M3 a glob claim ranks as a directory and a directory as a glob',
        'src/primitives/derive_routing_candidates.rs',
        "    Some(if entry.contains(['*', '?', '{']) {\n",
        "    Some(if !entry.contains(['*', '?', '{']) {\n",
        'primitives::derive_routing_candidates::tests::path_claims_rank_exact_then_glob_then_directory_then_by_number',
    ),
    (
        'M4 an entry that is the path claims nothing',
        'src/primitives/derive_routing_candidates.rs',
        "    if entry.trim_end_matches('/') == path.trim_end_matches('/') {\n",
        '    if false {\n',
        'primitives::derive_routing_candidates::tests::path_claims_rank_exact_then_glob_then_directory_then_by_number',
    ),
    (
        'M5 a path is reported unclaimed while a plan is unread',
        'src/primitives/derive_routing_candidates.rs',
        '    let unclaimed = if unread {\n',
        '    let unclaimed = if false {\n',
        'primitives::derive_routing_candidates::tests::an_unreadable_plan_is_skipped_by_name_and_holds_back_unclaimed',
    ),
    (
        'M6 a plan linked outside the project is read',
        'src/primitives/derive_routing_candidates.rs',
        '        let entries = if crate::primitives::leaves_project(repo, &plan_rel) {\n',
        '        let entries = if false {\n',
        'primitives::derive_routing_candidates::tests::a_plan_linked_outside_the_project_is_skipped_by_name_not_read',
    ),
    (
        'M7 no named path is ever reported unclaimed',
        'src/primitives/derive_routing_candidates.rs',
        '            .filter(|(_, count)| **count == 0)\n',
        '            .filter(|(_, count)| **count == usize::MAX)\n',
        'primitives::derive_routing_candidates::tests::a_named_path_no_plan_claims_is_its_own_result_not_an_empty_list',
    ),
    (
        'M8 a line-number citation is not stripped before the path is read',
        'src/primitives/derive_routing_candidates.rs',
        '        &crate::primitives::line_citations::strip(description),\n',
        '        description,\n',
        'primitives::derive_routing_candidates::tests::a_path_is_read_from_a_code_span_with_its_line_citation_stripped',
    ),
    (
        'M9 runtime-work offers again a spec a path claim offered',
        'src/primitives/derive_routing_candidates.rs',
        '        Some(slug) if candidates.iter().any(|c| c.target == slug) => {\n',
        '        Some(slug) if false => {\n',
        'primitives::derive_routing_candidates::tests::runtime_work_does_not_offer_again_a_spec_a_path_claim_offered',
    ),
    (
        'M10 a claim\'s reason drops how many plans claim the path',
        'src/primitives/derive_routing_candidates.rs',
        '        format!("1 of {claimants} plans that did")\n',
        '        "one plan that did".to_string()\n',
        'primitives::derive_routing_candidates::tests::a_path_claims_reason_says_its_plan_changed_the_path_never_that_it_owns_it',
    ),
    (
        'M11 the routeInboxItem request drops the unclaimed paths',
        'src/interpreter/payload.rs',
        '        .get("unclaimed-paths")\n',
        '        .get("unclaimed-paths-unread")\n',
        'interpreter::payload::tests::route_inbox_item_carries_the_paths_no_plan_claims',
    ),
    (
        'M12 the returned result drops the unclaimed paths',
        'src/primitives/derive_routing_candidates.rs',
        '        unclaimed_paths: derived.unclaimed_paths,\n',
        '        unclaimed_paths: Vec::new(),\n',
        'primitives::derive_routing_candidates::tests::the_returned_result_carries_the_named_and_the_unclaimed_paths',
    ),
    (
        'M13 a spec root that cannot be listed counts the path-claim source as examined, not skipped',
        'src/primitives/derive_routing_candidates.rs',
        '    let Some(listed) = listed else {\n        skipped.push(RoutingSkip {\n            source: "path-claim".into(),\n',
        '    let Some(listed) = listed else {\n        examined.push("path-claim".into());\n        skipped.push(RoutingSkip {\n            source: "path-claim".into(),\n',
        'primitives::derive_routing_candidates::tests::a_named_path_with_no_spec_root_skips_the_path_claim_source',
    ),
]
