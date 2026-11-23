# Interfaces, sign-in and access

Weave is designed as a headless backend. People and software use the same commands through REST, GraphQL, SDKs, a command line and the supported agent protocols. There is no browser application in the first version: Weave's runtime supplies the rules, descriptors and [components](#components-look-the-same-on-every-surface) that independently built clients render.

## Choose a surface by the job

| Surface | Intended use |
| --- | --- |
| REST and JSON | Semantic commands; identity and history; definition and package lifecycle; artifacts; event recovery and reconciliation |
| GraphQL | Rich reads generated from installed definitions, through the shared [query model](queries.md) |
| Rust SDK | In-process access; re-exports the domain authoring builders |
| TypeScript, Python and Go SDKs | Typed remote clients over REST and GraphQL, generated or checked against public contracts |
| Command line | Authoring, operation, diagnostics and recovery; equivalent journeys must work with ordinary HTTP clients |
| Signed webhooks | References that wake a consumer to read the committed occurrence |
| MCP, ACP and A2A | The subset each protocol can represent faithfully, enabled by the deployment profile |

Remote SDKs never open the database or embed another runtime. The lifecycle manager, `weave-host`, has its own REST and SDK surface, separate from the instance service. A later browser, mobile or chat client uses these public contracts with no private write path and no ontology of its own.

REST, GraphQL and the official SDKs together cover the instance service. An authorised agent using REST can use its full surface, because an agent is an acting record like any other and not a lesser actor. Each route preserves actor identity, typed input, purpose, expected revisions, causal parents, all four command outcomes, uncertainty and recovery. Routes exposing the same operation produce the same canonical command and native outcome.

A protocol cannot claim capabilities it cannot express. Keep unsupported, disabled by profile, concealed, unauthorised, invalid, conflict, committed and commit-unknown distinguishable. An A2A task is not automatically a Weave work item; cancelling an ACP request cannot roll back an effect. Importing an A2A artifact requires an explicit Weave command. No interface adds or drops a domain guard. If an adapter cannot preserve a required field or outcome, exclude that operation from the adapter or define a distinct operation with its own approved contract.

## Components look the same on every surface

A component is a unit of interface that Weave serves and a host surface renders: a card, a panel, a list or a review prompt. The word micro-frontend means only this. Weave supplies the typed command, query and presentation descriptors and the access-filtered record views that let a host's cards materialise correctly; the host, whether a chat room, a terminal, a web page, a note in a knowledge base or an agent's conversation, owns rendering and interaction. Components are not a separate browser application.

Each component is a window onto a record, never a copy of it. Opening it reads the current state with the reader's permitted credentials. A view distinguishes fresh, stale, denied and unknown state, keeps required relations visible, and applies the same checks to nested and reverse reads, counts, search and artifacts as any other query. A stale component says so and cannot authorise a change.

The same records and checks apply on every surface. A component in a chat room and the same component in a terminal show the same record under the same access rules, and a command sent from either produces the same canonical command and the same four outcomes. A panel can show several exact related records without inventing a primary account for convenience. Cards and layouts are views, not new record types or editable copies of truth. A renderer can attest that an authenticated actor submitted an intent; the application decides whether it commits. [Buzz integration](../integrations/buzz/README.md) applies this contract to chat cards.

## Four sign-in profiles feed one actor model

| Profile | How the caller is identified |
| --- | --- |
| Standalone | OIDC or OAuth2 for people; signed credentials for workloads |
| Buzz identity | Weave verifies the Nostr proof and resolves an accepted, revocable key-to-actor binding |
| Pale-controlled | An explicitly enabled profile accepts Pale's signed, bounded statement admitting the operation |
| Authentication off | No authentication gate; the caller still supplies its durable acting identity |

Authentication off is a convenience for a selected local or trusted setup. It changes no operation typing, access rule, guard, revision check, history, commit record or effect rule. Weave records the active profile. There is no anonymous record model or separate test execution path.

## Access is decided inside Weave

The typed policy evaluates principal, actor, action, resource, context, scopes and purpose against exact policy and definition versions and the captured read frontier. Scoped role occupancy is a first-class way to write, administer and explain that policy; it compiles into the same model rather than creating another permission store.

An embedded evaluator, preflight tool, simulation or policy projection may assist the check. It must bind the exact policy and definition digests and read frontier. Cedar, OpenFGA or OPA are possible evaluator styles, not selected owners of permission. No outside evaluator may use fresher facts than Weave or widen its rules. An asynchronous answer cannot allow a write when Weave cannot confirm the permission at commit.

Admitting a record into the graph decides what the graph may contain. It does not let every reader see every record. Access limits apply separately to records, fields and relations, for each acting person, agent or application.

The engine enforces controller, scope, purpose, disclosure, retention and model-use limits supplied by their owners. Domains define the vocabulary and policy. An orchestration service, data-loss-prevention system or legal owner can supply evidence or narrow those limits; none can widen them through an adapter. A domain and operation name the deciding person or quorum independently of seniority, family or organisational shape.

Membership is a relationship, not access. Access belongs to an identified actor within the permitted scope. A shared account or login never substitutes for the actual actor.

## Revocation freezes uncertain work

Revocation immediately blocks new starts. Open or uncertain work is frozen for attention: an unconfirmed effect is not retried, and responsibility does not silently fall to someone else. A permitted transition must reassign, cancel or reconcile it. Completed work retains its attribution and evidence. A revoked caller can still [replay an earlier command](store.md#recover-the-original-command) to learn its outcome, without seeing any content its access no longer covers.

See [command recovery](store.md) for commit checks and [queries](queries.md) for access checks that cover filtering, traversal and information revealed by a result.
