# Separate writers

These are planned acceptance cases, not passing tests. Apply the [shared case requirements](../README.md) to each branch.

**The three kinds of change.** Ordinary data, a new definition version and a managed-constitution change, including recovery from a partly materialised change; comments need no recompilation, and new definitions never rewrite old records.

**Applied policy.** Change a tunable policy value without changing the domain version; retain both in transition history. Evaluate a rule set as the experimental subject. Plant an unavailable assigned arm and prove that an allowed fallback records the revision and arm actually used, with the failed assignment separately visible. A rejected fallback must produce no transition. Compare the recorded provenance with the actual evaluator inputs, rather than checking only that labels were written.
