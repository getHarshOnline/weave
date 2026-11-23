# Committed occurrences and Pale

This is Weave's design for publishing its own committed facts to Pale. The exact wire schema, signing profile and joint settlement format still require implementation and qualification by both owners. Compiling the crates or reading this description does not establish a supported connector.

## One request through Pale and back

Take an investigation of a customer's failed export. Someone creates it in Weave, assigns it and links the request, and Weave commits the change together with its outgoing occurrence. The publisher delivers the reference. Pale reads the committed occurrence, checks that the work is permitted and chooses the agent, model, tool or machine. The executor's result comes back for that investigation and that attempt. A reviewer then accepts, rejects or reopens the investigation through a separate Weave command. If the investigation changed during the run, or a blocker appeared after the reviewer's check, the command conflicts and the result waits for a fresh decision. The sections below say what each product records and checks.

## Each owner records its part

Weave owns semantic commands, record and relation transitions, organisational causal links and commit receipts. Pale owns admitted operations, leases, execution history, constitutional consequences and its journal. Neither copies the other's causal body or infers settlement from delivery.

For a command originating in Pale, Pale allocates its operation coordinate before invoking Weave. Weave records that immutable causal parent and returns its own occurrence coordinate. Pale links the result to its original operation. For a change originating in Weave, Pale records the Weave coordinate when reconciling the occurrence; Weave makes no circular update merely to acknowledge that Pale saw it.

An occurrence identifies the origin instance, incarnation or producer epoch, immutable occurrence ID and canonical content digest. Its committed actor, trigger and workflow references are preserved. Retries retain the coordinate; changed bytes under that coordinate conflict. A new delegated parent must be an actual Weave transaction, never a record fabricated by integration configuration.

## Publication and recovery

The Weave commit writes its secret-free outbox entry atomically with history and result. The publisher sends a bounded reference through Pale's event admission and keeps its own delivery and recovery state. It selects no agent, model, tool or machine. A notification, acknowledgement or delivered occurrence grants no permission and proves no consumer action.

Consumers verify source identity, incarnation, digest, ordering and gaps under their pinned contract, then read back the exact committed occurrence through an authorised route. Missing native evidence leaves the outcome unresolved. A later record revision cannot replace the original occurrence's result.

When a Pale-originated command's reply is lost, a matching committed occurrence plus native read-back can supply its result to the original Pale operation. Exact replay must leave one command, transition and outbox occurrence, even after a later permitted edit. A genuine result for another command or causal parent cannot settle the original operation. A separately admitted Weave-originated event needs no return mutation just to acknowledge it.

## What Weave checks on a Pale-controlled command

The enabled sign-in profile validates issuer, audience, world, instance, actual actor, operation and target, permission reference, expected revisions, purpose and limits, expiry and revocation, idempotency and causation. Weave then checks its domain rules and protected read set. Pale's approval cannot bypass local consistency, and Weave does not replay Pale's entire constitutional graph.

A fact read from Weave can inform another system's declared input; it cannot create that system's principal, delegation, permission, provider or machine binding. Managed fields have a separate writer and staged activation, as described in [Concepts](../docs/concepts.md#three-kinds-of-change-have-separate-writers).

## A child result is evidence for a parent decision

An engineering result returned through Pale binds the originating Weave publication, admitted handoff and exact execution attempt to the executor's committed outcome. Acceptance, rejection or reopening is a separate authorised command against the current Weave parent revision. A merge is not business acceptance.

If the parent changes while the child runs or between validation and commit, retain the child result and require a fresh decision. An optional terminal context-episode reference must match the same attempt and handoff. Weave can consider the exact executor result independently of that episode's closure or expiry; the parent command leaves the episode unchanged. Recover a committed parent disposition by its original command identity without adding another transition.

The [Pale acceptance cases](../tests/pale/README.md) specify lost replies, mismatched references, independent closure, parent races and replay. Pale owns its own dispatch, context-admission and execution-replacement cases.
