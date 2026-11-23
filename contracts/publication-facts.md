# Publication facts

This design describes Weave's half of a publisher reading selected facts. It is not a released wire format or an implemented Weave provider. The types Gazette already has for acquiring facts do not by themselves establish this provider or allow external facts into deterministic compilation.

## Read an exact permitted field

A response must identify subject, record and field, owning instance and owner, delegated field responsibility, permitted-use references, exact revision or source frontier, observation time and freshness. Resolve only explicitly selected fields under the caller's current access and purpose. A display name cannot create a principal or change who owns publication rights.

The acquisition boundary resolves and freezes the requested values and their source evidence before compilation. A set of successful reads is not an atomic cross-record snapshot unless the provider contract and acquisition prove one. Templates cannot perform live reads or silently fall back to an older local value. Installing Weave does not replace local input selections.

## Refuse without substituting a value

Gazette's current `ExternalFacts` port uses these `UnavailableKind` outcomes:

| Outcome        | Meaning at the caller boundary                         |
| -------------- | ------------------------------------------------------ |
| `Unconfigured` | No provider was selected for this binding              |
| `Unsupported`  | The provider cannot supply the request                 |
| `Missing`      | No current value is available                          |
| `Stale`        | The selected value fails the caller's freshness policy |
| `Revoked`      | The owner revoked the value or access                  |
| `Unauthorized` | The caller may not acquire the selected value          |

The error identifies the exact failed request item and returns no partial snapshot. None of these outcomes grants retry, fallback or publication permission. Weave's richer state may also distinguish conflicting or deleted source records, but those are not current Gazette enum variants. The provider mapping must preserve the explanation and choose an explicit supported outcome, or require a reviewed contract extension; it cannot silently invent a new variant or collapse conflict into success.

Gazette currently projects an existing principal's display name and preserves opaque provider source and revision references. Its configuration binding is not a Weave address or record identity. The adapter translates the complete request and response, and Gazette checks exact membership before freezing the capture. Extending the target set, verifying freshness, or claiming snapshot coherence requires the corresponding owner contract and tests.

## Publication remains the publisher's act

Weave supplies facts, not permission to publish them. Gazette owns input capture, compilation and release; its workspace owns source selections, composition, placement, domains, routes, theme and overrides. Its registry and release journal are not a second editable Weave record store.

A person or Pale can run Gazette over local or Weave input. Gazette owns that four-case caller/input matrix and its conformance tests. An optional [Radar](../domains/commercial-work/radar.md) workflow may lead to investigation, drafting and review before ordinary Gazette build and release steps; the approved draft, orchestration operation and publication release remain distinct records. Direct publication input does not require that workflow or a running Pale.
