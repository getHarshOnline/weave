# Instances

Each organisation would run its own Weave instance, with a separate database, credentials, storage and keys. Organisations could exchange work while keeping their own decisions about what to take on and what to accept. This is the deployment design; Weave's runtime, host applications and exchange protocol are not implemented.

## One instance per tenant

An instance could serve one person, a household or a company's members. The same design serves an organisation running its own instance and a shared hosted service: in both, every tenant is its own instance. Shared hosting still gives each tenant a separate server process or container, PostgreSQL database, application role, connection pool, storage and key namespaces. Definitions, recovery and the position reached in its occurrence history belong to that instance too.

A SQLite instance is its own database file, and several PostgreSQL databases may share a cluster. They do not share business tables or cross-tenant foreign keys. Operating a database per instance costs more, but lets a host back up or restore the client and provider separately. Each instance’s record points to the other with an external reference. A relation between two tenants on one cluster uses the same signed external reference and [correspondence claim](#sameness-is-a-claim-never-a-merge) as two instances hosted apart.

| Application | Responsibility |
| --- | --- |
| `weave-server` | One instance's records, commands, queries, schedules and outbox |
| `weave-host` | Provisioning, upgrades, backup, restore and retirement of one or more instances |

The host manages deployment lifecycle without reading business records. Its API has a separate listener or hostname, authentication audience, access checks, logs and rate limits. Tenant routing follows an authenticated binding; a request-body tenant field cannot select another instance.

Its records cover instance identity and incarnation, desired and observed runtime, deployment/database/storage/key/network references, health, readiness, migrations, backup, restore and decommissioning. Docker, Kubernetes, systemd, Terraform, OpenTofu or a managed platform can supply deployment adapters. The host owns the portable lifecycle and its result, not those platforms' implementations. Operating a deployment grants no permission to inspect or change its business records.

A running instance must remain available when its lifecycle manager or Pale is unavailable. Removing it from configuration leaves it in place. Only a caller allowed to retire the instance can start the retirement process, which includes recovery and decommissioning.

An optional controller may declare instances, release versions, bindings, sign-in profiles, domain packages, retention and desired lifecycle. Compiling those declarations has no effects. A separately permitted reconciliation makes the host attempt the change, read the provider back and record its own result. A standalone adopter reaches the same contracts through its controller, CLI or platform tools. Shared infrastructure never merges graphs, permissions, transactions, controllers or data ownership.

## Work moves between instances by offer and acceptance

A client asks a provider to investigate a failed export. The provider may refuse, negotiate or take responsibility for it. The client's request and the provider's commitment are separate local records. Reviewing the provider's trace within the provider's own instance cannot accept the result for the client.

The exchange below uses HTTPS for now. The carrier stays provisional until the comparison under [Sameness is a claim, never a merge](#sameness-is-a-claim-never-a-merge) is finished:

### Commit the offer at its origin

The origin commits the change and its outgoing occurrence in its own database transaction.

### Notify and read back

A signed notification carries the occurrence reference. The destination detects duplicates and retrieves only the source state it is permitted to read.

### Decide locally

The destination forms a proposal and decides whether to accept it under its own rules and authority.

### Record the destination's responsibility

A local command commits the resulting responsibility and issues the destination's own receipt. Any later result or acceptance follows its own command.

Neither instance writes the other's database or closes the other's work. The service contract describes the request, result, rights, evidence and settlement terms. Provider, consumer and any parent acceptance stay separate.

The directional contract connects independent responsibilities; it creates no parent relationship between the organisations and implies no reverse agreement. The work references both parties' accountability without copying their internal records. It binds an immutable directional service edge, governing instrument and schedule, independent provider and consumer work, handoff and expected revisions, budget boundary, cancellation, termination, dispute, compensation and reopening rules, separate provider/consumer/parent acceptance, and typed references to foreign effects and settlement. Live service work waits until the selected contract's replay and recovery have been proved.

TLS, caller authentication and an occurrence signature answer different questions. Occurrence identity includes the origin instance, origin incarnation or producer epoch, immutable occurrence ID and canonical content digest. A retry keeps that identity. Reusing it with changed bytes must fail, and missed occurrences must be recoverable. An HTTP retry key cannot replace it.

If a notification is lost, the destination would recover the missing occurrence at its origin before deciding whether to accept it. The wire format, proof of the sender’s identity and recovery behaviour still need design and testing. There is no endpoint or deployment recipe to follow yet. For what each participant owes, return to [the relationship example](https://rawthoughts.in/weave/relationships-and-mandates/#agreeing-the-next-piece-of-work).

## Sameness is a claim, never a merge

The client and provider may each hold a record for the same person. Matching names, emails or keys are not enough to establish that connection or permit access. A revisioned correspondence claim records how far the connection has been agreed:

| Strength            | Meaning                             |
| ------------------- | ----------------------------------- |
| `candidate`         | Someone proposed the correspondence |
| `locally_accepted`  | This instance accepted it           |
| `mutually_attested` | Both instances attested it          |

Each query or operation declares the strength it needs. Even mutually attested records keep their local identity, revisions, rights and retention.

Correspondence between people is separate from identifying a duplicate message or equal file content. Each claim needs its own evidence and has different consequences for deduplication.

The carrier between instances, the exact attestation format and the trust evidence remain design work. Before choosing them, compare CRDTs, torrent content identity, rclone's synchronisation model and iroh under these constraints: local permission and revisions, three correspondence strengths, changed-content rejection, replay, revocation and independently owned retention. A mechanism that establishes equal bytes does not by itself establish the same person or permit a merge.

## Weave stands on its own

Weave is designed for direct human and software access without chat or agent orchestration. OIDC or OAuth2 would authenticate people; signed credentials would authenticate workloads. Optional profiles would admit Buzz/Nostr identity bindings or Pale-signed admission.

An explicit no-auth profile would remove the authentication gate. It would still require an acting identity, access checks, guards, revisions and history.

Buzz conversation and Pale federation can carry references without gaining authority over Weave's records. Room or network membership does not admit work. Removing an integration must preserve records and leave uncertain results available for reconciliation.

HTTPS is the provisional first exchange carrier until that comparison is finished. Pale's planned federation between people uses Nostr through the owner's relay; transport between one person's machines remains to be chosen. Buzz uses Nostr for conversation under its own identity and access rules. The signed inner occurrence keeps its identity and owner even if another selected carrier transports it; standalone Weave requires neither Pale nor Buzz.

Continue to [command recovery](./concepts.md#every-command-ends-in-one-of-four-outcomes), or return to [Weave](./index.md).
