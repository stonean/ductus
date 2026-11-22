"""Mutation list for 020's AC19: a stored waiver or violation deferral covers only the
findings at its `(rule, file, site)` anchor, up to the number it covers, and a fixture of
each case fails with its guard removed (`specs/020-code-review/scenarios/waiver-expiry.md`).
Each name ends with the case it proves: (a) a finding at another site is not covered,
(b) more findings than `covers` expire the anchor, (c) an anchor recorded without a site
covers nothing, (d) a site-less finding is covered by no site anchor, (e) `--waive` and
`--defer` refuse a site-less finding, (f) `covers` is recorded per site and lowered, never
raised, (g) the count never collapses across sites, (h) only a rule ID's key reads as
recorded before sites, (i) `process-decisions` counts `fired-violations` as the writer
stores `covers`, (j) a departure waiver covers every site by its `file`.

Run with `python3 scripts/mutation/run.py scripts/mutation/site-anchors-020.py [ID ...]`;
see run.py for the format. This file is data, read with ast.literal_eval: keep every value
a literal. An id is never reused: a mutation that no longer applies is removed, and the
next one takes the next free number. When the code a mutation anchors on changes, update
the anchor in the same change; `run.py --check` names every anchor that no longer matches
exactly once.
"""

EXPECTED_SURVIVORS = {
    "M19": "redundant guard: process-waivers applies an anchor exactly when 0 < count <= covers over the same collapsed findings, so an anchor the run did not apply has no count below covers to lower",
    "M39": "redundant guard: write-review stores a departure waiver by file with no anchors, and process-waivers applies one by its file with no site, so lower_covers, which lowers only an anchor an applied waiver names by site, reaches a departure entry's anchor only when a hand edit gave it one at the site of a hand waiver of its rule",
}

MUTATIONS = [
    (
        'M1 waiver site match removed: a finding at another site of the file counts at the anchor (a)',
        'src/primitives/process_waivers.rs',
        '                    && anchor.site.as_deref().is_none_or(|site| {\n                        finding.site.as_deref().map(str::trim) == Some(site.trim())\n                    })',
        '                    && anchor.site.as_deref().is_none_or(|_site| true)',
        'process_waivers::tests',
    ),
    (
        'M2 waiver site-less finding counts at every site anchor (d)',
        'src/primitives/process_waivers.rs',
        'finding.site.as_deref().map(str::trim) == Some(site.trim())',
        'finding.site.as_deref().map(str::trim).is_none_or(|found| found == site.trim())',
        'process_waivers::tests',
    ),
    (
        'M3 waiver count guard removed: surplus findings never expire the anchor (b)',
        'src/primitives/process_waivers.rs',
        '            && count > covers as usize\n',
        '            && count > covers as usize * 1000 + 1000\n',
        'process_waivers::tests',
    ),
    (
        'M4 waiver count expiry skipped on a restricted run (b restricted)',
        'src/primitives/process_waivers.rs',
        '            && file_exists\n            && count > covers as usize',
        '            && file_exists\n            && !self.restricted\n            && count > covers as usize',
        'process_waivers::tests',
    ),
    (
        'M5 waiver count notice drops the count text (b notice)',
        'src/primitives/process_waivers.rs',
        '"waiver expired: rule {rule} at {at} — {count} findings fire where it covers {covers} ({reason})"',
        '"waiver expired: rule {rule} at {at} ({reason})"',
        'process_waivers::tests',
    ),
    (
        'M6 a file-only non-departure entry is read as a departure waiver (c)',
        'src/primitives/process_waivers.rs',
        '                kind: if departure {\n                    Kind::Departure\n                } else {\n                    Kind::PreSite\n                },',
        '                kind: Kind::Departure,',
        'process_waivers::tests',
    ),
    (
        'M7 pre-site anchor branch removed: falls through to normal placement (c)',
        'src/primitives/process_waivers.rs',
        '            if located.kind == Kind::PreSite {',
        '            if false && located.kind == Kind::PreSite {',
        'process_waivers::tests',
    ),
    (
        'M8 pre-site anchor retained on a restricted run (c restricted)',
        'src/primitives/process_waivers.rs',
        '            if located.kind == Kind::PreSite {',
        '            if located.kind == Kind::PreSite && !pass.restricted {',
        'process_waivers::tests',
    ),
    (
        'M9 pre-site notice drops the recorded-before-sites text (c notice)',
        'src/primitives/process_waivers.rs',
        ' — recorded before sites ({shown_reason})"\n                ));\n                classified.expired.push(anchor);',
        ' ({shown_reason})"\n                ));\n                classified.expired.push(anchor);',
        'process_waivers::tests',
    ),
    (
        'M10 a departure waiver listing paths is read as recorded before sites (j)',
        'src/primitives/process_waivers.rs',
        '                kind: if departure {\n                    Kind::Departure\n                } else {\n                    Kind::PreSite\n                },',
        '                kind: Kind::PreSite,',
        'process_waivers::tests departure_waivers',
    ),
    (
        'M11 a departure anchor is count-capped like a site anchor (j)',
        'src/primitives/process_waivers.rs',
        '        if let Kind::Site { covers, .. } = kind\n            && file_exists\n            && count > covers as usize',
        '        let covers = match kind {\n            Kind::Site { covers, .. } => covers,\n            _ => 1,\n        };\n        if file_exists\n            && count > covers as usize',
        'process_waivers::tests departure_waivers',
    ),
    (
        'M12 collapse merges findings across sites (g)',
        'src/primitives/write_review.rs',
        'existing.place() == finding.place()\n                && ranges_overlap(range, parse_range(existing.range()))',
        'existing.place().0 == finding.place().0 && existing.place().1 == finding.place().1\n                && ranges_overlap(range, parse_range(existing.range()))',
        'process_waivers::tests write_review::tests process_decisions::tests',
    ),
    (
        'M13 process-waivers counts findings uncollapsed (f)',
        'src/primitives/process_waivers.rs',
        'counted: collapse_at_sites(&args.fired),',
        'counted: args.fired.iter().collect(),',
        'process_waivers::tests',
    ),
    (
        'M14 new waiver covers counts findings uncollapsed (f)',
        'src/primitives/write_review.rs',
        '    let counted = collapse_at_sites(&args.findings);\n    for (idx, waiver) in args.new_waivers.iter().enumerate() {',
        '    let counted: Vec<&ReviewFinding> = args.findings.iter().collect();\n    for (idx, waiver) in args.new_waivers.iter().enumerate() {',
        'write_review::tests',
    ),
    (
        'M15 waiver covers can be raised (f)',
        'src/primitives/write_review.rs',
        '                && count > 0\n                && count < covers\n            {\n                anchor.covers',
        '                && count > 0\n                && count != covers\n            {\n                anchor.covers',
        'write_review::tests',
    ),
    (
        'M16 waiver covers is never lowered (f)',
        'src/primitives/write_review.rs',
        '                && count > 0\n                && count < covers\n            {\n                anchor.covers',
        '                && count > 0\n                && false\n            {\n                anchor.covers',
        'write_review::tests',
    ),
    (
        'M17 deferral covers can be raised (f)',
        'src/primitives/write_review.rs',
        'if count > 0 && count < covers {',
        'if count > 0 && count != covers {',
        'write_review::tests',
    ),
    (
        'M18 deferral covers is never lowered (f)',
        'src/primitives/write_review.rs',
        'if count > 0 && count < covers {',
        'if false {',
        'write_review::tests',
    ),
    (
        'M19 waiver covers lowered for anchors the run did not apply (f)',
        'src/primitives/write_review.rs',
        '                held.rule == rule && held.file == file && held.site.as_deref() == Some(site)',
        '                true',
        'write_review::tests',
    ),
    (
        'M20 --waive accepts a finding naming no site (e)',
        'src/primitives/write_review.rs',
        '            .find(|finding| finding_site(finding).is_none())',
        '            .find(|_finding| false)',
        'write_review::tests',
    ),
    (
        'M21 --defer accepts a finding naming no site (e)',
        'src/primitives/write_review.rs',
        '            && finding_site(finding).is_none()\n        {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "write-review".into(),\n                argument: format!("findings[{idx}].disposition"),',
        '            && false\n        {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "write-review".into(),\n                argument: format!("findings[{idx}].disposition"),',
        'write_review::tests',
    ),
    (
        'M22 --defer lets a finding naming no site through on a decision-key (e)',
        'src/primitives/write_review.rs',
        '            && finding_site(finding).is_none()\n        {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "write-review".into(),\n                argument: format!("findings[{idx}].disposition"),',
        '            && finding_site(finding).is_none()\n            && decisions::nonblank(finding.decision_key.as_deref()).is_none()\n        {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "write-review".into(),\n                argument: format!("findings[{idx}].disposition"),',
        'write_review::tests',
    ),
    (
        'M23 pre-site key test accepts any word after rule (h)',
        'src/primitives/decisions.rs',
        '.is_some_and(|(id, _)| super::check_rule_ids::is_a_rule_id(id))',
        '.is_some()',
        'process_decisions::tests',
    ),
    (
        'M24 pre-site key test drops the no-backtick guard (c)',
        'src/primitives/decisions.rs',
        "\n        && !key.contains('`')\n}",
        '\n}',
        'decisions::tests',
    ),
    (
        'M25 pre-site key expiry removed from process-decisions (c)',
        'src/primitives/process_decisions.rs',
        'if record == DecisionRecord::Review && decisions::is_pre_site_violation_key(key) {',
        'if false && record == DecisionRecord::Review && decisions::is_pre_site_violation_key(key) {',
        'process_decisions::tests',
    ),
    (
        'M26 pre-site key expires only when it fires (c)',
        'src/primitives/process_decisions.rs',
        'if record == DecisionRecord::Review && decisions::is_pre_site_violation_key(key) {',
        'if record == DecisionRecord::Review && firing > 0 && decisions::is_pre_site_violation_key(key) {',
        'process_decisions::tests',
    ),
    (
        'M27 deferral count guard removed (b)',
        'src/primitives/process_decisions.rs',
        '(firing > covers as usize).then(|| {',
        '(firing > covers as usize * 1000 + 1000).then(|| {',
        'process_decisions::tests',
    ),
    (
        'M28 process-decisions fired-violations counted uncollapsed (i)',
        'src/primitives/process_decisions.rs',
        '    let violations = collapse_at_sites(&args.fired_violations)\n        .into_iter()',
        '    let violations = args.fired_violations.iter()',
        'process_decisions::tests write_review::tests',
    ),
    (
        'M29 process-decisions fired-violations keyed without the site (i)',
        'src/primitives/process_decisions.rs',
        'decisions::violation_key(&violation.rule, &violation.file, violation.site.as_deref())',
        'decisions::violation_key(&violation.rule, &violation.file, None)',
        'process_decisions::tests write_review::tests',
    ),
    (
        'M30 violation_key drops the site (a, deferral)',
        'src/primitives/decisions.rs',
        'Some(site) => format!("rule {} at {} in `{site}`", rule.trim(), file.trim()),',
        'Some(_site) => format!("rule {} at {}", rule.trim(), file.trim()),',
        'process_decisions::tests write_review::tests',
    ),
    (
        'M31 stored deferral covers is the constant 1 not the collapsed count (i)',
        'src/primitives/write_review.rs',
        'decision.covers = Some(findings_at_key(counted, &key));',
        'decision.covers = Some(1);',
        'write_review::tests',
    ),
    (
        'M32 deferral covers counted uncollapsed in merged_decisions (g/i)',
        'src/primitives/write_review.rs',
        '    let counted = collapse_at_sites(&args.findings);\n    let mut merged',
        '    let counted: Vec<&ReviewFinding> = args.findings.iter().collect();\n    let mut merged',
        'write_review::tests',
    ),
    (
        'M33 report waiver_reason ignores the anchor site (a, report)',
        'src/primitives/write_review.rs',
        '                    .is_none_or(|site| finding_site(finding) == Some(site.trim()))',
        '                    .is_none_or(|_site| true)',
        'write_review::tests',
    ),
    (
        'M34 pruning an expired anchor ignores its site (a)',
        'src/primitives/write_review.rs',
        'entry.rule == rule && entry.file == file && entry.site.as_deref() == site)',
        'entry.rule == rule && entry.file == file && { let _ = site; true })',
        'write_review::tests',
    ),
    (
        'M35 a new waiver holds a rule by file, not by anchor (a)',
        'src/primitives/write_review.rs',
        '.any(|anchor| anchor.is_at(file, site))',
        '.any(|anchor| anchor.file.as_deref() == Some(file))',
        'write_review::tests',
    ),
    (
        'M36 an anchor without a site is usable (c)',
        'src/primitives/mod.rs',
        '            decisions::nonblank(self.site.as_deref())?,',
        '            self.site.as_deref().unwrap_or("x"),',
        'process_waivers::tests write_review::tests',
    ),
    (
        'M37 a new waiver anchors one site per file, not per site (a)',
        'src/primitives/write_review.rs',
        '*sites.entry((finding.file.as_str(), site)).or_insert(0) += 1;',
        '*sites.entry((finding.file.as_str(), "(top level)")).or_insert(0) += 1;',
        'write_review::tests',
    ),
    (
        "M38 invalidation drops a waiver's anchors",
        'src/primitives/write_review.rs',
        '.map_or(EntryValue::Absent, EntryValue::Anchors),',
        '.map_or(EntryValue::Absent, |_| EntryValue::Absent),',
        'invalidate_review::tests',
    ),
    (
        'M39 departure waiver detection in lower_covers removed (j)',
        'src/primitives/write_review.rs',
        '    for waiver in waivers.iter_mut().filter(|waiver| !is_departure(waiver)) {',
        '    for waiver in waivers.iter_mut() {',
        'write_review::tests departure_waivers',
    ),
    (
        'M40 the report dedup merges findings across sites (a, report)',
        'src/primitives/write_review.rs',
        'existing.place() == finding.place()\n                && ranges_overlap(range, parse_range(&existing.line_range))',
        'existing.rule == finding.rule\n                && existing.file == finding.file\n                && ranges_overlap(range, parse_range(&existing.line_range))',
        'write_review::tests',
    ),
    (
        "M41 write-review accepts a violation's decision-key that is not its own (a, deferral)",
        'src/primitives/write_review.rs',
        '            finding.decision_key.as_deref(),\n        ) {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "write-review".into(),\n                argument: format!("findings[{idx}].decision-key"),',
        '            None,\n        ) {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "write-review".into(),\n                argument: format!("findings[{idx}].decision-key"),',
        'write_review::tests',
    ),
    (
        "M42 process-decisions accepts a fired violation's decision-key that is not its own (a, deferral)",
        'src/primitives/process_decisions.rs',
        '            violation.decision_key.as_deref(),\n        ) {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "process-decisions".into(),',
        '            None,\n        ) {\n            return Err(PrimitiveError::InvalidArgument {\n                primitive: "process-decisions".into(),',
        'process_decisions::tests',
    ),
    (
        "M43 key_anchor reads an observation's shape before a sited violation's (retain)",
        'src/primitives/decisions.rs',
        '    let named = sited_violation_key_file(key)\n        .or_else(|| {',
        '    let named = None::<&str>\n        .or(sited_violation_key_file("").filter(|_| false))\n        .or_else(|| {',
        'decisions::tests',
    ),
    (
        'M44 a departure waiver holding anchors is read by its anchors (j)',
        'src/primitives/process_waivers.rs',
        '    if !departure && let Some(anchors) = &waiver.anchors {',
        '    if let Some(anchors) = &waiver.anchors {',
        'departure_waivers',
    ),
]
