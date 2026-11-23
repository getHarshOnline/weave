# Weave

Agents decide far more often than people, so by the time one writes, what it read has usually changed, and the comments, findings and reasons that people and agents write to each other change with it. Weave's design makes a read say what else to read, and a write is tried for what it affects before it is committed alone, made together with others or proposed to the owners. Weave keeps the records the other RawThoughts products work from. It is where an organisation keeps its decisions, owners, acceptance conditions and outcomes as records, so a person can set the direction with a short decision and the next agent picks up the direction from the records rather than from a long conversation. [Radar](domains/commercial-work/radar.md) proposes a direction or a finding, and a finding becomes work only when someone takes responsibility for it. Any configured agent, on any accepted host and model, reads and writes the same records ([acting is a capability, not a kind](docs/concepts.md#acting-is-a-capability-not-a-kind)).

It is a headless backend that does the jobs of a work tracker and a relationship manager together, the jobs Jira and Salesforce do separately, for organisations whose members include people and agents. It serves tasks, goals, accounts and other CRM-like records, parent and child work, comments and the relationships between them, in one record model with each change attributed to its actual actor. It also serves components that look the same on every host surface: a chat room, a terminal, a web page or an agent's conversation renders the same window onto the same record under the same checks, and the components are not a separate browser application ([the component contract](docs/interfaces.md#components-look-the-same-on-every-surface)). Weave's design stands on its own: alone it is a headless Jira and Salesforce, a database of the organisation's records behind a capable interface. With Pale, which can constitute roles and coordinate agents while Weave keeps the organisation's records, people and agents work inside one organisation and across several, each instance kept separate.

Weave is a RawThoughts project of RawThoughts Enterprises Private Limited (RTEPL).

The repository holds six crates (core, authoring, ports, application, the PostgreSQL adapter and the repository-quality checks), shared repository checks, a dependency manifest and licence files. The problem in front is the first vertical: a service, record operations and the store that saves a change and recovers it.

## Read the design

The design is a standalone Rust engine with one relational store per instance, SQLite or PostgreSQL; the first vertical will prove PostgreSQL. A graph database enters only if engineering strictly requires it, and until then the kernel answers graph questions over the relational store. Weave runs as its own service, which Pale and other callers reach as clients, and inside Pale's process as a module; which one Pale uses is an engineering choice, and both keep the same commands and checks. If Twenty is used, it is a client of that engine through Weave's interfaces, never Weave's runtime or store, and no decision reads a value people can edit in Twenty.

Start with the [developer guide](docs/index.md). It explains records and commands, storage and recovery, queries, access, provider effects, domains and independent instances. The [commercial-work domain](domains/commercial-work/README.md) supplies the first relationship and work model. [Acceptance journeys](tests/README.md) preserve the positive, failure, concurrency and recovery cases each feature must eventually pass.

The first proof is a complete standalone journey through public interfaces and a deterministic local provider, with no live mail or calendar requirement. There is no first-version browser application, built-in model or historical-data import. Backup, restore, schema evolution and decommissioning remain required.

## Find the implementation home

- [Core](crates/weave-core/src/lib.rs), [ports](crates/weave-ports/src/lib.rs) and [application](crates/weave-application/src/lib.rs) explain what belongs in each layer, and [authoring](crates/weave-authoring/src/lib.rs) builds domain definitions as inert data that reaches only core.
- [The crates](docs/crates.md) names every planned crate, with what it builds on and the rule it keeps: the kernel, the Rust facade, the other adapters, the REST, GraphQL, CLI, MCP, ACP and A2A interfaces, the instance server and lifecycle host, the integrations and the tools. Each arrives with its first real operation and test.
- `adapters/postgres` holds the PostgreSQL store the first vertical will prove. The SQLite store adapter is still to be added.
- [Pale](integrations/pale/README.md) and [Buzz](integrations/buzz/README.md) describe their integrations. No mail, calendar or model provider is selected.
- [Contracts](contracts/README.md) and [tests](tests/README.md), which also lists the performance cases, keep emitted shapes, behavioural proof and measurement separate.

The intended package names are `weave-*` Rust components, `@rtepl/weave` for JavaScript/TypeScript, an RTEPL-namespaced Python distribution imported as `weave`, and a Go module under RTEPL's getHarshOnline organisation. No package is published, and a name here proves no registry availability.

## Build and check

Use Rust 1.94 or later. From this repository:

```sh
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
```

This builds the layered crates and runs the architecture test. There is no entry point, working API, SDK or integration, and no database schema, migration, released contract or deployable product.

The architecture test uses the `rtepl-repository` Git dependency from [rtepl-common](https://github.com/getHarshOnline/rtepl-common), pinned to an exact commit; Weave keeps its own small graph policy in [the test](tests/architecture/main.rs). Cargo fetches that commit on the first build. Do not replace the pin with a sibling path dependency or commit a machine-specific mapping.

The suite checks the exact local graph, outside production dependencies and Rust source rules. Core and authoring may not reach ambient filesystem, network, environment, clock, randomness or async-runtime operations. The application may not expose ports or adapters, and protected result types cannot acquire implicit construction traits. Negative fixtures exercise those refusals. Source inspection runs before macro expansion and does not prove the behaviour of allowed dependencies; generated code and exposed APIs still need review.

Rustdoc is generated under `target/doc/` and stays out of Git. The test-suite READMEs describe future behaviour, while the architecture suite checks the crate layout and dependency rules. [AGENTS.md](AGENTS.md) supplies working conventions.

## Licence

The engine, application, adapters, interfaces, integrations, first-party domains, Rust SDK and tools use [AGPL-3.0-or-later](LICENSE). Emitted contracts and generated types in `contracts/generated`, and genuinely thin JavaScript/TypeScript, Python and Go bindings in their own source homes, use Apache-2.0. Commercial use is permitted under the applicable licence. Alternative terms require a separate written agreement with RawThoughts Enterprises Private Limited; this repository grants none. See [NOTICE](NOTICE) for ownership, [CITATION.cff](CITATION.cff) for citation and [CONTRIBUTING.md](CONTRIBUTING.md) for contribution requirements. Outside contributions remain closed pending review of the [draft agreement](CLA.md) and a way to record acceptance. Third-party material keeps its own terms and required notices.

A binding implementing compiler, permission, graph, runtime, reconciliation or effect logic is not thin and falls under the AGPL.

The test is that a thin binding computes no Weave fact. Everything it shows its caller is a field the server returned, a fact about the client's own input and output (these bytes were sent, no reply arrived) or a fixed instruction the published contract declares, so two conforming clients given the same server bytes and the same input and output history show the same outcome and next step. Saving a request and its key before sending, reporting a lost reply as the client's own value, calling the lookup by identity and following the server's recovery instruction are thin. Computing the replay digest, deciding from record fields that a command committed, treating a missing lookup as safe to resend, choosing per route whether a resend is allowed, working out which commands to offer, or embedding the engine are not.

An adopter's own systems, private configuration and data remain theirs. Alternative terms require a separate agreement with RawThoughts Enterprises Private Limited. [NOTICE](NOTICE) records the scope and shared-check origin.
