# Acceptance journeys

These files specify the behaviours an implementation must prove. They are not executable tests or evidence that Weave's runtime works. Each case binds actors, starting record revisions, policy and definition versions, the public command/query sequence, expected native commit and independent read-back, emitted occurrences and who may accept the final state. Include failure, competing-order and recovery branches. Fixtures and wire formats are still implementation work.

Qualify standalone, hosted-instance and optional-integration claims separately for an exact release. Standalone cases use the local conformance provider and require no live external account. Add the selected integration's cases before claiming support for it.

The journeys cover every path a person or product can take through Weave on its own, every path that joins Weave to another product as [the guide](../docs/index.md) lists them, and an adopter's workspace. Each joined path is tested with Weave standalone, through Pale, or both together, wherever it supports them. Each joined journey is listed here once, with the suites it combines; each other product tests its own half.

| Journey | Owning suite |
| --- | --- |
| First vertical: a steward qualifies an account, the next action becomes work and the entitled reviewer accepts it | [Standalone](standalone/README.md) and the commercial-work domain |
| Human-only commercial work | [End to end](#human-only-commercial-work) and the commercial-work domain |
| An adopter's configured organisation | The adopter workspace, against a released Weave; private topology is not an engine fixture |
| Concurrent person and agent edits | [Concurrency](#concurrent-commands) |
| One orchestrator and several executor kinds | [Operation roles](operation-algebra/README.md) and [lineage](#causal-lineage) |
| Direct context without Scope | [Pale integration](pale/README.md) |
| Ordinary data, definitions and managed rules | [Separate writers](definition-lifecycle/README.md) |
| Approval and comment publication in chat | [Buzz integration](../integrations/buzz/tests/README.md) |
| Two instances and a hosted adopter | [Cross-instance](cross-instance/README.md); Gazette owns its four caller/input combinations |
| Complete standalone service | [Standalone](standalone/README.md) |
| Definition evolution | [Schema evolution](#definition-evolution) |
| No information leaks | [Privacy](privacy/README.md) and [security](security/README.md) |
| Hosted isolation | [Security](security/README.md) and the host's own tests |
| Occurrence delivery between instances, over the provisional HTTPS carrier | [Cross-instance](cross-instance/README.md) and [outbox](#atomic-occurrence-delivery) |
| Artifact recovery and erasure | [Privacy](privacy/README.md) and the object-storage adapters' tests |
| A publisher reads a permitted Weave fact, or gets an explicit unavailable result | [Contracts](#contracts); Gazette tests its own caller and input combinations |
| Pale requests a change and recovers a lost reply | [Pale integration](pale/README.md) |
| Weave publishes committed changes for Pale to check | [Pale integration](pale/README.md) and [outbox](#atomic-occurrence-delivery) |
| Context, judgment or evaluation reaching Weave work only through Pale | Pale's tests; Weave's evidence references and parent checks in [Pale integration](pale/README.md) |
| Scope reads Weave without Pale, and Weave calls neither Scope nor a model | Scope's own tests of its read through Weave's port, and [standalone](standalone/README.md) for Weave without Pale |
| Engineering work handed off through Pale and returned for a parent decision | [Pale integration](pale/README.md) and [lineage](#causal-lineage); Pale tests the dispatch |
| Scheduled work that runs while its owner is away, through Pale to execution | [Scheduling](scheduling/README.md) and [Pale integration](pale/README.md) |
| An accepted finding becoming work only when someone takes responsibility | [Radar branches](../domains/commercial-work/tests/README.md) |
| A chat key acting through Weave's own binding, the Pale-controlled profile, or both | [Buzz integration](../integrations/buzz/tests/README.md) and [security](security/README.md); no suite yet for the combined profile |
| The second vertical, finance written through an adopter's workspace | No suite yet; specified with that vertical |

[Radar failure branches](../domains/commercial-work/tests/README.md) extend the standalone journey. Pale's Reckon integration owns evaluator-version comparison; Weave must retain its exact evidence references and wait for a separately permitted owner ruling. Dispatch, Scope admission and execution replacement belong to Pale's tests, while [Weave's parent checks](pale/README.md#a-child-result-leaves-a-separate-parent-decision) preserve independent acceptance and recovery.

## Suites of a paragraph

A suite with its own folder carries cases long enough to need one. These are the suites whose cases fit a paragraph, kept beside the requirements they apply.

### Integration

Select and qualify each optional provider independently. Removing it must preserve source records and unknown outcomes. A delivered message cannot establish command acceptance, outside effect or consumer settlement.

### Causal lineage

Exercise the actor/subject and engine-appended lineage cases in [operation roles](operation-algebra/README.md), and result-to-parent matching in [Pale integration](pale/README.md). Caller parents cannot overwrite engine history.

### Conformance

Compare the same operation through every enabled surface. Preserve variants, payloads, errors, revisions, actual actor and unknown outcomes. A protocol that cannot express a capability reports it unsupported rather than inventing parity.

### Presentation

Exercise fresh, stale, denied and unknown views; mandatory relations; several related objects without an invented primary; nested/reverse reads, counts, search and artifacts. A renderer records submitted intent, never a native outcome by itself.

### Read back

Confirm exact command, occurrence, source version and provider coordinates independently. Substitute another genuine object or digest and require refusal or unresolved state. A later revision must not overwrite the result recovered for an earlier command.

### Portability

Build from a clean checkout on Linux and macOS using the declared toolchain and lockfile. Run the repository architecture and source checks on each. A platform build proves only that the crates and entry points build until cases for Weave's runtime exist.

### Concurrent commands

**Concurrent edits by a person and an agent.** A changed task and an added relationship invalidate a stale command; the actor must reread and propose again, nothing is silently overwritten, and the refusal names the reason.

### Capability resolution

Prove exact actor, capability, target, context, policy, purpose and freshness resolution. Cover denied, ambiguous, unavailable and stale bindings; selecting a provider grants no permission. Use the separately deployed deterministic local provider before any live vendor.

### Atomic occurrence delivery

Exercise the [HTTPS delivery cases](cross-instance/README.md) and [lost command reply](pale/README.md#reconcile-a-lost-command-reply). Prove each committed change and its occurrence appear together, including deletion, cancellation and acceptance; delivery failure cannot roll back the source commit.

### Definition evolution

**Definition evolution.** Versions one and two coexist; new records take the active version two; an old record stays on version one until a guarded adoption; an incompatible GraphQL change gets its own type coordinate; removing a version blocks new use while historical evaluations still render.

### Contracts

Generate language forms from the producing declaration and run positive and negative consumer cases across Rust, JavaScript, Python and Go. Preserve operation, definition, capability, outcome, lineage, presentation, store, interface, cross-instance and Pale contracts without a second handwritten model. Version and signing profiles remain implementation work.

### Store

Run against each supported database, PostgreSQL first because the first vertical will prove it: atomic records/history/commit/outbox, record and relation conflicts, absent blockers and policy changes before commit, crash windows and unknown results. Rebuild derived indexes without changing authoritative outcomes. Backup and restore the complete recovery unit before storing valuable data.

### Human-only commercial work

**A human-only commercial journey.** An account, an opportunity and a case, booking-like scheduled work, versioned evidence, service levels and checklists, changes to child records, a cancellation race, an exact outside audit or effect receipt and a distinct final outcome. Work scheduled more than a year out starts its eligibility later, not as a fake paused service level.

## Performance cases

These workloads remain to be implemented against real behaviour. A build of the near-empty crates supplies no throughput, memory or latency result.

| Workload | Measure |
| --- | --- |
| Store | Guarded commit, concurrent read-set conflicts, outbox and recovery |
| Operations | Representative multi-record changes and bounded derivation |
| Graph | Dense adjacency, parallel and n-ary relations, cycles and generation fencing |
| Query | Bounded traversal, history, authorised filtering and deterministic pagination |
| Hot projection | Memory density, stale-handle rejection and rebuild cost under memory pressure |

Choose representative records and ownership limits before fixing the physical layout. Report the exact implementation, data, configuration and machine for each result. Correctness cases stay in the suites above.
