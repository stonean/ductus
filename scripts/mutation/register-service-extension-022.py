"""Mutation list for spec 022's `registerService` extension point, `/{project}:link`'s
registration step under `ductus exec` (scenario `link-registration-is-an-extension-point`,
task 65): the walker writing a validated response through `register-service` (M1), bound to
the response alone (M2) and halting on a refusal (M3); the response's validation (M4); the
request's `missing` fields (M5), its halt on a registry the reader refuses (M6) and its bound
(M7); and the link walk's seed, which drops the session target (M8).

Run with `python3 scripts/mutation/run.py
scripts/mutation/register-service-extension-022.py [ID ...]`; see run.py for the format. This
file is data, read with ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next free number. When
the code a mutation anchors on changes, update the anchor in the same change; `run.py --check`
names every anchor that no longer matches exactly once.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    (
        'M1 the walker does not write a registerService response',
        'src/interpreter/mod.rs',
        '        let registration = (identifier == REGISTER_SERVICE).then(|| response.clone());',
        '        let registration: Option<Value> = None;',
        'interpreter::tests::a_register_service_response_is_written_through_the_primitive',
    ),
    (
        "M2 the primitive is bound to the walk's context, not the response alone",
        'src/interpreter/mod.rs',
        '        let dispatched = dispatch_primitive(REGISTER_SERVICE_PRIMITIVE, &bindings, &self.repo);',
        '        let mut bindings = bindings;\n        bindings.extend(self.context.clone());\n        let dispatched = dispatch_primitive(REGISTER_SERVICE_PRIMITIVE, &bindings, &self.repo);',
        'interpreter::tests::a_register_service_response_is_written_through_the_primitive',
    ),
    (
        'M3 a refused registration does not halt the walk',
        'src/interpreter/mod.rs',
        '        self.take_dispatched(\n            REGISTER_SERVICE_PRIMITIVE,\n            dispatched,\n            location,\n            &step_label,\n        )',
        '        let _ = (dispatched, location, step_label);\n        Ok(None)',
        'interpreter::tests::a_refused_registration_halts_the_walk',
    ),
    (
        'M4 a registerService response is not validated',
        'src/schema/extensions.rs',
        '        "registerService" => check!(RegisterServiceResponse),\n',
        '',
        'schema::extensions::tests::register_service_response_requires_alias_repo_and_path',
    ),
    (
        'M5 the request names no field as missing',
        'src/interpreter/payload.rs',
        '        .filter(|(_, value)| value.is_none())\n        .map(|(key, _)| (*key).to_string())',
        '        .filter(|(_, value)| value.is_none() && false)\n        .map(|(key, _)| (*key).to_string())',
        'interpreter::payload::tests::build_register_service_request_names_the_supplied_the_missing_and_the_registered',
    ),
    (
        'M6 a registry the reader refuses is sent as an empty one',
        'src/interpreter/payload.rs',
        '    let registered = crate::primitives::resolve_references::load_services(repo)?',
        '    let registered = crate::primitives::resolve_references::load_services(repo).unwrap_or_default()',
        'interpreter::payload::tests::build_register_service_request_halts_on_a_registry_it_cannot_read',
    ),
    (
        "M7 the registry's entries stay inline past the read size",
        'src/interpreter/payload.rs',
        '        "registerService",\n        typed_only(&typed),\n        &["registered"],',
        '        "registerService",\n        typed_only(&typed),\n        &[],',
        'interpreter::payload::tests::build_register_service_request_moves_the_registry_past_the_read_size',
    ),
    (
        'M8 a walk of link keeps the seeded session target',
        'src/main.rs',
        '    if command == "link" {',
        '    if command == "never" {',
        '--bin ductus a_link_walk_drops_the_seeded_target',
    ),
]
