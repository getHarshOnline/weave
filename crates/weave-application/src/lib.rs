// SPDX-License-Identifier: AGPL-3.0-or-later

//! The use-case layer of Weave, where a command is checked, committed and read back. It
//! builds on `weave-core` and `weave-ports`, and reaches storage and every outside system
//! through ports. The `weave` facade will build on it, and interfaces and integrations
//! will reach it through that facade.
//!
//! Use cases for definitions, checks, permission, commands, queries and transitions.
//! Coordinates capabilities, managed rules, consequences, schedules, lineage, commits,
//! outbox, reconciliation, recovery, schema evolution and presentation through ports.
//! Owns the four command outcomes in the planned `CommandOutcome` type. Only the module
//! that commits and reads back creates a `CommitReceipt`, with private fields and no
//! public constructor, decoding, default or conversion from another type. The facade and
//! interfaces can read a receipt but cannot build one; the store adapters have no
//! dependency on this application.
//!
//! None of this is implemented yet.
