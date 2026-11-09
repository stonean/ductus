---
section: "Behavior"
---

# Reference-reads-follow-the-cited-sections

## Context

A command's reference is read where a step points at one of its sections. In the cost round, analyze read its whole reference, 49,132 bytes as installed, in one call returning 50,087 characters with the reader's line numbers, its fourth of 15 model messages, so the 11 after it carried it. clarify read 11,859 characters, groom 10,775, plan 9,781 and fold 5,210, each whole. review listed the headings and read 3,603 (`research.md`, R4).

## Behavior

- A command's run reads, of its reference, the sections its path's steps point at, not the whole file.
- A section a step points at is reachable in one read, without the model first listing the file's headings.
- What the reference holds is unchanged: what a step does with a result, and the host's judgment. Only how much of it a run reads changes.

## Edge Cases

- A section that points at another: the run reads both.
- A reference over the read size is still read section by section, so no read of it crosses the host's limit.
- A run whose path points at no section reads none of the reference.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
