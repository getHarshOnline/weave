# Pale integration

These are planned acceptance cases, not passing tests. Apply the [shared case requirements](../README.md) to each branch.

**Context without Scope.** Exact hydration from a Pale pointer, compared with the optimised path through Scope, with the required context recheck before use. Exercise missing mandatory evidence, revocation after delivery and an exhausted context budget.

**The live seam.** Match the published occurrence contract and the consumer pin before qualifying a live seam.

## Reconcile a lost command reply

In the selected Weave–Pale profile, lose the command reply after Weave commits a Pale-initiated mutation. Deliver the committed occurrence while Pale still has no reply, then redeliver it after a later authorised Weave edit. Verify that Pale links it to the original operation using Weave's commit receipt and exact read-back. Read back one original command, one transition and one outbox occurrence for that mutation. The later Weave revision stays in place.

Withhold the native read-back, then supply a valid occurrence for a different causal parent or command in separate runs. Keep the original Pale operation unresolved. Verify that Pale sends no second command and records no successful settlement for the mismatched reference. As a control, deliver a separately admitted Weave-originated occurrence with no Pale parent. Pale handles it under its selected policy without sending a Weave mutation to acknowledge delivery. See the [occurrence contract](../../contracts/pale-occurrences.md).

## A child result leaves a separate parent decision

Read the child owner’s committed outcome, Pale’s linked observation and Weave’s parent revision separately. The dispatch and Scope-admission cases belong to Pale; these cases exercise the receiving parent command.

| Exercise | Required result |
| --- | --- |
| Supply a valid Pale observation and matching handoff, but withhold Gas Town's native commitment evidence. Repeat with a different digest returned at the same owner reference. | The result-dependent command cannot commit. Read back the unchanged parent revision and absence of a parent disposition in the outbox. Restore the exact committed outcome and verify that a fresh authorised command can consider it. |
| Admit a replacement after Gas Town commits the predecessor disposition. With the parent revision unchanged, supply the predecessor's genuine result as evidence for the replacement. Repeat with a genuine result for a different Weave publication. | Preserve each result under its own lineage. Weave refuses the mismatched evidence and commits no parent transition or acceptance outbox row. A matching result is eligible for review under the parent command's own authority and revision checks. |
| After an assisted launch, deliver the exact Gas Town result and admitted handoff to Weave. Run parent acceptance and Scope terminal delivery in both orders. Repeat with Scope expiry before terminal delivery. | An authorised parent command can commit against the current parent revision in each run. Read back both owners after each transition. Scope's update leaves the parent unchanged. The parent command leaves the episode unchanged, including when Scope records the delayed event as late evidence after expiry. |
| Supply an optional terminal episode reference, then substitute a reference with a different attempt or a different handoff in separate runs | Weave can consider the exact matching reference. Each substitution is rejected as evidence and the parent revision stays unchanged. |
| Advance the Weave parent while the child runs, then submit acceptance against the old revision. Repeat by advancing the parent after command validation but before commit. | The child result remains evidence. The stale command commits no parent transition or outbox row. A fresh command must use the current revision and review the evidence again. |
| Lose the response after Weave commits an authorised acceptance, rejection or reopening. Advance the parent with another authorised command before replay. | Recover the original disposition and commit receipt by command identity and native read-back. Exact replay adds no second transition or outbox occurrence and leaves the later parent revision in place. |
