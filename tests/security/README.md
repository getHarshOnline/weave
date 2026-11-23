# Actor checks and hosted isolation

These are planned acceptance cases, not passing tests. Apply the [shared case requirements](../README.md) to each branch.

**No leaks through authorisation.** A concealed relation affects no filter, ordering, existence check, count, aggregate, traversal or lexical ranking, and absent, redacted, concealed, unknown, not requested and not applicable stay distinguishable. An admitted actor succeeds within its scope. A shared login with no admitted acting record is refused. Repeat with authentication switched off and verify that the actor and authorisation checks still apply.

**Hosted isolation.** Two sovereign databases share a cluster but no table, role, pool, object or key namespace or data-plane audience; `weave-host` completes lifecycle work without reading business data; a missing controller declaration does not decommission an instance.

**Lifecycle host.** Verify separate listener, audience and credentials, and separate database, pool, object and key namespaces; lifecycle management must read no business data. Omission from controller configuration cannot retire an instance.
