# Effects, schedules and artifacts

A Weave transaction records an intent to change something outside the database. A separate provider performs the action, and Weave reconciles the provider's result. A database commit does not prove that mail arrived, a file was stored or another party accepted the work.

## Resolve a capability before selecting a provider

`ResolveCapability(actor, capability, target, context)` returns a provider binding or an explicit denied, ambiguous, unavailable or stale result. The input includes the exact target, operation context, policy version, purpose and disclosure limits, and required freshness. The same mechanism supports messaging, publication, object storage, repository reads, meetings, code execution, models and agents without product-specific branches in the engine.

A provider binding grants no permission beyond what the actor and operation already have. The deployment supplies eligible providers, policy and bindings; an optional Pale workspace may compile those selections. Replacing a provider changes configuration, not the command rules.

Concrete providers live in separately deployable integration packages using the public SDK. They hold credentials and make native calls. Weave keeps secret references and limited result records, and owns capability resolution, effect intents, leases, reconciliation and references to settlement. Mail, calendar and outbound delivery outcomes remain with their providers.

A commit, an effect intent and a delivery attempt have independent states. Lost or duplicated jobs and provider outages cannot lose the original comment or create duplicate work. Fan-out, causal chains and recovery loops have explicit limits.

The first provider is a separately deployed deterministic local test provider with an inspectable sink. It must inject rejection, duplication, lost delivery, unknown outcomes and read-back cases. It demonstrates the contract without pretending to send real mail or requiring live vendor accounts. Live mail and calendars are not first-version prerequisites.

## A recurrence creates a separate work item

A durable Schedule record produces a distinct WorkItem for every occurrence, with its own assignment, outcome, attempts and history. Each item links the schedule and policy revision and, where declared, the previous occurrence. One-shot scheduling and a paused service level are different cases.

When an organisation runs agents through Pale, scheduled work that comes due reaches an executor along Weave, then Pale, then execution. Weave owns the schedule, the work record and its own checks; Pale admits the operation and records how the work was carried out, as the [occurrence contract](../contracts/pale-occurrences.md) describes. A chat runtime or any other executor cannot start scheduled work around that path.

The worker uses narrow store, clock and wake ports. The store retains schedules, commands and causal history. On PostgreSQL, `LISTEN/NOTIFY` supplies a wake hint; on either database a recovery timer and a bounded leased scan back the hint, so a missed notification cannot lose work that is due. This uses the existing database and a Weave worker rather than adding another scheduling system. Cron or BullMQ could later supply wake adapters only.

Each schedule declares a recovery window, backlog limit and missed-run policy:

| Policy             | Required result                                         |
| ------------------ | ------------------------------------------------------- |
| `catch_up`         | Create each missed occurrence within the declared limit |
| `coalesce`         | Create one occurrence identifying those it covers       |
| `skip`             | Record which occurrences were skipped                   |
| `pause_for_review` | Hold the schedule for a decision                        |

Due-time and cancellation races must use captured clock inputs, command identity, expected revisions and native read-back. Admitting an occurrence is a command whose key comes from the schedule, its revision and the occurrence's nominal time, so a second wake replays the first answer. A wake from another instance uses that instance's occurrence identity in the same way. If Weave faults before the occurrence is saved, the key stays unused, and a later wake within the recovery window can admit the occurrence under the same key. A refused occurrence is recorded with its disposition; trying it again takes a new attempt number in the key and references the refusal. Work due more than a year away, such as a booking made far ahead, starts its eligibility later; it is not a service level falsely marked paused.

## Watch deadlines without blocking work

Time spent in a state is a declared rule. Its policy revision supplies the due interval, warning threshold, escalation and conditions. An overrun record identifies the affected work, the applied rule, when it became due, when it was detected and how it was resolved. Repeated scans must not create duplicate overruns; leaving the state resolves the active overrun while retaining its history.

Monitoring and notification failures cannot block a valid transition. They remain visible and recoverable through the same outbox and read-back rules as other effects. Watching can reveal missed obligations or lost value; it cannot guarantee that neither occurs. Any rule that actually prevents a transition belongs among that command's declared guards and authority checks.

Tunable values use [policy revisions](extending.md#policy-revisions-record-what-actually-ran), independently of the domain's definition version. A rule set may itself be evaluated in an experiment. History must name the rule revision and experiment arm actually used, including a fallback, so later comparisons describe what happened.

## Keep each outcome with its decision-maker

A checklist attestation does not prove an audit or outside effect. Completion, disposition, native effect, provider acceptance, consumer acceptance, business acceptance and settlement by an optional orchestrator remain separately attributable. Each can be pending, rejected, deferred, failed, partly settled, conflicting, unknown or a no-op. A duplicate or cancellation is not success. A goal needs its own previously agreed success conditions and independent verification.

## Files are managed content or external references

| Kind | What Weave holds |
| --- | --- |
| Managed artifact | Content through its selected object-store adapter; digest, media type, size, policy, provider coordinate, custody state and result in the store |
| External reference | Owner, instance, object, revision, digest, locator, freshness, use limits and revocation details; the source retains its content and lifecycle |

Capturing an external object creates a new managed snapshot linked to the source. It does not take over the source's rights. Managed content is immutable per revision. The store and object storage share no transaction, so staging, provider finalisation, metadata commit, ambiguous outcomes, orphan cleanup, missing bytes, deletion and recovery need explicit states and independent checks.

Weave owns recovery and provider read-back for managed content. Historical locators must resolve the right revision, and consumption verifies its exact bytes. If revisions share a body, recovery must account for every referring revision. For an external reference, the source owner supplies recovery and evidence under its own retention policy.

The first conformance adapter is a confined local filesystem. It operates from an already-open root, rejects traversal, symlinks and special files, verifies digests, finalises durably and reads back independently. S3-compatible storage is required before claiming hosted support. Its tests cover conditional writes, checksums, multipart upload, versioning, read-back, namespace isolation and deletion. An S3 ETag is not Weave's content digest.

Storage, repositories, document services and transport are different provider families. A file write is not a Git commit, a Git commit is not GitHub publication, a Drive export is not the source document, and a local iCloud write does not prove synchronisation. A transport moves bytes between stores without owning them. Which carrier joins Weave instances, iroh among the candidates, is still being [compared](instances.md#sameness-is-a-claim-never-a-merge); a transport an organisation reaches through Pale stays Pale's.

## Erasure removes recoverable content

An authorised erasure deletes or crypto-shreds reconstructable content under Weave's control: current and historical fields, relation properties, managed content, search surfaces, hot views, caches, exports, any future engine-owned embeddings, and eligible backup copies under the retention process. Retain only the minimum non-identifying tombstone, permission reference and erasure result. Historical reads report `erased`.

Commit records and outbox payloads must already minimise content. Inspect surviving metadata for disclosure as well as reconstruction: a digest of predictable input can still reveal that input. If removing inputs makes replay impossible, state that limit explicitly rather than claiming the result can still be reproduced. Test all copies and recovery paths against the controller’s retention and legal-hold rules so a retained diagnostic or backup cannot silently reconstruct erased data. Other owners confirm erasure of their own content separately.

See [store recovery](store.md), [instance isolation](instances.md) and the planned [artifact and privacy cases](../tests/privacy/README.md).
