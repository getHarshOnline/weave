# Concepts

Weave's model connects typed, revisioned records through relations with their own identity and history. An installed domain defines what those records mean and which operations can change them. This chapter explains the contracts an interface, domain or adapter must preserve. Weave's runtime is not implemented yet.

Consider an investigation of a customer's failed export. Its trace, assignment, review and acceptance need to stay connected without becoming one task status. The [product walkthrough](https://rawthoughts.in/weave/from-request-to-accepted-work/) follows that work from the participants' perspective.

The core knows a few kinds of thing: [records](#records) with owned fields and independent state axes, [relations](#relations-are-records) with named slots, the definitions that govern both, [operations](#operations-have-seven-roles) and each act that runs one, and the [four outcomes](#every-command-ends-in-one-of-four-outcomes) an act ends in. People, agents and organisations are records whose definitions let them act or be acted for, and relations and acts are records too, so everything Weave keeps forms one graph. Every business meaning, from an account to a work item, is a domain installed on top ([Extending Weave](extending.md)).

In Weave's design, the primitive beneath these records is the object. A person, an agent, a piece of work, a relationship, a decision and a guard are each an object, and only the schemas and transitions authored through the SDK say what an object is and where it goes. This carries the rule that the engine has no root class one level further down: actor, record and relation are not kernel kinds either. “Record” stays the word people and this guide use for what Weave keeps; the object is what the kernel knows. An object's state is its readable value as its schema defines it, so its versions and the history of how it changed carry everything else: what a reader saw, what a decision rested on and what a pointer names.

Every object is public, private or protected, as fundamentally as it has an identity. That property belongs to the object, and enforcing it is the work of a guard, which is itself an object, defined a certain way and invoked a certain way. A guard invoked on a read is the access and sandboxing principle, and both federation and Pale's sandbox follow from it. Because every read crosses a guard, the cost of reading is real and cannot be designed away, only placed ([queries](queries.md#every-read-crosses-a-guard)). What protected is relative to is still being decided.

Actions are what happen to objects, and keeping them separate matters. Creating, reading, updating and deleting is the one primitive action, and it can act on any object, whether that object plays the part of an actor, a record or a relation. Not every action is a decision: appending a comment is a create and nothing more. A guard and a transition are each a decision followed by one of those actions, and they serve the same purpose from two sides. A guard decides in order to prevent something; a transition decides in order to optimise the order in which things execute. Every decision is an object and is always kept, read decisions included, because a read decision records what a later decision was based on, and the two together answer whether that basis has changed. Composition goes as deep as authors take the SDK and their schemas: a guard can compose a transition and a guard, and a transition can compose a guard and a transition.

A composition may cycle only when time or an event drives each turn, so a watch that re-arms itself or a retry after a delay is a new execution, never a loop inside one. That makes time a primitive of execution: guards and transitions need a clock and a watchdog while they run. Three clocks stay apart, because confusing them is the classic bug of distributed systems: wall-clock time for deadlines, schedules and expiry; monotonic time for a running budget, which must never jump when someone sets the wall clock; and logical or hybrid time for ordering across instances, which share no clock.

## Records

A record carries its stable semantic identity, owning instance, exact definition and policy versions, revision, state, attributes, source and history, together with disclosure and retention limits. Its semantic identity is independent of its database row ID; no public interface exposes the row ID. People, configured agents, tools, organisations and other collections of actors, services, applications, accounts, evidence, goals, work items and schedules use this shared model. Their definitions give them different meaning, and the engine has no root class for any of them. “Entity” and “node” describe how a record is being considered, not another stored class. Each change keeps the acting record, the policy versions and the permission reference it was admitted under.

State can have several independent axes. The investigation can have an accepted result while a related message change remains open. A domain may also declare a transition that changes several axes together when their shared conditions pass.

Identify which system can change each field and where a reader gets its current value.

| Kind of field | Meaning |
| --- | --- |
| State held in Weave | The owner has made Weave responsible for this state |
| Copy or computed view | Weave reads another source to show the field |
| Organisation rules | The organisation's owner supplies structural roles, grants or bindings |
| Foreign reference | Weave points to information held elsewhere |

For example, Weave could own the investigation's acceptance but hold only a reference to the customer's trace. Delegating a field to Weave identifies its allowed values, source, consent, retention, conflict handling, degraded behaviour and eventual removal from service. Before delegation, a missing field is not a gap the engine must fill.

A copy keeps its owner, source instance and record, source version, observation time, freshness, permitted access, purpose and model-use limits. It also names who resolves conflicts and what happens when access is revoked or the source is retired. The captured source position is called its **source frontier**. Context derived by another service remains distinguishable from a direct source copy. Legal and data owners supply the rights the engine enforces; storing a fact creates none.

Deletion follows the source owner's policy. An authorised deletion or suppression request affects only the matching copy, with a minimal non-content reference and deletion receipt kept where required. Replaying that receipt makes no further change. Old information or a record missing from a synchronisation result calls for review; neither is permission to delete it, and neither is a task, a workflow or a passing date. Weave never deletes copied content on its own initiative: stale, conflicting or withdrawn source state creates review work and blocks anything destructive.

## Acting is a capability, not a kind

A record can act when the operation definition permits it. A person can be the subject of a contact update and the actor accepting an investigation. An agent can investigate or request a change concerning itself where the rules allow it. The engine does not need a separate class of actors.

Every command identifies the actual acting record. The service sending the request, the party it represents, the role it uses and the assignee can all differ. A service submitting an investigator’s result must identify itself and show that it may act for that investigator.

Keep these facts separate: what a record can do, its role and who occupies it, its assigned work, whom it acts for, its software account, whose data the act concerns, who holds the credential and who actually acted. The actual actor needs its own admitted evidence. It is never inferred from a machine, a label or a missing agent attribution.

A principal can be seen two ways, and both hold. Every record that can stand in the world, holding an identity, history, relationships and grants and answering for its own conduct, is a principal: a person, an agent, an organisation or a client organisation. Within one act, the principal is the party the act is for, whose interest it serves and who owns its acceptance. The second view is the first seen through one relation, acting for someone, and that relation's definition decides who may fill each end, and the engine enforces whatever it declares. The rules in the rest of this paragraph are the ones a domain for an organisation of people and agents declares, commercial-work among them. A person acts and can be acted for. An agent acts but is never the principal of an act. An organisation is acted for and acts only through a seat that a person or an agent fills. A tool or service carries or performs an act and is never acted for. A seat is an organisation's position, occupied by a person or an agent; it is a relation, not a kind of record.

Every chain of acting for ends at a person or an organisation. An agent that hands work to another agent is recorded as an intermediary, and the work stays for the same principal. In one act an agent may do only what both its own grants and its principal's delegation allow, and what it learns from a refusal cannot exceed what its principal could see. The agent answers for its conduct within its grant; the principal answers for the outcome and owns acceptance. Who created or configured an agent is a separate relation from whom it acts for, so making an agent gives no claim to be its principal.

A configured agent is one record with one identity and one history, whichever host runs it and whichever model it runs on. The harness host that ran an act and the model it ran on are separate bindings recorded with the act beside the acting record: they say how the act was performed, not who performed it. No host is privileged. Any configured agent, on any accepted host and model, reads and writes the same records, [Radar](../domains/commercial-work/radar.md) included, under the same checks. Changing the host or the model creates no new agent, moves no history and widens no grant.

No command is anonymous to the record model. If agent attribution is absent, keep the credentialed session that issued the change. A default “human” label does not establish that a person performed it. An unobserved origin or actor stays unobserved. A tool definition is distinct from an invocation, and assigning work does not create a permanent agent.

Assignment gives responsibility, not permission. Membership does not automatically grant record access. A contact is not automatically a software user. These distinctions allow a person to hand work to an agent without giving it all their access, and to remain in relationship history after their own software access ends.

A commercial account, legal entity, data controller, organisation, workspace, community, principal, provider and deployment also remain distinct, even when their names match. An outside client does not become a child unit of its service provider or expose its internal organisation. Access belongs to a named acting record at a stated scope; a shared login cannot stand in for that actor. A trusted-context profile must state the privacy limits it has actually proved.

### Bind outside identities explicitly

An identity binding names its issuer, instance, subject, scope, source and validity. Matching names, email addresses, domains or keys never joins records owned by different parties. A binding already accepted by one owner may follow its existing permitted relations, but cannot invent a missing relation.

A chat key reaches a Weave actor only through an accepted, revocable binding. A record may have no binding, several bindings or a stale binding without changing its owner. Membership, role occupancy, assignment, software access and permission remain separate relationships. See [sign-in and access](./interfaces.md).

## Relations are records

A relation has its own identity, definition, revisions, state, policy, source, validity and history. Named participant slots describe what it connects. A relation can involve more than two records; its definition sets allowed definitions or capabilities, cardinality, ordering, uniqueness and whether a participant may belong to another instance. It also declares attributes, independent state axes, operations and access rules.

An assignment might connect the investigation, assignee and role for a stated period. A later assignment involving those same participants is a different relation. A review relation connects the result to its authorised reviewer. A decision can involve several participants, and whether each one decides or is only informed is a property of that participant's slot on the decision, not a rule for the whole organisation. Two equal partners can each decide one matter and only be informed of another. Relations can receive comments and evidence, be superseded or revoked, become the subject of an operation and participate in another relation. Slot names such as `dependent` and `prerequisite` carry meaning that a bare arrow cannot.

Transaction time records when Weave learned a fact; effective time records when the fact applies. Weave’s recording history supplies the former, so a caller’s backdated effective date cannot rewrite when the assertion arrived. Binary edges can represent two-participant relations efficiently. A relation among three or more participants can also be stored through binary structures, provided the relation record keeps its own identity and joint meaning. Splitting a multi-participant rule into pairs, or checking each pair separately, can lose constraints that apply to the whole group.

## Operations have seven roles

An operation declares which roles it needs. These are not seven mandatory fields on every request.

| Role | Meaning | Example: accepting the investigation |
| --- | --- | --- |
| WHO | Acting record, or records where the definition allows several, and any admitted representation | The authorised reviewer |
| WHAT | Subject of the operation | The investigation |
| HOW | Capability and the provider that [binding policy resolves](effects.md#resolve-a-capability-before-selecting-a-provider) for it, if required | No outside provider is needed |
| WHERE | Scope or destination | The owning instance and permitted work |
| WHY | Causal references supplied by the caller | The submitted result and prior context |
| WHEN | Relevant validity, schedule, observation or execution-time constraints, distinct from a revision | The conditions for this decision |
| Δ | Requested change; absent for a read | Accept the named result |

The operation definition makes a role required, optional, plural, forbidden or derived. The runtime adds the command identity, evaluated transition, resulting revisions and commit receipt after the transaction commits and independent read-back confirms it. A caller supplies permitted causal references but cannot forge that appended history. A transition is started by a command, by a schedule coming due or by a reaction the domain declares to a committed occurrence; all three pass the same guards, and none needs an agent to start it.

A comment may require WHO, WHAT, WHY, WHEN and Δ while forbidding an outside provider. An effect may need all seven roles; a query uses its declared actor, subject and scope without a change. These are separate operation definitions composed from shared machinery, not seven schemas or a universal request full of nullable fields.

Caller-supplied causal parents must identify occurrences the engine has already admitted. A later outside effect adds the provider's own result through a separate reconciliation; it cannot rewrite the original commit's history. Weave owns its commands, transitions, revisions and results. An orchestration service owns its own admission and execution history; the two link exact occurrences without copying either history. Each service owns a different fact about the same occurrence; neither keeps a competing causal graph.

## Every command ends in one of four outcomes

| Outcome | What happened | What the caller does next |
| --- | --- | --- |
| `Rejected(Refusal)` | A required condition failed and the requested change did not happen | Supply the missing material or permission, or resolve the other failed check, then send a new command |
| `Conflict(Conflict)` | A revision or dependency changed | Read the current state and reconsider the request |
| `Committed(CommitReceipt)` | The named transaction committed and read-back confirmed it | Use the recorded result and new revisions |
| `CommitUnknown(ReconciliationHandle)` | The caller cannot yet establish whether the command committed | Look up the outcome using the original command identity before retrying |

Every interface preserves all four outcomes. It must not reduce them to a boolean or a generic success-or-error result: a refusal, a conflict and an unknown commit require different next actions.

Replay identity pairs a selector, made of the destination instance, acting identity, operation and a command key the caller supplies, with a digest of the command that must match. If a reply goes missing, use that identity to recover the result. Every outcome is final for its key: repeating the same request returns the original outcome, a refusal included, without deciding again or accepting twice. Trying again after a refusal, or editing a command, needs a new key. [The store](store.md#recover-the-original-command) gives the full rule.

A fault inside Weave before anything is saved produces no outcome and leaves the key unused; [the store](store.md#recover-the-original-command) explains why this is not a fifth outcome.

> [!IMPORTANT]
>
> **A `CommitReceipt` proves only that the named Weave transaction committed and independent read-back confirmed it.** It does not prove message delivery, an outside effect, another party's acceptance, goal satisfaction or Pale settlement. Each needs evidence from the system or person responsible for it. A client that decodes a `Committed` outcome from a reply holds a `CommitReport`: a report of the receipt that it confirms by reading back, never the receipt itself.

The implementation must keep `CommitReceipt` fields private and provide no public constructor, deserialisation, default value or conversion from another type. Only the module that runs the commit and its independent read-back may create one. Persisted receipt data and decoded reports cannot construct that proof type. These are requirements for the runtime still to be built.

Return to the [investigation review](https://rawthoughts.in/weave/from-request-to-accepted-work/#keep-the-result-and-the-remaining-responsibility-visible) for the practical difference between submission, acceptance and the customer's outcome.

## Protect the decision through commit

Accepting work can depend on the reviewer's current authority, an evidence relation and the absence of a blocker. Checking only the work item's revision will miss a blocker added after validation.

Every outside fact the decision needs is gathered before the transaction begins and passed in as a value. The transaction handle must reach only Weave's store, and the types must enforce it: code that calls a provider, another instance or any other outside service from inside a decision must not compile. A slow or failing outside service then never holds a transaction open.

The application must protect every record and query result that affects the decision until commit. This **read set** includes policies, relationships and checks for absence. A **read frontier** identifies the captured state against which a dependency was checked. If someone adds a blocker, the command must conflict even when no previously read row changed.

The locking and isolation mechanism still needs implementation and testing. A transactional database alone does not establish that these checks work.

Within the transaction, record changes, history, commit receipt data and secret-free outbox commit together. A later read confirms the named commit. Queues and wake notifications prompt work or recovery; the database remains the source of truth.

An outside effect or a change in another instance happens separately and needs reconciliation. A larger activity is a resumable sequence of commands, each with its own outcome. Clients cannot submit arbitrary transaction scripts that bypass these checks.

## Three kinds of change have separate writers

| Change | Who writes it | Effect |
| --- | --- | --- |
| Ordinary data | Authorised people or software callers | Changes comments, assessments and work through guarded commands; nothing is recompiled |
| Domain definitions | The domain owner | Installs a new immutable version; existing records change version only through explicit adoption |
| Managed constitution | The organisation's owner | Supplies authorised roles, grants and bindings through a checked, staged change |

A record can contain fields managed by different writers, as long as their field sets do not overlap. A domain definition cannot create authority, and a work update cannot overwrite the organisation's constitution. Optional Pale admission still leaves Weave's local checks in place.

A managed change proceeds through capture, compilation, materialisation and confirmation by read-back before activation. Partial materialisation is not active configuration. Revocation stops new use under the applicable rules while earlier evidence retains its attribution. There is no second Weave workspace or desired-state database, and no shared editable blob spanning the three writers.

Continue to [Extending Weave](./extending.md) for declaring these rules, or return to [Weave](./index.md) for the application layers.
