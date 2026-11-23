# Recovery by original identity

These are planned cases. Apply the [shared case requirements](../README.md).

Exercise [parent replay after a later revision](../pale/README.md#a-child-result-leaves-a-separate-parent-decision), [standalone restore](../standalone/README.md), and [Radar retry and compensation](../../domains/commercial-work/tests/README.md). Unknown outcomes remain unknown until the owner can confirm them; replay cannot create a replacement command.

Refuse a command because a blocker is present, clear the blocker, then resend the same command under the same key. The original refusal returns and nothing commits. A new key that references the refusal is decided afresh. Resend changed content under the refused key: it is refused as a reused key and nothing executes. Repeat the first resend after activating a newer definition version, and the original refusal still returns. Two identical commands under two keys remain two commands.
