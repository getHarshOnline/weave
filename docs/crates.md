# The crates

Weave is a Rust workspace of layers, with dependencies pointing toward the core ([how a command runs through them](index.md#follow-a-command-through-the-runtime)). Six workspace members exist today and each holds a module comment that states its place; none yet holds behaviour. Every other crate arrives with its first real operation and test, and is added to the dependency graph in [the architecture test](../tests/architecture/main.rs) in the same change. This page keeps what each planned crate is for, what it builds on and the rule it must keep, so the rule returns with the crate.

## The crates that exist

| Crate | What it is for | Builds on |
| --- | --- | --- |
| `weave-core` | Shared meaning: records, relationships, definitions, operations and transitions; identity, capabilities, permission, time, source, revisions and where things came from; and the pure decision over an installed operation with its plain results (the plan, `Refusal`, `Conflict`, the reconciliation handle). Business families and infrastructure stay out. | Nothing |
| `weave-authoring` | The builders with which a domain defines its world as inert, digest-bound data. A domain built here reaches records, definitions and operations, never the ports or stores that run them. | `weave-core` |
| `weave-ports` | The narrow contracts between decisions and infrastructure: storage, object storage, cache, wake, queue, search, clock, identity, secrets, observability, effects and event publication. Concrete providers stay outside. | `weave-core` |
| `weave-application` | The use-case layer, where a command is checked, committed and read back. It owns the four command outcomes and is the only place a `CommitReceipt` is created. | `weave-core`, `weave-ports` |
| `weave-postgres` | The canonical store of an instance on PostgreSQL, the store the first vertical proves. Record changes, history, command results and the secret-free outbox commit together. The business application reaches it only through the port. | `weave-core`, `weave-ports` |
| `weave-repository-quality` | The home of the architecture test, which checks every other crate against this layering and the source rules. It holds no code of its own. | Nothing |

The core's intended decision is a non-async function with no ports: `decide(operation, command, read set, decision instant) -> Result<Plan, Refusal>`. The application derives the read set, loads it under protection, reads the instant through the clock port, calls the decision and commits its plan. The application's use cases cover definitions, checks, permission, commands, queries and transitions, and coordinate capabilities, managed rules, consequences, schedules, lineage, commits, outbox, reconciliation, recovery, schema evolution and presentation through ports. `CommitReceipt` has private fields and no public constructor, decoding, default or conversion from another type, so the facade and interfaces can read a receipt but never build one, and the store adapters have no dependency on the application.

## The crates that arrive

### Mechanics, facade and domain

| Crate | What it is for | Builds on, and what builds on it |
| --- | --- | --- |
| `weave-kernel` | The deterministic mechanics behind graph questions and queries, kept apart from domain rules and I/O so results can be repeated from known inputs: adjacency, generation-local handles, components, indexes, snapshots, traversal and compact layout. It arrives when graph mechanics first do. | Builds on no Weave crate; `weave-application` and `weave-host` build on it. |
| `weave` (`sdk/rust`) | The public in-process Rust facade, the one surface that interfaces, integrations and adopter domains build on. It re-exports the authoring builders; callers use its contracts rather than private storage or application details. | Builds on `weave-application`, `weave-authoring` and `weave-core`; every integration and every interface except `weave-host` builds on it. |
| `weave-commercial-work` | The first complete domain, written with the same authoring builders adopters use: accounts, evidence, signals, goals and work. Its definitions are described in [the domain pages](../domains/commercial-work/README.md). | Builds on `weave-authoring`; nothing builds on it, because an instance installs it as definitions. |

### Adapters

Each adapter implements one port, builds on `weave-core` and `weave-ports`, and is composed by `weave-server`; `weave-host` also uses the clock, observability and secrets adapters.

| Crate | What it is for, and the rule it keeps |
| --- | --- |
| `weave-cache` | Rebuildable caches that speed up reads. A cache never owns records, permissions or history. No cache technology is selected. |
| `weave-clock` | Clock observations for captured operation and schedule inputs. Time is explicit evidence, never a substitute for a record revision. |
| `weave-effects` | Effect attempts, and reading the provider back to see what happened. A database commit does not establish an outside effect or its acceptance. |
| `weave-identity` | Verification of external identity proofs, so a caller is named before an operation is checked. Bindings identify callers without granting the operation permission. |
| `weave-object-storage-filesystem` | Confined storage of immutable managed content on a local filesystem. It rejects traversal, symlinks and special files and verifies bytes after durable writes. |
| `weave-object-storage-s3` | The same port for S3-compatible services. Conditional writes, checksums, multipart upload, isolation and independent read-back need proof; an ETag is not a content digest. |
| `weave-observability` | Diagnostics and measurements for an instance and its lifecycle host. Logs may describe conflicts and unknown outcomes without exposing secrets or replacing owner evidence. |
| `weave-queue` | Transport that carries job references. Jobs can be lost or duplicated; the relational store retains commands, schedules and causal truth. |
| `weave-search` | Search over declared fields and indexes, PostgreSQL first. Permission checks cover candidate selection, ranking, counts and returned values. |
| `weave-secrets` | Resolution of a secret reference at the edge of an instance. Configuration holds references; key bytes stay out of records, outbox entries, logs and generated contracts. |
| `weave-wake` | Notifications and bounded recovery prompts that tell a consumer to look again. A notification requests canonical read-back and never establishes that a record changed or work was accepted. |

With `weave-postgres` these are the twelve adapters that `weave-server` composes. The SQLite store adapter is still to be added.

### Interfaces

Every interface builds on the `weave` facade, with no other Weave crate building on it, except where noted. Protocol access never bypasses the facade or adds permission.

| Crate | What it is for, and the rule it keeps |
| --- | --- |
| `weave-rest` | Semantic commands and narrow reads. It preserves acting identity, permission, revisions, all four outcomes and recovery. `weave-server` composes it with GraphQL. |
| `weave-graphql` | Rich reads generated from installed definitions. A resolver owns no separate traversal, ranking, disclosure or pagination rules; all come from the shared query model. `weave-server` composes it. |
| `weave-mcp` | A projection of the operations this protocol can faithfully represent. |
| `weave-acp` | A projection for clients that speak ACP. Request cancellation cannot roll back an outside effect; unsupported capabilities remain explicit. |
| `weave-a2a` | A projection for agents that speak A2A. Protocol tasks and artifacts become Weave records only through explicit commands under local rules. |
| `weave-cli` | The command line, for authoring, operation, diagnostics and recovery. |
| `weave-server` | The service of one instance: it composes the facade, the REST and GraphQL interfaces, the twelve adapters, schedules and the outbox. No other Weave crate builds on it. |
| `weave-host` | The lifecycle host, for provisioning, upgrading, backing up, restoring and retiring instances. It builds on `weave-ports`, `weave-kernel`, `weave-clock`, `weave-observability` and `weave-secrets`, and may not depend on the business application, the facade, the commercial-work domain or the business storage adapters (`weave-postgres`, `weave-search`, `weave-cache` and both object-storage adapters). |

### Integrations

Each builds on the `weave` facade, with no other Weave crate building on it. No mail, calendar, model or repository provider is selected.

| Crate | What it is for, and the rule it keeps |
| --- | --- |
| `weave-pale` | How Weave takes part in a Pale deployment: it publishes committed occurrences to Pale, with delivery and recovery. Pale's own reader and command caller live in Pale's repository, and neither half of the seam selects the other's permissions. See [the Pale integration](../integrations/pale/README.md). |
| `weave-buzz` | How Weave meets Buzz, the conversation surface: commands arrive from chat and permitted cards go back through the facade. Signed events resolve an exact actor and command; message delivery is a separate outcome. See [the Buzz integration](../integrations/buzz/README.md). |
| `weave-local-provider` | The stand-in provider for the first complete service: a deterministic external-effect provider with an inspectable sink, so no live mail or calendar account is needed. It can produce rejection, duplicates, lost replies and unknown outcomes on demand, so recovery can be tested. |
| `weave-mail` | The mail-provider family, as separately deployable packages. Native clients and credentials stay here once selected; the core keeps generic intents and independently verified results. |
| `weave-calendar` | The calendar-provider family, as separately deployable packages. A provider binding selects no account or permission. |
| `weave-repository` | The repository-provider family, for local Git or hosted repositories. A file write, a commit and a remote publication remain different effects. |
| `weave-model` | The optional model-provider family: a Weave instance may call a model only behind explicit caller permission and limits. No model, catalogue, provider or deployment is selected. |

### Tools

Neither builds on a Weave crate and no Weave crate builds on either.

| Crate | What it is for, and the rule it keeps |
| --- | --- |
| `weave-contract-codegen` | Keeps language bindings following the contracts, each of which has one producer, by generating the projections from the owning declarations. Generated schemas must not become a second handwritten command model. |
| `weave-release` | Assembles a release and the capability evidence for what it contains. Building or checking the workspace does not approve publication, deployment or activation. |

### Clients and emitted contracts

Thin remote clients for JavaScript and TypeScript (`@rtepl/weave`), Python (an RTEPL-namespaced distribution imported as `weave`) and Go (a module under RTEPL's getHarshOnline organisation) are planned in `sdk/js`, `sdk/python` and `sdk/go`. Each is a client over Weave's public REST and GraphQL contracts. It preserves the acting identity, revisions, all four command outcomes and recovery, and never opens PostgreSQL or embeds another server. Bindings are generated or checked against the producing contract, with shared positive and negative conformance cases, and domain builders must emit the same portable artifact as the reference Rust facade, so none carries a second semantic engine ([Extending Weave](extending.md)).

Emitted schemas, manifests and generated language types will live in `contracts/generated`, and each thin client in its own source home. These are the Apache-2.0 parts of the repository: each brings its own `LICENSE` and `NOTICE` when it first exists, under the [licence test](../README.md#licence). No contract has been generated or released yet; a change goes in the declaration at its producing layer, and the output is regenerated rather than maintained by hand.

## What the architecture test holds

- **Inner layers cannot reach outer code**, even through test dependencies: core, authoring, the commercial-work domain, kernel, ports and application depend only on what the graph above gives them.
- **Lifecycle management cannot read business records**: the host's forbidden dependencies are listed with it above.
- **Pure crates stay free of ambient effects.** Core, kernel, authoring and the domain may not reach the filesystem, network, environment, clock, randomness or an async runtime, and take no outside dependencies in any declaration (production, build or development).
- **The application exports no ports or adapters**, publicly or through a type alias, and the facade will carry the same rule the day it exists; and protected outcomes (`Refusal`, `CommandOutcome`, `CommitReceipt`) acquire no implicit construction (`From`, `Default` or `Deserialize` as the type requires). A source check proves the refusal with a failing fixture and the permitted form with a passing control.
- **Production source holds no tests and no unchecked calls** (`unwrap`, `expect`), and no file reaches 1,500 lines.
- **Domain and storage cannot gain runtime edges**: authoring cannot reach ports, the domain cannot reach the facade, and the store cannot reach the application.
