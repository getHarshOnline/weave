# Commercial-work acceptance

These planned cases use the public SDK and interfaces; there are no implemented domain tests yet. Apply the [shared case requirements](../../../tests/README.md). First run the [first vertical](../../../tests/standalone/README.md), then the [human-only journey](../../../tests/README.md#human-only-commercial-work) and the failure branches below.

Once the Radar family is installed through the SDK, extend the [standalone journey](../../../tests/standalone/README.md) with fresh test data. The [Radar design](../radar.md) defines the required distinctions.

Change a required source after the run starts, or make its selector fail while the source bytes stay the same. If the dependency cannot be reconciled, read back the preserved evidence and the blocked run. Any staged inference stays provisional, and the run emits no current candidate or handoff. Repeat with a change outside the declared dependencies. The run can finish against the same verified snapshot.

Deliver the blocked run's reader view and verify that its terminal state stays blocked. Then resolve the required source in a fresh run and keep the evidence below the declared materiality threshold. Read back a completed no-change result with the checks performed, strongest counter-case and next question. Verify that neither run produces an owner ruling.

Fail a required validation before the authorised ruling is applied. Read back the original ruling, the failed attempt and the prior owner state. In a separate run, fail a reader view after application commits. Read back the committed owner revision and the stale or unavailable view independently. Where the test authorises compensation, record and read it back as its own transaction. Reject state-changing retries with stale preconditions or expired authority. Authorise a fresh attempt and link it to the failed one. Exact replay returns its original receipt and creates no second state change.

Apply the strict decoding and failed-write checks to a persisted record that fails its declared schema. Verify that recovery preserves the original bytes and requires its own bounded authorisation. Read back the prepared correction separately from the committed correction and the retried ruling, and verify that the references to the findings and the ruling's disposition are unchanged.

For a ruling that requests a downstream effect, withhold the destination commit and verify that the Radar record keeps the authorisation while the effect remains unverified. In a second run, drop the reply after the destination commits. Reconcile against the destination's native receipt and read-back, then replay delivery and verify that the same outcome returns without a second effect. Use the standalone conformance provider. See [effects and artifacts](../../../docs/effects.md).

Also exercise a second unrelated domain through the public SDK with no private data, Pale, Scope, Buzz or Gas Town. That separate journey establishes the clean-adopter claim.
