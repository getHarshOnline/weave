// SPDX-License-Identifier: AGPL-3.0-or-later

//! The shared meaning every other Weave crate builds on: records, relationships,
//! definitions, operations and transitions. It builds on no other Weave crate.
//! `weave-ports`, `weave-authoring`, `weave-application`, the `weave` facade and every
//! adapter build on it. Identity, capabilities, permission, time, source, revisions and
//! the record of where things came from belong here, with the pure decision over an
//! installed operation and its plain results: the plan, `Refusal`, `Conflict` and the
//! reconciliation handle. The four command outcomes and commit receipt belong to the
//! application, which alone commits and reads back. Business families and infrastructure
//! belong in domains and adapters.
//!
//! The intended decision is a non-async function with no ports: `decide(operation,
//! command, read set, decision instant) -> Result<Plan, Refusal>`. The application
//! derives the read set, loads it under protection, reads the instant through the clock
//! port, calls the decision and commits its plan.
//!
//! None of this is implemented yet.
