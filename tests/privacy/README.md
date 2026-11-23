# Private reads and erasure

These are planned acceptance cases, not passing tests. Apply the [shared case requirements](../README.md) to each branch.

**No leaks through authorisation.** A concealed relation affects no filter, ordering, existence check, count, aggregate, traversal or lexical ranking, and absent, redacted, concealed, unknown, not requested and not applicable stay distinguishable. An admitted actor succeeds within its scope. A shared login with no admitted acting record is refused. Repeat with authentication switched off and verify that the actor and authorisation checks still apply.

**Artifact custody and erasure.** An external reference, an explicit managed capture, equivalent behaviour on the filesystem and on S3-compatible storage, minimal receipts, authorised erasure or crypto-shredding with a non-identifying tombstone, and no claim that a transport or repository is the object store.

**Filesystem storage.** Reject traversal, symlinks and special files under an already-open confined root; prove digest verification, durable finalisation, independent read-back and recovery across every metadata and content split.

**S3-compatible storage.** Run the same journey as the filesystem. Add conditional writes, checksums, multipart failures, versioned read-back, namespace isolation and deletion; never substitute an ETag for the content digest.
