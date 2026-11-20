---
section: "Follow-on scenarios"
---

# Spec-side-parser-hardening

## Context

The tasks.md-side walkers received parsing hardening over time — `SkipScanner` comment/fence awareness, CRLF-tolerant frontmatter splitting — that the spec.md-side walkers never got. The 2026-07-11 runtime accuracy review surfaced four resulting defects, all splitting the two-paths guarantee (an LLM reading the file and the runtime parsing it disagree):

- `validate-frontmatter` emits no finding when `status` or `dependencies` is missing entirely — both checks are presence-guarded — so a spec missing a required field reports `clean: true`, while the constitution's §text-first-artifacts Validation Severity classifies a missing `status`/`dependencies` as hard-fail, and `read-spec`/`traverse-deps` halt with an operational YAML error on the same file.
- `read-spec`'s acceptance-criteria walker and `mark-criterion` have no comment/fence awareness. The shipped spec template embeds an example `- [ ]` checkbox inside the Acceptance Criteria guidance comment, so every template-state spec reports a phantom criterion, and `mark-criterion` will flip a checkbox inside an HTML comment.
- `set-status` hardcodes the frontmatter opener offset to the 4-byte `---\n` while `split_frontmatter` also accepts a 5-byte `---\r\n` opener; on CRLF checkouts every transition splices one byte early and corrupts the frontmatter.
- `check-rule-ids`' deprecation scan slices the rule file at a raw byte offset (`abs + 256`); landing mid-UTF-8-character panics. Shipped rule files are em-dash-dense, so the panic is reachable from ordinary adopter files and violates the descriptive-error-JSON contract.

## Behavior

The spec-side parsers match the hardening standard the tasks-side parsers already meet:

- `validate-frontmatter` reports a hard-fail finding when `status` or `dependencies` is absent from spec frontmatter (field presence, not just value validity, per constitution §text-first-artifacts).
- `read-spec`'s acceptance-criteria walk and `mark-criterion`'s checkbox addressing skip content inside HTML comments and fenced code blocks (`SkipScanner` semantics), so a template-state spec reports zero acceptance criteria on both paths and comment-embedded checkboxes are never flipped.
- `read-spec`'s acceptance-criteria walk folds a line that directly follows a criterion, with no blank line between, into that criterion when it starts no other block (a heading, list item, block quote, table row or thematic break), indented or not, so an edit to a wrapped criterion's second line changes its text and `classify-spec-edit` reopens a `done` spec for it. The criteria's indexes stay keyed to checkbox lines alone.
- `set-status` derives the frontmatter start offset from the actual opener the splitter matched, so CRLF spec files splice at the correct byte.
- `check-rule-ids`' deprecation scan only slices on `char` boundaries and cannot panic on multibyte content.

## Edge Cases

- A spec whose frontmatter is present but empty (`---\n---\n`) reports both missing-field findings, not a parse halt.
- An unindented line after a blank line, or an unindented line that starts a list item, is not a lazy continuation; an indented line folds as before.
- A checkbox on the same line as an opening HTML comment delimiter follows the documented `SkipScanner` delimiter-line behavior.
- The deprecation scan's 256-byte window can still false-positive a live rule that sits within 256 bytes of a `**DEPRECATED` marker belonging to a neighbor; scoping the scan to the rule's own section is tracked in the inbox, not this scenario.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
