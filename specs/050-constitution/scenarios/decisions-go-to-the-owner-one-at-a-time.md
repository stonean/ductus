---
section: "Follow-on scenarios"
---

# Decisions-go-to-the-owner-one-at-a-time

## Context

During a rule-file review an agent put the owner's open decisions to them as a group, first as one multi-question prompt and later as a seven-item numbered list. The grouped prompt was pushed out of view by background-agent reports and went unanswered. The list drew a correction: *"when i need to make decisions do NOT group all questions together. present each question, one at a time to be addressed. that way the question or response can be clarified. presenting as a group does not offer a clean way to do this"* — and the owner noted they had said so before. A grouped list makes the owner answer every question before any one can be clarified, and an answer like "3b" cannot be questioned on its own without unpicking the others. The owner decided the rule applies to adopting projects too, as a default a project may replace in its own `AGENTS.md`.

## Behavior

- When the agent has more than one decision for the owner, it presents one per message — the question, the context it turns on, its options, and its recommendation (worked out per §recommendations) — then waits for the answer before presenting the next.
- It does not group decisions into one list or one multi-question prompt, and it does not put a question while work it started is still reporting into the conversation, where the question scrolls out of view.
- The rule is a default. A project MAY replace it in its `AGENTS.md`, and the constitution's floor (§governance-precedence) names that exception, so replacing it is not loosening a rule the floor forbids loosening.

## Edge Cases

- A decision whose answer changes another pending one is still asked alone; the agent revisits the dependent question, with the impact stated, when it comes up.
- Approval gates a command already defines one at a time (a status transition, `/{project}:clarify`'s per-question loop) are unchanged.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
