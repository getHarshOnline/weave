// SPDX-License-Identifier: AGPL-3.0-or-later

//! The canonical store of a Weave instance on PostgreSQL, the store the first vertical
//! will prove; SQLite is the other supported store. It builds on `weave-core` and
//! `weave-ports`, and `weave-server` composes it; the business application reaches
//! it only through the port. Record changes, history, command results and the
//! secret-free outbox must commit together.
//!
//! None of this is implemented yet.
