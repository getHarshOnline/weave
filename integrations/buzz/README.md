# Buzz integration

This planned integration lets Buzz conversation carry explicit Weave commands and display permitted views of work. Buzz owns identities, rooms, membership, messages and delivery evidence. Weave owns its records and commands. This package has no working connector yet.

Use stock Buzz through its published contracts. Weave never writes into Buzz's databases or object storage; it uses Buzz's own commands. What a consumer keeps is a limited reference to a Buzz event, the source position it has read up to and its own resulting state. Its Nostr transport is distinct from Weave's [provisional HTTPS exchange](../../docs/instances.md#sameness-is-a-claim-never-a-merge) between instances and from Pale's private federation transport. Conversation can end entirely in Buzz. Selecting communication does not select Buzz's own agent or workflow runtime to execute Weave work.

## Bind the speaker before accepting a command

A Buzz actor is its canonical 32-byte Nostr public key; `npub` displays the same bytes. Display names and NIP-05 handles are mutable metadata and cannot join identities. The key is not a community, room, relay, device, deployment, session or membership.

An accepted, revocable and revisioned binding connects that key to one Weave record with acting capability and an explicit permission scope. It records the issuer, evidence, validity and revocation. Equal keys across independently admitted graphs establish the same Buzz actor only; they import no roles, membership, history, messages or permission. The binding can follow already permitted local relations but cannot create a missing edge. No Buzz-specific rule belongs in the core. A substitute provider can use its own stable handle and signature verifier under this contract.

A replacement key is a different actor unless the provider proves that the new key succeeds the old one. A later link between the two records the relationship without making them equal. Pairing or remote signing can move one existing identity to another device; it creates no new person and grants no authority.

A record can have no binding, several bindings or a stale one without changing its owner. Every acting person or agent appears individually through its own role presence. An outside contact or a referenced work item creates no software access, internal role or room membership. Keys stay in the provider's secret storage; plans, Git, logs and results contain public identity and references only.

## An approval becomes one exact command

When a speaker types “approved,” resolve the key, its permitted scope, one exact action and the current target revision. Ask for clarification if several actions fit and a reply is permitted. Apply all ordinary guards. Record the actual actor in the commit; a refusal leaves work unchanged. The native chat path needs no Pale admission merely because Pale is installed. A deliberately selected Pale-controlled profile additionally requires its signed admission and still applies Weave's own checks.

Unbound identity, ambiguity, stale revision, missing evidence and revoked binding have explicit outcomes. Two rooms approving the same work at the same revision cannot commit twice. Before command admission, a relay outage leaves a visible gap or pending state until the exact signed event is reconciled. After admission, a lost response is recovered by the original command identity.

Retain the Buzz instance, signed event ID, command identity and mapping revision together. A redelivered event reconciles that original command even if the room now points to another work item. Current permission still governs read-back and replies. If the room can no longer receive a response, the outcome remains available to permitted Weave readers.

Chat-to-Weave commands and Weave-to-chat publication have separate contracts. Each identifies participating instances, field and effect limits, mapping version, idempotency, revocation and owner evidence. A room or thread can reference several records and a record can appear in several conversations. Every individual command or placement still resolves to one exact target.

## A card is a view

A card shows compact identity, status, accountable role or steward and the next milestone. Expansion reads current Weave state using permitted credentials. A stale card is labelled and cannot authorise a mutation. Omit foreign objects the reader cannot see unless their owner permits an opaque placeholder. Cards create no membership, stewardship, acceptance or permission.

The card contract precedes implementation. It declares field limits, publisher, staleness clock, identity and idempotency, when access is checked, expansion credentials, fan-out limit, milestone presentation and deleted/unknown states. Weave supplies semantic and presentation descriptors; the client renders them.

## A comment and its publication have separate results

When a group's declared duties include conversation, each genuine comment, reply, mention or question commits one secret-free placement obligation in the same transaction. It binds the comment revision, placement contract and idempotency coordinate. The obligation can remain unresolved; delivery waits for one permitted placement. An outage leaves the comment committed. Recovery before the transaction finds neither; recovery after it resumes the same obligation. Groups without that duty create none, and routine telemetry stays silent.

Once bound, the publication's audience and room remain fixed. A known room does not prove publication. A retry cannot invent another comment, target or room. Clarifications, refusals and card updates need their own publication permission and must respect current field and disclosure limits independently of the command permission.

An attachment upload is itself disclosure. Verify the room, exact reply anchor and attachment grant before sending bytes. Record upload and event outcomes separately; reconcile a partial upload or a later event failure under the provider's retention rules.

## Publication, delivery and reading are different evidence

| Milestone | What it establishes | What it does not establish |
| --- | --- | --- |
| Request recorded | One exact intent to publish | That Buzz accepted it |
| Relay `OK` | The relay's disposition for the named event | Storage when the response says duplicate or superseded, fan-out, delivery, display or reading |
| Exact-ID read-back | The original signed event was visible to that reader at that recorded point | Current visibility after replacement, deletion or revocation, delivery, or anyone's reading |
| Delivery attempt | One recipient or device path was attempted | That it arrived |
| Recipient delivery | Evidence that a specified device received it | Display, attention or reading |
| Display | A client showed it | Attention or reading |
| Own read frontier | One identity synchronised its own reading state | What anyone else read |
| Other reader's receipt | A supported and permitted reading report from that reader | A response, work or acceptance |
| Response | A reply or reaction referencing the event | Work, an effect or acceptance |
| Communication settlement | The declared communication obligations reached their own typed end | Any consequence elsewhere, or acceptance |

No milestone proves work, outside effects or acceptance. A destination's consequence and the initiating owner's acceptance each have their own record. A milestone that Buzz cannot observe stays recorded as unsupported or unknown; it is never inferred from a later one. An observation record proves only that a consumer durably admitted a bounded Buzz reference under its provider, schema, generation and source position. A publication result needs both relay disposition and exact-event read-back, and the read-back is judged under the visibility policy that applies to that reader. A consequence result comes from the owner that performed the requested action. Unsupported or unobserved milestones stay unsupported or unknown.

When the reply is lost, retain `publication_pending_reconciliation`. Read back the original signed event without signing a fresh one. An affirmative duplicate response can describe an exact prior event or a losing replacement that was not stored. Read visibility and relay disposition therefore remain separate. Distinguish a different current event from an unknown result for the original. Record a permitted read-back of the exact event even while the relay's disposition is still unknown. When visibility or retention limits what can be reconciled, keep each missing milestone explicit. A historical read-back does not prove current visibility after replacement, deletion or revocation, delivery or anyone's reading. A function return, queue acknowledgement, WebSocket frame, reconnect or phone notification proves none of those later steps.

## Repair gaps without claiming completeness

A source frontier records how far one subscription has demonstrably read from one source in one provider instance. It may need composite time/event ordering, overlapping reads, duplicate detection, per-room pagination, replacements, deletions and bounded gap repair. Neither a last timestamp nor `EOSE` proves a complete interval. Repair stays within declared limits, and gaps it cannot close stay visible. On a selected Pale route, connect the proved provider frontier to the matching durable journal admission.

What a reader sees combines their current permission to the room with the proved frontier. Distinguish permitted history, denial, deletion, retention-bound absence, stale synchronisation, known gaps, unsupported evidence and unknown state. Counts, previews and alerts must not leak denied content. A missing polling result does not prove deletion; retain historical observation references only as policy allows. Durable outward event delivery and the selected consumer's admission require their own proof before claiming that conversation reliably triggers another system.

## Interrupt only where the duty calls for it

A critical failure follows an explicitly declared escalation relation to its named room under the current generation and publication permission. Ordinary delay queues quietly. The alert uses a protected link; it neither moves the work nor changes membership. Buzz owns publication evidence, while Weave records dereference, acknowledgement, expiry and resolution.

Meaningful progress can appear as natural updates when the group's contract says someone needs it. Raw logs and routine telemetry stay with their owners. Rich mail, call, meeting and research history stays in Weave; a room is raised at a named coordination or decision point. Mail and calendar delivery remains with those providers.

The intended experience includes adding Pale to a community so its configured agents join individually, role-based pickup in threads, `/weave` commands and calls that bring in the relevant agents. These require provider capability and integration work; they are not available commands. The [acceptance cases](tests/README.md) preserve the approval, disclosure, gap and recovery branches the implementation must pass.
