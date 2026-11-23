# Extending Weave

A domain defines the records, relationships and actions an organisation needs. First-party and adopter domains would use the same public SDK. The SDK is the only way to extend Weave, and a first-party domain gets no power an outside adopter lacks. This chapter explains the extension design through an investigation's acceptance rule. The SDK and Weave's runtime are not implemented, so there are no executable builders or package commands yet.

## A domain is declared, compiled and installed

The designed path has three stages: `DomainDefinition` → `ValidatedIR` → `InstalledDefinition`.

### Declare the business rules

Typed builders describe record types, fields, relations, operations, state changes and their conditions. Rust is the reference SDK. TypeScript, Python and Go builders would produce the same portable package artifact.

### Validate the definition

The compiler primarily validates the surface the domain declares. It checks types, dependencies, permitted effects and whether evaluation will terminate. It rejects invalid or unsupported behaviour before installation. The validated intermediate representation stays private. Validation and ordering precede transactional application, so an invalid definition cannot partly change the store. The validator is typed, with a closed field vocabulary; declared dependencies and explicit precedence determine the order of changes. How installed definitions are cached remains open.

### Install a version

The installed definition makes that version available to records and operations. The artifact has a schema version, a canonical form, a digest that identifies its contents and support for signing. A record refers to the version that governed it.

The builders form a closed expression tree; closures are never stored or run later. Authors use builders in ordinary languages, rather than a new source language. Raw artifact JSON is an interchange format, not the intended authoring experience. Later visual or textual tools must emit the same public artifact. The compiler's validated representation, permission plan, query plan, storage plan and hot representation remain private.

One mechanism covers types and subtypes, field constraints, relation cardinality, operation-role constraints, independent lifecycle and other state axes, guards, value-returning derivations, validations, deterministic reactions, effect intents, post-commit observers, evidence, checklists, waiting reasons, service levels, calendars, schedules, permissions, command and presentation descriptors, views, events and reporting dimensions. A transition may change several state axes together when their joint conditions pass.

### Express an investigation's acceptance

In the [export investigation](https://rawthoughts.in/weave/from-request-to-accepted-work/), supplying a trace and accepting the investigation are separate actions. A commercial-work domain needs to express both:

| Declaration | What it expresses |
| --- | --- |
| Work, evidence and participant definitions | The investigation, supplied trace and acting records |
| Assignment and review relations | Who owes the result and who can accept it |
| Submission operation | Supplies a result without accepting it for the reviewer |
| Acceptance operation and guards | Checks that the reviewer may accept the work, that the agreed material is present and that the conditions still hold |
| Declared reads | Names the records, relations, policies and absences used by the decision |
| Transition and result presentation | Records acceptance, or explains a refusal or conflict |

This example shows the rules a domain would need. The domain defines what acceptance requires. The application protects the decision through commit, and the interface presents its result. A successful response from an outside service cannot cause a provider adapter to insert an acceptance.

Compiled behaviour declares its reads, affected transitions, determinism, purity, permitted effects, captured dependency versions and behaviour version. Guard order is explicit where it affects meaning. These declarations let the runtime detect a new blocker or revoked reviewer authority, as explained in [Protect the decision through commit](./concepts.md#protect-the-decision-through-commit).

### Keep evaluation deterministic

Evaluation must be typed, finite and deterministic. It has no ambient access to the network, database, models, clock, randomness, filesystem or secrets. Outside facts enter only as authorised, captured inputs or as later results from the service that handled an effect. This includes clock inputs: the decision uses a captured value from its permitted source, not a caller-selected replacement for the decision’s clock. An action that depends on nondeterministic behaviour produces an effect intent for later execution. Evaluations are auditable, and an unsupported or invalid evaluation is an explicit result.

For example, accepting a trace cannot call a mail provider midway through the database transaction. Acceptance can commit with an outgoing intent; a later attempt records the provider's delivery result. A lost response can then be recovered without pretending that a database commit delivered a message.

Domain authors must declare more than an arbitrary callback would require. In return, the runtime has explicit dependencies and effects to check. Hiding rules in migrations, queue consumers or interface code would bypass those checks.

A domain cannot patch the engine or bring a second semantic engine. Remove a capability the runtime cannot constrain before an effect; detecting the violation afterwards is insufficient. A possible later deterministic WebAssembly boundary would require digest-bound modules, typed inputs and outputs, declared reads and writes, resource limits and the same lack of ambient access. It is not part of the implemented SDK.

## Keep a finding distinct from its follow-up

[Reviewing an observation](https://rawthoughts.in/weave/review-an-observation/) needs separate records for the observation, evidence, proposed explanation and review. The writer proposing a finding cannot accept it. Review needs an authorised decision-maker, criteria, evidence and policy version, protected by the same read and revision checks as other commands.

A completed review can find no material change. A run that could not obtain a required source instead records the missing prerequisite. It cannot claim to have ruled out change. Later evidence can support a new review without rewriting the earlier decision or exceeding the source's retention rules.

Accepting a finding does not assign work or deliver a remedy. A separate work-creation command establishes authority and acceptance conditions, then links the result to the finding. Return to [Create follow-up deliberately](https://rawthoughts.in/weave/review-an-observation/#create-follow-up-deliberately) for the participant's decision.

## Versions never rewrite history

Each record and relation revision names its immutable definition version and artifact digest. Activating a new version makes it the default for new records. Existing records keep their interpretation.

Suppose a later investigation definition requires another piece of evidence. It cannot retroactively invalidate an earlier acceptance. Moving existing work to that version requires an authorised adoption command checked against the record's expected revision. Mixed versions are normal; adopting many records is a resumable series of commands.

Retiring a version prevents new selection. Its artifact, interpretation and history remain while revisions or receipts refer to it. Definition adoption is separate from storage schema evolution. Neither can silently reinterpret old data.

A new definition version maps the names it changes explicitly. A migration that leaves a record or field out does not delete it: an omitted record stays intact until an authorised lifecycle command ends or deletes it.

Archiving a referenced artifact elsewhere does not remove this retention requirement. Lifecycle is one state axis among several, and constrained values are not database enums. The store keeps universal identities and revisions in normalised tables and declared fields in validated JSONB with declared indexes.

## Policy revisions record what actually ran

A domain version defines the meaning and permitted shape of its rules. Tunable values, such as an interval or escalation threshold, belong to separately versioned policy records within that shape. Changing a threshold does not require inventing a new domain version, and a policy cannot change the domain's meaning or exceed its constraints. Both versions remain part of the transition's captured dependencies and history.

A rule set can be the subject of a test or experiment. Record the policy revision and, when applicable, the experiment assignment and arm actually evaluated. If loading an assigned arm fails and an allowed fallback runs, name the fallback and the failure; do not label the resulting transition as execution of the requested arm. Comparing candidate rules requires the same inputs and explicit differences in the rules, with effects confined by the evaluation's authority.

## Packages are released on their own

Each domain has its own source and release cycle and depends only on public Weave contracts. The engine depends on no domain and must pass its kernel and application tests with none installed. Package dependencies use exact versions and digests. Conflicting extensions fail; load order cannot determine meaning or widen authority.

The first planned domain is [commercial work](../domains/commercial-work/README.md): accounts, people, relationship contexts, evidence, Radar records, goals, work, comments, artifacts and schedules. It must install and be removable without rebuilding the engine. Its families, commercial cases and work rules live with that package. A second unrelated domain must demonstrate the same extension powers without private engine imports or client-specific branches.

Domain definitions, ordinary data and the organisation's managed constitution have separate writers. Defining an assignment relation does not grant its occupant data access or spending authority. Removing a package cannot discard history still tied to its definitions.

## Contracts have one producer

Declare a contract where its meaning is produced. Generate its language forms or test them for conformance; do not maintain another handwritten command model in `contracts/`. That directory holds emitted schemas, manifests, generated types and conformance material, with design notes clearly distinguished from released wire formats. Variants, payloads, errors and uncertainty must survive every projection. Decoding and permission checks remain separate.

[Capability resolution](effects.md#resolve-a-capability-before-selecting-a-provider) selects replaceable provider packages through the same actor, target, purpose and freshness rules. A binding grants no permission. The deterministic local provider is the first complete contract example; a live vendor is not required to prove it.

## Licensing

The engine, application, adapters, interfaces, integrations, first-party domains and Rust SDK use AGPL-3.0-or-later. Emitted contracts, generated types and genuinely thin language bindings use Apache-2.0. A binding that carries Weave's semantic logic is not thin and falls under the AGPL.

An adopter's own systems, configuration and data stay theirs. Alternative terms require a separate agreement with RawThoughts Enterprises Private Limited.

Return to [reviewing the investigation](https://rawthoughts.in/weave/from-request-to-accepted-work/#keep-the-result-and-the-remaining-responsibility-visible) for the work these rules protect, or to [Weave](./index.md) for the runtime's responsibilities.
