---
section: "Check Families"
---

# Family-36-self-url-resolution

## Context

Family 35 established that a file the manifest ships cannot cite ductus's own spec corpus with a relative link: `specs/` is this project's development record, it is not in the manifest, and `../../specs/017-…` is a claim about a neighbour the adopter does not have.

Three repairs were possible and two were rejected. **Prose** — dropping the `](…)` and keeping the name — cannot dangle and cannot rot, but it throws away the reason the citation exists: a rule's `**Source:**` field is the evidence an adopter's reviewer leans on, and a pointer they cannot follow is a weaker artifact than one they can. **Rewrite-at-install-time** — keeping relative links here and converting them during the copy — preserves local verifiability and adopter reachability at once, and was rejected as machinery in the install path serving a defect class that a check can prevent outright. The chosen repair is the **absolute URL**: `https://github.com/stonean/ductus/blob/main/specs/017-…`, which resolves from a maintainer's checkout and an adopter's alike because it names the repository instead of assuming it.

**That trade bought correctness in the adopter tree and spent verifiability here.** A relative link is checked by Family 26 and by `check-corpus-links` on every commit. An absolute one is checked by nobody. So consolidating spec 052 into 054, or folding a branch-scoped directory, leaves 32 URLs pointing at paths that no longer exist — and nothing reports it. The reader who finds out is the one who clicks, and for a shipped rule file that reader is an adopter, who cannot distinguish a ductus defect from their own misconfiguration.

Silent rot in the reassuring direction is the shape `QUAL-CLAIM-001` names, and adopting a link form whose failures are invisible would be this repository committing the defect its own rule file describes — in the very file that describes it.

## Behavior

A `/{project}:audit` family resolves every absolute GitHub URL pointing back into this repository against the working tree.

The check is deliberately the obvious one. Every such URL has the form `<repo>/blob/<ref>/<path>` where `<path>` is a path in this repository, so stripping the prefix and testing the remainder is sufficient. No network call, no rate limit, no flake — and **stronger** than the relative-link check it replaced, because there is no depth arithmetic to get wrong. The dominant Family 26 finding is a scenario file one `../` short of its target; that failure mode does not exist here.

- **The repository is derived, never hardcoded.** The slug comes from the installer's own archive URL in `framework/bootstrap/ductus.md` — the single place ductus states its canonical repository. A literal `stonean/ductus` in the family would be a second copy of exactly the fact under test, and it would make every finding *wrong* in a fork rather than merely absent.
- **`main` URLs are resolved.** This project is live-on-main: the installer and everything `/ductus` fetches track it, so a `main` URL is a claim about the current tree and the working tree is the right authority for it.
- **A tag- or sha-pinned URL is deliberately historical.** It names a state the working tree is not, so resolving it against the working tree would manufacture findings. Those are **counted and reported on stderr**, never resolved and never silently dropped — "we did not check these" and "these were fine" must not read alike.

Three findings, because three repairs differ:

- **`unresolved`** — the path does not exist. A genuine 404 waiting for a reader.
- **`blob-names-dir`** and **`tree-names-file`** — the URL kind disagrees with what the path is. GitHub redirects both, so neither is broken today; they are reported because the repairs differ from each other and from a genuine miss, and one message covering all three would send a maintainer looking for the wrong thing.

## Edge Cases

- **A `{placeholder}` URL.** The installer's release-download URLs carry `{pin}` and `{triple}`. These are documented shapes rather than pointers and are excluded by construction.
- **Trailing punctuation, fragments, and queries.** A URL ending a sentence absorbs the period; a `#L12` or `#anchor` is not part of the path. Both are stripped before the existence test, and the path is percent-decoded, so a target with an escaped character resolves rather than reporting falsely.
- **Generated command copies.** Not a subject. They carry whatever their source carries and the repair is a ductus release, not an edit there — the line Families 26 and 34 already draw.
- **An empty markdown corpus** is a finding: a scan that enumerated nothing must not exit like one that enumerated everything.
- **Zero URLs is *not* a finding.** A repository citing nothing absolutely is legitimately clean. The URL count on stderr is what keeps that from reading the same as an extraction that silently stopped matching — the one number separating "every self-referencing URL resolves" from "the regex no longer matches any", which otherwise exit identically.

## Resolved Questions

- **Why not verify the URLs over the network instead?** Because the answer would be about GitHub's availability as much as about the link, and a family that fails on a rate limit teaches maintainers to ignore it. The path portion is a local fact and the local check is the exact one; a network check would be a weaker claim wearing a stronger costume.
- **Should this family also forbid relative links to `specs/` in shipped files, so the two rules live together?** No — that is Family 35's assertion, and it is stated there against the manifest that defines which files are shipped. This family's subject is every tracked markdown file, shipped or not, because an absolute self-URL rots identically in `AGENTS.md` and in a rule file. Two subjects, two families; merging them would give one family a subject that is the union of two unrelated sets.
- **What happens when the trunk is renamed?** Every `blob/main/` URL becomes unresolvable at once and this family reports all of them in a single run, which is the correct outcome: a trunk rename *is* a corpus-wide link rewrite, and discovering it as 32 findings in one commit is strictly better than discovering it as one adopter's 404 months later.
