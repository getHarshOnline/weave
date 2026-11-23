# Chat approval and publication

These are planned cases, not passing tests. Use the [shared case requirements](../../../tests/README.md) and [Buzz contract](../README.md).

**Approval in chat.** An authorised person's "approved" in a room lands as one exact command with the actor bound. An unbound identity, an ambiguous target, a stale revision or missing evidence produces a refusal. Two rooms cannot approve the same thing twice. Prove a relay outage before admission leaves a visible gap or pending state, and losing the reply after a native commit recovers the outcome without another command. Exercise comment publication separately.

Exercise reconnect after losing a Buzz room interval with an authorised approval inside it. Return `EOSE` with the approval still missing. Verify a visible gap and no fabricated command. Repair the gap through an authorised read, then deliver the same signed approval twice across the overlap. Verify one native command and one outcome by Weave read-back. Change the room's work mapping before the second delivery and verify that no command reaches the new target. A second event at the same timestamp keeps its own identity.

Run an authorised approval through the native chat path with Pale absent, then with Pale installed and the integration unselected. From equivalent starting records, both runs must produce the same Weave outcome without a Pale admission. Select the Pale-controlled path and repeat with and without the required admission. Verify the refusal leaves no native transition, and the admitted case still passes Weave's actor and revision guards.

## A committed comment can still owe publication

Revoke the selected chat placement after the Weave commit and before a delivery retry. Read back the original comment and unresolved obligation. The retry publishes nothing, creates no new comment and selects no substitute room.

Replace an already published event whose original exact-ID read-back was recorded. Retain that receipt as historical evidence. Show the successor or current absence according to the reader's permissions. The old receipt cannot confirm current visibility or recipient delivery.

Return a different event ID with plausible room and thread tags to an exact-ID query. Leave publication unresolved. If the query was resolving a reply anchor, refuse placement before sending. Read back that no substitute room, thread or Weave command was selected. The adapter must not treat plausible tags as proof of the requested identity.

Repeat with an attachment and a missing or substituted parent. Read back zero uploads and zero events. With an authorised parent, acknowledge the upload and then refuse the event. Preserve separate upload evidence and reconcile it without marking the comment published.

Exercise Weave's [placement contract](../README.md#a-comment-and-its-publication-have-separate-results). Commit a comment during an outage with and without a declared conversation duty. Only the first creates an obligation. Repeat with two eligible rooms and verify that one unresolved obligation commits and no event is sent. Then lose the reply after an admitted send and revoke the send grant. With read authority intact, record the exact event's visibility and leave the receipt pending if the relay disposition is unavailable. With read authority denied, keep the read-back unresolved. Verify no second event, substitute room or comment mutation.

For a replaceable projection, replay the winner, then submit a superseded event under a different ID. Exercise an affirmative duplicate response with a read-back that returns only the winner in place of the requested event. Keep the superseded attempt separate. Read back the exact signed event and its relay disposition as the control for a publication receipt. Delivery and reading need their own evidence.

Revoke the room's reply grant after Weave records a refusal and, in a separate run, after it commits an approval. Verify the native outcomes by authorised read-back. The room receives no reply or card update and retry commits no second command. Repeat with the reply grant intact and verify its disclosure limits independently of the actor's command grant.

Compare a quiet delay with a critical breach under the same escalation contract. Revoke the escalation route before send in one run and revoke access to the protected link after publication in another. The first sends no alert. The second retains the alert receipt and denies dereference. Replaying the alert or acknowledging it leaves the original duty and its assignment unchanged.
