# Working on Weave

Weave's design is where an organisation of people and agents keeps its relationships and work: who owes what, what happened and who accepted the result. The repository holds six layered Rust crates (core, authoring, ports, application, the PostgreSQL adapter and the repository-quality checks) with their dependency checks and no behaviour yet; the first vertical, a service with record operations and the store that saves a change and recovers it, is the problem in front. Add a crate only with its first real operation and test. The standalone Rust/PostgreSQL engine described here is the engine. If Twenty is used, it is a client of that engine, never Weave's runtime or store; whether it hosts any part of Weave is still being compared.

Start with [README.md](README.md) for the proposed layout and [the manual](docs/index.md) for the model. Read [Concepts](docs/concepts.md) before changing records or commands, [Extending Weave](docs/extending.md) for domain rules and licensing, and [Instances](docs/instances.md) for hosting, identity and exchange.

## Build and check

Run `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`. The [README](README.md#build-and-check) explains the shared-check dependency. Keep its Git pin; do not use a sibling path dependency.

The architecture suite holds Weave’s exact dependency graph, outside-crate allowlists and source policy and calls the shared `rtepl-repository` engine. Core and authoring stay free of outside effects, as the kernel and domains will. The application exposes no ports or adapters, nor will the facade; protected outcomes and receipts permit no implicit construction. Keep the negative cases proving these refusals with the policy. Each crate’s module comment owns its responsibilities, and [the crates page](docs/crates.md) holds the planned crates and their rules. Generated Rustdoc stays outside Git. Behavioural tests belong in each component’s `tests/` or the named root suites as implementation arrives.

## Keep responsibilities clear

The layers are `crates/weave-core` for shared meaning, `crates/weave-authoring` for domain authoring that reaches only core, `crates/weave-ports` for infrastructure contracts, `crates/weave-application` for use cases and `adapters/` for implementations; the kernel for portable deterministic mechanics arrives with the first graph mechanics. SDKs and interfaces use the same commands. Product families belong in domain definitions, not new root classes in the core.

The easy mistakes to avoid are:

- Acting is a capability of a record, and which records may fill each end of an acting-for relation is declared by the domain, never hard-coded in the engine. Keep the caller, represented party, role, assignee and data subject distinct. The commercial-work domain declares that an agent acts but is never the principal of an act and that every chain of acting for ends at a person or an organisation; the engine must be able to enforce such a declaration.
- Preserve all four command outcomes: rejected, conflict, committed and commit-unknown. Recover an unknown result by command identity before retrying. Every outcome, a refusal included, is final for its key; trying again is a new command.
- Protect relationships and absent blockers as well as scalar revisions through commit. A lost reply or a stale card cannot change the recorded outcome.
- Keep one canonical store. Queues, caches and wake-ups carry references; Redis and BullMQ are unselected, and graph handles are temporary derivatives.
- Keep Weave's committed-event publisher in `integrations/pale`; Pale's reader and command caller belong in Pale's repository. A Weave fact grants no Pale principal, delegation, capability, provider, machine binding or right to act. Each product owns its own causal history and results.
- Keep ordinary data, domain definitions and organisation rules separately writable. A provider binding does not grant permission, and a submitted result still needs its designated reviewer's acceptance.

Keep adopter configuration and private data with their owners. Contract declarations have one producer; language bindings and emitted artifacts follow it. Preserve the manual's worked cases when extending these rules. The lifecycle host may not depend on the business application, SDK or business-storage adapters. Fresh native data does not remove backup, restore, schema evolution, decommissioning or unknown-effect recovery requirements.

Use the Engineering, Architecture, Automation and Evaluation families from Pale Packages when their methods help the task. Reach them through the machine’s shared navigation; public repository documentation contains no private checkout paths. Project behaviour belongs in the guide and code, not a copied procedure on every page.

<!-- BEGIN shared repository rules -->

- Review a source file above 1,000 lines and split it by concern before 1,500.
- Keep Rust tests under `tests/`; keep Go's `_test.go` files beside their package.
- Finish a goal with the codebase the same size or smaller unless a requested feature requires growth. Preserve the behaviour, tests, cases and examples.
- Every engine must build from a fresh clone on Linux and macOS.
- Keep generated API reference and build output out of Git; change their source or generator.
- Write comments and API documentation for the code's reader. A crate or module comment first says what it is for in the system, what it builds on and what builds on it, then its purpose, inputs, results and useful edge cases, with the purpose written problem-first in the present tense: the problem it solves and what solves it today. One plain line after that opening says what is not built yet, as the problem now in front of it, and a negative appears only where a reader would expect the opposite: a limit stated as a property of what exists stays ("it runs on loopback with no caller authentication"), while a negative measured against the finished design is a hedge. Keep them self-contained, with links inside the repository.
- Push each repository as one commit whose whole message is `init`, with Harsh as author and committer under the account the folder uses and no `Co-Authored-By` or `Claude-Session` trailers; later commits stay short and plain.
- Describe the present design and research. The repository neither links nor ships lineage: earlier papers, predecessor versions, history tags and the commits behind them.
- Public repository files, including generated text, link within the repository or to public addresses their readers can open.
- A README is the repository's front door: it carries the product account for the technical reader who arrives there, opening with the product's plain name and the problem it solves, in the words its published pages use.
- Name every operation, command, tool, type, field and variable so a reader without its author's context can tell what it does, what it enables and when to use it; docstrings say the same. Keep full-field record schemas for the engine's consistency, named in plain words like everything else, and author each surface's schemas separately, one per intent, in its caller's terms: SDKs and APIs, the command line, the language server and the agent protocols (MCP, A2A, ACP) are different surfaces with different callers, and the agent protocols compose intents over the operations the others expose.
- Write each Markdown paragraph on one source line. Run the repository's formatter with `proseWrap: never` on changed Markdown.

<!-- END shared repository rules -->
