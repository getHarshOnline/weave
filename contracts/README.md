# Shared contract design

Weave defines each command, query and occurrence shape at its producing layer. This directory will hold generated schemas, manifests, language types and conformance fixtures, under Apache-2.0 with their own `LICENSE` and `NOTICE` when the first exists. Do not create a second handwritten semantic model here.

The following design notes describe what the producing implementation must establish. They are not released wire schemas, supported endpoints or proof of a working connector:

- [Committed occurrences and Pale](pale-occurrences.md): identity, causal parents, delivery and independent settlement.
- [Publication facts](publication-facts.md): exact permitted fields, source revisions and refusal before a publisher freezes its input.

Publish versioned shapes and positive/negative consumer cases as their native semantics are implemented and proved. A consumer pins that publication separately from enabling the live integration. Generated projections must preserve variants, payloads, errors and uncertainty; decoding a shape never grants permission to use it.

So that clients need compute nothing, every command answer carries the server-held command digest and a recovery instruction (look up, resend allowed or do not resend), and whether a route permits an exact resend is contract data per route and caller. The lookup by identity still needs three answers designed: an identity the server has never seen, how long an identity is kept, and a lookup answered after a restore.
