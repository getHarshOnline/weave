# Graphs, queries and search

Weave's records and relations form a typed, directed, temporal graph. GraphQL types derive from installed definitions. All interfaces lower reads into one private query form and one permission check, so a resolver cannot invent its own traversal, filtering or redaction rules.

## Relations retain their identity

Parallel edges are independent facts with their own revisions. Symmetry comes from a relation's definition, not invented inverse facts. Each definition decides whether self-relations and structural cycles are allowed. Traversal remains bounded even when cycles are valid. A witnessed route is distinguishable from a direct edge; neither supplies permission merely by existing.

The canonical store retains stable identities and full temporal meaning. A bounded hot view may use dense generation-local handles, compact adjacency ranges and column-style indexes. A handle never crosses an interface, becomes a foreign key, carries permission or is serialised. A generation fence rejects a stale handle. Graph, operation and memory-pressure benchmarks must establish the physical layout: memory density affects whether this model is practical.

## One private query form serves every interface

The internal form is typed and bounded; it is not a public query language. Its central job is to answer how records A and B are related under stated constraints. GraphQL provides rich reads, traversal, filtering, ordering, pagination and definition introspection. REST supplies narrow identity and history reads. SDK builders, domain clients, the CLI and agent protocols expose the queries they can represent faithfully. All use the same access, temporal, redaction and pagination rules and one execution plan, so no resolver owns any of that meaning itself.

A request chooses one route, a shortest qualifying route, several bounded routes or an explanatory subgraph. It declares permitted relation types and directions, deterministic relevance policy, validity time, snapshot, and depth, node, edge, cost and result limits. Limits also cover selected fields, fan-out, filter size, page and response sizes, history range and execution cost.

The result identifies exact revisions and applied constraints, whether it is complete or truncated, and unavailable, stale or denied portions where disclosure permits. Ordering is total and deterministic. A supported continuation binds the query, resolved definitions, snapshot, access frontier and last position; it supplies no permission. An expired snapshot requires a new query.

Compatible additive definition versions share a stable logical GraphQL type and member identities, with version-availability metadata. Incompatible changes get a new major type coordinate. Every result preserves its exact definition version. Missing for that version, absent, redacted, concealed, unknown, not requested and not applicable are distinct results, never one undifferentiated null.

## Five times answer different questions

Keep effective validity, observation time, record revision, read snapshot and query execution time separate. A collection of revisions does not by itself prove one atomic snapshot. Claim “no path” only after exhaustively searching the declared view; a limit, filter or unavailable history is not absence. Current disclosure rules also apply to history. The store retains the history its supported queries require rather than promising a universal bitemporal database.

The [operation-algebra cases](../tests/operation-algebra/README.md) include responsibility changing from a direct stewardship relation to a role, work serving a goal, blockers and evidence supporting a decision. A relation required by the question cannot be ranked or budgeted out of the answer.

## Every read crosses a guard

In Weave's design the cost of read guards is real and cannot be bypassed. Because every object is public, private or protected ([concepts](concepts.md)), every read crosses a guard, and reads are where most of a burst lands, a burst being the sudden run of reads and writes a swarm of agents produces. So the dependency walk, the traversal of the graph for everything a read or write depends on, is the heart of the engineering, and the design, the stack and the implementation must solve it rather than route around it. Pale meets the same cost with its compiler: its runtime expects the declared world not to change while it is live, so the compiler walks the graph once and precomputes the decision indexes its runtime only looks up, moving the cost from run time to compile time. Weave's objects change constantly, and delegated access adds to the difficulty, so Weave has to place the cost deliberately.

## Permission covers what a query reveals

Check permission before fetching or using a fact. The check covers selection, predicates, ordering, ranking, traversal, existence, counts, aggregation, export, disclosure and model use. A concealed fact must not change visible matches, ranking, page boundaries, path existence, counts or timing. The [access model](interfaces.md) supplies the decision; checking only returned fields is insufficient.

Exploration, dependencies, notifications, reporting, context hydration for Pale use the same identities and graph mechanics with different traversal policies. Each organisation authors its room layout. Notification coverage is a bounded computation over the graph, with explicit fan-out, duplicate and cycle limits. A consequence cannot quietly change the inputs that produced it, and a publication-safe view cannot replace the full record a permitted reader needs.

## The store supplies first-version search

Discovery uses exact identity and revision lookup, typed filters, bounded permitted traversal and lexical full-text search over fields declared searchable. Results have stable pagination and revision evidence. Each definition declares searchability, indexes and ranking inputs. A combined search vector must not cross fields with different use permissions.

Contextual or vector retrieval belongs to a separately selected Scope integration through Pale. Scope reads Weave through Weave's read port, still to be proposed by Scope's and Weave's builders, with Pale or without it, and without Pale it is how an agent reads Weave; Weave itself never calls Scope. Scope owns eligibility, freshness, exact, lexical and graph retrieval, permitted embeddings, fusion and use evidence for its own accepted projections. It is not a Weave search adapter. Weave owns no first-version embedding index. A future external search engine could only accelerate a rebuildable view and must recheck permission and read Weave back before consequential use.
