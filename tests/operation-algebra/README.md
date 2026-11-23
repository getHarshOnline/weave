# Operation roles and graph questions

These are planned acceptance cases, not passing tests. Apply the [shared case requirements](../README.md) to each branch.

**One orchestrator, many kinds of executor.** A person, a tool, an agent and a project each execute work, with zero or more attempts per work item, and a merge or result is not the consumer's acceptance. It includes a person as the subject of an operation, an agent as both actor and subject, an operation that forbids a provider or destination, and lineage appended by the engine.

## Bounded graph answers

The fixture describes no real people. Each edge is a separately identified relation.

| ID  | From | Relation        | To  | Effective interval |
| --- | ---- | --------------- | --- | ------------------ |
| e1  | H    | stewards        | A   | [T0, T2)           |
| e2  | H    | occupies        | R   | [T2, infinity)     |
| e3  | R    | accountable_for | A   | [T2, infinity)     |
| e4  | W    | child_of        | P   | As declared        |
| e5  | P    | contributes_to  | G   | As declared        |
| e6  | W    | blocked_by      | X   | As declared        |
| e7  | E    | supports        | C   | As declared        |
| e8  | C    | considered_in   | D   | As declared        |

At a retained snapshot, H's current responsibility for A returns e2 and e3. W's contribution to G returns e4 and e5 under the goal-explanation policy; e6 appears only when requested. E's bearing on D returns e7 and e8. H's responsibility for A at T1 returns e1 only when the history remains available and the reader may see it. A required governing relation cannot be ranked or budgeted away. These cases test [bounded query answers](../../docs/queries.md).
