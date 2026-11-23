# Weave

Agents decide far more often than people, so by the time one writes, what it read has usually changed. Weave's design makes a read say what else to read, and a write is tried for what it affects before it is committed alone, made together with others or proposed to the owners. Weave keeps the records an organisation's people and agents work from, so that whoever comes next, a person or an agent, picks up the story from what was kept rather than from a long conversation. It is a headless backend that does the jobs of a work tracker and a relationship manager together, standing on its own; other products, such as [Pale](https://rawthoughts.in/pale/), Scope and Gazette, return to its records. Accounts, commitments, evidence, goals and work share a record model, so a relationship can be followed through to what was promised, supplied and accepted, and parent and child work, comments and their relationships are records with their own history. Each actor uses its own identity and permission. The [components](interfaces.md#components-look-the-same-on-every-surface) Weave serves look the same on every host surface. An optional Pale integration constitutes roles and coordinates agents while Weave keeps the organisation's records.

The [repository README](../README.md#build-and-check) records what is built. These chapters describe the design, rather than an installation recipe for a working service. Use them to trace a command, design a domain or work out what an integration owns. For the participants' experience, start with [From a request to accepted work](https://rawthoughts.in/weave/from-request-to-accepted-work/).

## Keep submission and acceptance separate {#a-system-that-refuses}

An agent supplies a trace for a customer's failed export. The person named to review the investigation decides whether it answers the question. The design has a command for each action. A comment saying “done” cannot stand in for either one.

Every interface uses the same commands and checks. A missing trace causes a refusal. A newer revision causes a conflict. A lost response calls for recovery of the original command's outcome. The developer's responsibility is to preserve those checks when adding a caller or domain. [Concepts](concepts.md) explains the record model and four outcomes.

## Follow a command through the runtime

The target is one Rust runtime with one relational transactional store per instance, SQLite or PostgreSQL; the first vertical will prove PostgreSQL. The same runtime runs as its own service or inside Pale's process as a module. The engine supplies records, relations, definitions and operations. A domain uses the public SDK to define its business world. Accounts, work items and Radar records are domain definitions rather than built-in engine classes.

1. **Identify the action.** The interface binds the acting record, operation, subject and expected revisions. It carries the request to the application without deciding the business rule.
2. **Check the decision.** The application checks permission and the installed domain's rules. It protects every record, relationship and absence used by that decision through commit.
3. **Commit and confirm.** The store writes changes, history, commit receipt data and secret-free outbox in one transaction. Independent read-back confirms the named commit before the committing module creates a `CommitReceipt`.
4. **Deliver later consequences.** A dispatcher carries committed occurrences to consumers. Message delivery, outside effects and another party's acceptance each need their own result.

A chat card can show an accepted investigation while its own update is still pending. Read the acceptance in Weave and check the card update with the chat provider. A handler that writes directly to business tables would bypass the application's checks.

Dependencies point towards the core: shared record meaning in the core, deterministic mechanics in the kernel, use cases in the application and infrastructure implementations behind narrow ports. Domain logic cannot hide in an adapter or interface handler. Keeping the decision apart from the database and from outside calls makes it repeatable from known inputs, and lets a database or an interface change without changing what a decision means. A store port inverts dependencies without promising arbitrary database support. [Repository instructions](../AGENTS.md) govern changes to these layers.

Universal identities, revisions, relations, commands, history, commit receipts and outbox entries use normalised tables. Domain fields use validated JSONB and declared indexes. This takes more explicit modelling than a table and handler per feature, but gives every caller the same rules and recoverable history.

## Find the contract you need

| Question | Chapter |
| --- | --- |
| What is a record, who can act, and what can a command return? | [Concepts](concepts.md) |
| How does a command protect its decision and recover a lost reply? | [Store and recovery](store.md) |
| How do graph questions, history and search remain private and bounded? | [Queries](queries.md) |
| Which surface should a caller use, and how do sign-in and revocation work? | [Interfaces and access](interfaces.md) |
| How are providers, recurring work, files and erasure handled? | [Effects, schedules and artifacts](effects.md) |
| How does a domain define and evolve its rules? | [Extending Weave](extending.md) |
| How do independently owned instances exchange work? | [Instances](instances.md) |
| Which crates exist, which are planned, and what rule does each keep? | [The crates](crates.md) |

Remote clients never open the store's database or embed another server. An agent using REST follows the same rules as any caller. A protocol task does not become a Weave work item by implication, and request cancellation cannot undo an outside effect. Access checks cover what filtering, ordering, traversal, existence, counts and aggregation reveal as well as the fields returned. Export and model use retain their purpose limits; the internal query representation stays private.

## Integrations keep their owners

Weave works without chat, agents or [Pale](https://rawthoughts.in/pale/). Pale consumes Weave through the same SDK and contracts as any other client, whether Weave runs as its own service or inside Pale's process, and Weave's definitions have one compiler, Weave's own; no integration gets a second one. Removing an integration must preserve records and uncertain outcomes. Under the [committed-occurrence design](../contracts/pale-occurrences.md), each product records its own part of shared work. The [Weave publisher](../integrations/pale/README.md) sends committed references; Pale's acquisition and command caller belongs in Pale's repository. There is no supported live connector until its wire format and recovery have been proved.

The paths to Scope, Counsel and Reckon go through Pale, for the requests Weave's own work makes; Weave itself never calls Scope on either route. In the other direction Scope reads Weave through Weave's read port, still to be proposed by Scope's and Weave's builders, with Pale or without it; it is not one of the infrastructure contracts in `weave-ports`, and where it lives is part of that proposal. Without Pale, Scope is how an agent reads Weave; with Pale it reranks as genealogy updates arrive and writes a pointer to the context it gave each agent into Pale's genealogy, and it never writes Weave's state. Gazette may acquire [publication facts](../contracts/publication-facts.md) directly before freezing its build input. [Buzz](../integrations/buzz/README.md) carries conversation, explicit commands and permitted cards. Event delivery alone grants no permission and proves no consumer action or acceptance.

## The first complete service

The first build exercises one vertical, where a steward qualifies an account against recorded evidence, the next action becomes work through an explicit command and the entitled reviewer accepts it. It runs without Pale or Buzz; the [standalone journey](../tests/standalone/README.md) specifies it.

The first version combines a standalone service with a complete [commercial-work domain](../domains/commercial-work/README.md). Its first outside-effect provider is a deterministic local test provider that exposes duplicates, lost responses and failures; live mail and calendar accounts are not prerequisites.

It includes no browser application, built-in models or historical-data migration. Recovery, backup, restore, schema evolution and decommissioning are still required. `weave-server` serves one instance; `weave-host` manages lifecycle without business-data access. The [acceptance journeys](../tests/README.md) specify what each implemented capability must prove for an exact release.

Return to [reviewing the investigation](https://rawthoughts.in/weave/from-request-to-accepted-work/#keep-the-result-and-the-remaining-responsibility-visible) for what these responsibilities mean to the person waiting for a result.
