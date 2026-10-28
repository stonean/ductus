"""Mutation list for `rewrite-spec-links` (`runtime/src/primitives/rewrite_spec_links.rs`,
spec 051): the referrer behaviours its tests claim since commit 3b359742, and
the fix round on that review's findings 1-4 and 8 — every referrer, the spec
root's `system.md` among them, read before anything is written, a skipped one
reported, a write through a linked directory outside the project refused, a
kept tail reported, and the fragment that ran into the next token.

Run with `python3 scripts/mutation/run.py scripts/mutation/rewrite-spec-links.py
[ID ...]`; see run.py for the format. This file is data, read with
ast.literal_eval: keep every value a literal. An id is never reused: a
mutation that no longer applies is removed, and the next one takes the next
free number. When the code a mutation anchors on changes, update the anchor
in the same change; `run.py --check` names every anchor that no longer
matches exactly once.

M7, M11 and M12 anchor outside the primitive — the atomic write's mode
capture and the orphan check's shared token scanner — because the behaviour
the test names is one the primitive gets by calling them; each is filtered to
this primitive's tests, so a kill shows that its test, not the helper's own,
holds it.
"""

EXPECTED_SURVIVORS = {}

MUTATIONS = [
    # -- behaviours the window's tests claim (commit 3b359742) -----------------
    (
        'M1 a referrer token matched anywhere on its path, not at the spec root',
        'src/primitives/rewrite_spec_links.rs',
        '(segments.get(1) == Some(&from)).then(|| rebuild(&segments, 1, fragment, target))',
        'segments.iter().position(|segment| *segment == from).map(|index| rebuild(&segments, index, fragment, target))',
        'rewrite_spec_links',
    ),
    (
        'M2 a referrer token matched by prefix, not by whole segment',
        'src/primitives/rewrite_spec_links.rs',
        '(segments.get(1) == Some(&from))',
        '(segments.get(1).is_some_and(|segment| segment.starts_with(from)))',
        'rewrite_spec_links',
    ),
    (
        'M3 a referrer token\'s fragment not taken along',
        'src/primitives/rewrite_spec_links.rs',
        '        let end = span.end + fragment_len(&line[span.end..]);\n',
        '        let end = span.end;\n',
        'rewrite_spec_links',
    ),
    (
        'M4 a referrer that is a symbolic link followed and written through',
        'src/primitives/rewrite_spec_links.rs',
        '            let meta = match std::fs::symlink_metadata(&path) {\n',
        '            let meta = match std::fs::metadata(&path) {\n',
        'rewrite_spec_links',
    ),
    (
        'M5 system.md rewritten again after the walk, its pre-read copy not taken out',
        'src/primitives/rewrite_spec_links.rs',
        '                let referrer = pending.remove(index);\n',
        '                let referrer = PendingReferrer {\n                    path: pending[index].path.clone(),\n                    content: pending[index].content.clone(),\n                    passes: pending[index].passes,\n                    prefixes: pending[index].prefixes.clone(),\n                };\n',
        'rewrite_spec_links',
    ),
    (
        'M6 the spec root hardcoded as specs/ instead of the configured one',
        'src/primitives/rewrite_spec_links.rs',
        '        root: check_orphaned_references::spec_root_prefix(&layout),\n',
        '        root: "specs/".to_string(),\n',
        'rewrite_spec_links',
    ),
    (
        'M7 a rewritten file loses its mode',
        'src/primitives/mod.rs',
        '        std::fs::metadata(path)\n            .ok()\n            .map(|meta| meta.permissions().mode())\n',
        '        None::<u32>\n',
        'rewrite_spec_links',
    ),
    (
        'M8 an absent referrer raised as an error',
        'src/primitives/rewrite_spec_links.rs',
        'std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory',
        'std::io::ErrorKind::NotADirectory',
        'rewrite_spec_links',
    ),
    (
        'M9 the token pass disabled (no referrer, system.md included, is scanned for path tokens)',
        'src/primitives/rewrite_spec_links.rs',
        '    fn tokens(self) -> bool {\n        matches!(self, Self::Tokens | Self::Both)\n    }\n',
        '    fn tokens(self) -> bool {\n        let _ = self;\n        false\n    }\n',
        'rewrite_spec_links',
    ),
    (
        'M10 system.md walked for links only, not both passes',
        'src/primitives/rewrite_spec_links.rs',
        '            let passes = if path.starts_with(specs_dir) {\n                Passes::Both\n',
        '            let passes = if path.starts_with(specs_dir) {\n                Passes::Links\n',
        'rewrite_spec_links',
    ),
    (
        'M11 a revision-qualified path read as a reference by the shared scanner',
        'src/primitives/check_orphaned_references.rs',
        "\n                || (previous == Some(':') && names_a_git_object(&line[..token - 1]));\n",
        ';\n',
        'rewrite_spec_links',
    ),
    (
        'M12 a spec-root path inside a URL read as a reference by the shared scanner',
        'src/primitives/check_orphaned_references.rs',
        '            let not_a_reference = previous.is_some_and(is_path_char)\n',
        '            let not_a_reference = false\n',
        'rewrite_spec_links',
    ),
    (
        'M13 the referrers outside the spec root are read but never written',
        'src/primitives/rewrite_spec_links.rs',
        '    for referrer in pending {\n        sweep.rewrite(\n            &referrer.path,\n            &referrer.content,\n            referrer.passes,\n            &referrer.prefixes,\n        )?;\n    }\n',
        '    let _ = pending;\n',
        'rewrite_spec_links',
    ),
    # -- the fix round on findings 1-4 and 8 ------------------------------------
    (
        'M14 a non-UTF-8 referrer raised as an error, not skipped',
        'src/primitives/rewrite_spec_links.rs',
        '            let Ok(content) = String::from_utf8(bytes) else {\n                self.skip(shown, check_orphaned_references::UNREADABLE_AS_TEXT);\n                continue;\n            };\n',
        '            let content = String::from_utf8(bytes).map_err(|err| PrimitiveError::Io {\n                path: path.clone(),\n                source: std::io::Error::new(std::io::ErrorKind::InvalidData, err),\n            })?;\n',
        'rewrite_spec_links',
    ),
    (
        'M15 the non-UTF-8 reason no longer the orphan check\'s',
        'src/primitives/rewrite_spec_links.rs',
        '                self.skip(shown, check_orphaned_references::UNREADABLE_AS_TEXT);\n',
        '                self.skip(shown, "not UTF-8");\n',
        'rewrite_spec_links',
    ),
    (
        'M16 the referrers read after the spec-root walk has written',
        'src/primitives/rewrite_spec_links.rs',
        '    let mut pending = sweep.read_referrers(&referrers, &specs_dir)?;\n    sweep.walk_spec_root(&specs_dir, &referrer_paths, &mut pending)?;\n',
        '    let mut pending = Vec::new();\n    sweep.walk_spec_root(&specs_dir, &referrer_paths, &mut pending)?;\n    pending.extend(sweep.read_referrers(&referrers, &specs_dir)?);\n',
        'rewrite_spec_links',
    ),
    (
        'M17 a referrer that is not a regular file opened rather than refused',
        'src/primitives/rewrite_spec_links.rs',
        '            if !meta.is_file() {\n',
        '            if !meta.is_file() && false {\n',
        'rewrite_spec_links',
    ),
    (
        'M18 a symbolic-link referrer passed over without a skipped entry',
        'src/primitives/rewrite_spec_links.rs',
        '                self.skip(shown, SKIP_LINK);\n',
        '',
        'rewrite_spec_links',
    ),
    (
        'M19 the outside-project check removed',
        'src/primitives/rewrite_spec_links.rs',
        '            match super::validate_contained(self.repo, shown) {\n',
        '            match Ok::<(), PrimitiveError>(()) {\n',
        'rewrite_spec_links',
    ),
    (
        'M20 every linked directory refused, inside the project too',
        'src/primitives/rewrite_spec_links.rs',
        '            match super::validate_contained(self.repo, shown) {\n',
        '            match (if self.repo.join(".githooks").is_symlink() {\n                Err(PrimitiveError::InvalidPath { path: shown.clone(), reason: String::new() })\n            } else {\n                Ok(())\n            }) {\n',
        'rewrite_spec_links',
    ),
    (
        'M21 no kept tail reported',
        'src/primitives/rewrite_spec_links.rs',
        '                kept_tail: names_a_file_the_fold_did_not_write(tail)\n                    || target.does_not_hold_scenario(tail),\n',
        '                kept_tail: false,\n',
        'rewrite_spec_links',
    ),
    (
        'M22 a kept spec.md tail reported',
        'src/primitives/rewrite_spec_links.rs',
        '["" | "spec.md"]',
        '[""]',
        'rewrite_spec_links',
    ),
    (
        'M23 a kept scenarios/ tail reported',
        'src/primitives/rewrite_spec_links.rs',
        ' | ["scenarios", ..])',
        ')',
        'rewrite_spec_links',
    ),
    (
        'M24 a reference to the directory itself reported as a kept tail',
        'src/primitives/rewrite_spec_links.rs',
        '!matches!(tail, [] | ["" | "spec.md"]',
        '!matches!(tail, ["" | "spec.md"]',
        'rewrite_spec_links',
    ),
    (
        'M25 a scenario target reports a kept tail',
        'src/primitives/rewrite_spec_links.rs',
        '                text: rebuilt.join("/"),\n                kept_tail: false,\n',
        '                text: rebuilt.join("/"),\n                kept_tail: true,\n',
        'rewrite_spec_links',
    ),
    (
        'M26 a kept tail\'s line counted from 0',
        'src/primitives/rewrite_spec_links.rs',
        '        let line_number = u32::try_from(index + 1).unwrap_or(u32::MAX);\n',
        '        let line_number = u32::try_from(index).unwrap_or(u32::MAX);\n',
        'rewrite_spec_links',
    ),
    (
        'M27 a span the previous fragment ran over spliced (the panic)',
        'src/primitives/rewrite_spec_links.rs',
        '        if span.start < copied {\n            continue;\n        }\n',
        '',
        'rewrite_spec_links',
    ),
    (
        'M28 a referrer under the spec root (system.md) left unclassified before the walk',
        'src/primitives/rewrite_spec_links.rs',
        '            let path = self.repo.join(shown);\n',
        '            let path = self.repo.join(shown);\n            if path.starts_with(specs_dir) {\n                continue;\n            }\n',
        'rewrite_spec_links',
    ),
    (
        'M29 a skipped referrer under the spec root read by the walk',
        'src/primitives/rewrite_spec_links.rs',
        '            } else if !referrers.iter().any(|referrer| referrer == path) {\n',
        '            } else {\n',
        'rewrite_spec_links',
    ),
    (
        'M30 a carried scenario collapsed onto the routed one on a scenario target',
        'src/primitives/rewrite_spec_links.rs',
        '        Some(slug) if !names_a_scenario(tail) => {\n',
        '        Some(slug) => {\n',
        'rewrite_spec_links',
    ),
    (
        'M31 a carried scenario\'s fragment dropped on a scenario target',
        'src/primitives/rewrite_spec_links.rs',
        '            if let Some(fragment) = fragment {\n',
        '            if let Some(fragment) = fragment.filter(|_| target.scenario.is_none()) {\n',
        'rewrite_spec_links',
    ),
    (
        'M32 the scenarios directory, spelled with a trailing slash, kept as a carried scenario',
        'src/primitives/rewrite_spec_links.rs',
        'matches!(tail, ["scenarios", name, ..] if !name.is_empty())',
        'matches!(tail, ["scenarios", _, ..])',
        'rewrite_spec_links',
    ),
    (
        'M33 a referrer under a file where its directory should be raised as an error',
        'src/primitives/rewrite_spec_links.rs',
        'std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory',
        'std::io::ErrorKind::NotFound',
        'rewrite_spec_links',
    ),
    (
        'M34 a scenario tail the retiring directory does not hold is not listed',
        'src/primitives/rewrite_spec_links.rs',
        '            (["scenarios", name, ..], Some(held)) if !name.is_empty() => !held.contains(*name),\n',
        '            (["scenarios", name, ..], Some(held)) if !name.is_empty() => { let _ = (name, held); false }\n',
        'rewrite_spec_links',
    ),
    (
        'M35 the retiring directory\'s scenarios are never read',
        'src/primitives/rewrite_spec_links.rs',
        '    target.retiring_scenarios = retiring_scenarios(&specs_dir.join(&args.from))?;\n',
        '    let _ = retiring_scenarios;\n',
        'rewrite_spec_links',
    ),
]
