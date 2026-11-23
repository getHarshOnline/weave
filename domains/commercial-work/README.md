# Commercial work

This first-party domain is the planned example of Weave's public extension model: relationships and commercial commitments alongside accountable work for people and agents. It must use only the public SDK, install into a clean instance and be removable without rebuilding the engine. The engine must also build and pass its own tests with no domain installed. These are design requirements; this package has no implemented business behaviour yet.

The initial definitions are Account, Person, RelationshipContext, Evidence, Radar, Goal, WorkItem, Comment, ArtifactRef and Schedule. They are ordinary installed definitions, not root engine classes or any adopter's private ontology. A second unrelated domain must exercise the same SDK before the product claims to serve a clean adopter. Neither domain gets private imports, SQL migrations, GraphQL resolvers or a client-name branch.

The [first vertical](../../docs/index.md#the-first-complete-service) uses only the part of this model it needs, and the [standalone journey](../../tests/standalone/README.md) specifies it.

## Families and their operations

| Family | Relations and lifecycle | Operations and policy points |
| --- | --- | --- |
| Identity, actor capability and bindings | Many record types, source identities and participations, effective intervals | Register, bind, suspend, revoke; explicit issuer, actual acting record and role constraints |
| Accounts, organisations and opportunities | Many relationship contexts, stakeholders and mandates | Delegate fields, progress, qualify, propose, record outcome |
| Relationship, participation and stewardship | Independently identified many-to-many edges with no permanent primary | Attach, end, supersede, under role and scope guards |
| Research, claims, evidence and assessment | Many claims and evidence items; conflicting and superseded assessments kept | Investigate, attach, assess, review, supersede |
| Radar | Observation, proposal, candidate, review, disposition and outcome references | Admit, investigate, relate, dispose; never automatic work or publication ([Radar records](radar.md)) |
| Goal, project and work item | Isolated or contextual; structural parent distinct from dependencies; goal satisfaction a separately versioned outcome | Create, assign, start, submit, review, rework, resolve; satisfy a goal only against success conditions agreed in advance and verified independently |
| Assignment and accountability | Role and actor participation over time; zero to many attempts | Assign, transfer, end; fence active attempts where required |
| Work budget and approval | Native reservation, usage and approval records under named authority | Reserve, release, account, reconcile, with revision and attempt fences |
| Work relation | Dependencies, blockers, parent, escalation and any typed edge | Add, end, revise, under per-type cardinality and cycle rules |
| Work comment | Threaded replies; the body lives once | Add, reply, amend, redact; a comment's parent is not the work's parent, and a comment never moves work on |
| Commercial interaction | Admitted email, call and meeting chronology with its surrounding relations | Add, amend, redact; raises a room only at a named frontier; a committed next action becomes work only by an explicit act |
| Schedule and recurring series | Distinct logical occurrences; one-shot scheduling also valid | Enable, revise prospectively, suspend, materialise what is due |
| Checklist and service level | Instantiated requirements, intervals, calendars and reasons | Attest, evaluate, pause, resume, with policy and actor history |
| Attempt, effect and result | Zero to many execution and native-result references | Request, record, reconcile uncertain outcomes |
| Review, acceptance and disposition | Provider, consumer and business decisions, each attributable separately | Accept, reject, rework, resolve; duplicate or cancelled is never success |
| Commit, outbox, consequence and delivery intent | Commit-bound identity and independently retried delivery | Atomic admission and commit, idempotent dispatch and reconciliation |

Generic work may be isolated, contextual, parented, related, dependent or scheduled. It requires no operational case, permanent agent, mandatory parent, permanent primary relationship or Pale operation. Definitions keep structural parentage, dependencies, comment ancestry, responsibility, permission and execution separate.

## Relationships carry their period and purpose

Accounts, people, opportunities and relationship contexts are separate records. One person can participate in several contexts without duplication. No relation is permanently primary, and appearing in the record grants an outside person no internal role, software account or chat membership.

An account manager holds relationship continuity. An account executive carries a particular opportunity, mandate or commercial promise. A manager may steward several accounts; an account can have several mandates and stewards; a mandate can involve several delivery pods. Titles do not grant assignment, spending, effect or acceptance permission. Each delivery-pod occurrence binds its exact account, mandate, manager, executive, provider-acceptance point and consumer-acceptance point. Pod-type names remain adopter configuration.

Stakeholders have declared roles such as buyer, sponsor, technical owner or operating owner. Their labels alone grant no permission. Stewardship, next actions and review queues record the responsible owner, due date or freshness, and blocked, deferred, rejected or superseded state. Outreach, response, discovery, proposal and outcome are separate stages with recoverable histories.

Optional organisational structures include verticals, groups, discipline trees, pods and roles, with primary, leader or supporting coordination relations. A structural definition describes relationships and required evidence, never a mandatory title. A stable role binds its occupant separately from the software provider.

## An assessment preserves its reasoning

Account and opportunity assessments retain qualification checks and their results, disqualifiers, counterevidence, triggers, working hypotheses, confidence, freshness, policy version, assessor and reviewer's decision. Eligibility comes before scoring: a disqualified record is not scored. Later assessments supersede earlier ones while retaining why the judgment changed.

Weekly review and calibration exports are read-only views. They preserve the unit of analysis, coverage, stale or unknown evidence, queue movement, outcomes and governing policy versions. They are not a second editable store. Fit criteria, weights, source hierarchies and outreach/review policies are independently versioned normative material; assessments name the version they used. Reusable reconciliation and export procedures have their own homes too.

Email, call, meeting and research content can be kept with its source, freshness and permitted surrounding relationships. Sending mail and other outside effects stay with their providers. A proposed next action remains commercial context until an explicit command creates work. Raise a chat conversation at a named point where people need to coordinate or decide, not for every event.

## Work starts deliberately and ends with acceptance

A command with unambiguous permission, scope and destination turns conversation into work. Otherwise retain a proposal, backlog entry or question for its owner. One item represents one independently assignable and acceptable responsibility. Work nests to any depth: a step that can be assigned and accepted on its own is a child work item, while a condition its parent must meet before closing is a checklist requirement attested on the parent. Reuse an existing item only when the match is exact and unique; a similar title or linked thread is insufficient. A correction identifies the exact item when several are linked.

The actor who created the work is its creator. Keep the original speaker and message as source references, not as the creator or a delegation of permission. The record holds a short responsibility, acceptance conditions and exact conversation references rather than a copied transcript. A comment and a status change are separate commands. Each change records the actual actor, represented party or role, and work item or attempt.

Unstarted work can move to a verified successor or be revised. Active work must be fenced and explicitly transferred and reconciled. Completed work keeps its original actor and acceptance; neither it nor an outside committed effect is rewound. A policy permitting one active attempt requires concurrency-safe enforcement. Work reservations and usage grant no spending permission and replace no payment record.

Each recurring occurrence is a separate item linked to its Schedule, with its own assignee, outcome and history. Cross-team work stays scoped to its topic and creates no overall parent, council or transferred permission. Each party accepts its own responsibility independently.

Goals trace to a named person's decision and conditions agreed before work begins. Agents can propose and act within that authority, while the record preserves both the deciding person and acting agent. Goal satisfaction requires independent verification of those conditions; finishing its tasks is insufficient. Adding an agent to an organisation is a separate constitutional act, not a side effect of creating work or a contact.

See [domain extension](../../docs/extending.md), [effects and scheduling](../../docs/effects.md), [Radar](radar.md) and the planned [human-only journey](../../tests/README.md#human-only-commercial-work). Tests must exercise the domain through the public SDK and interfaces.
