// SPDX-License-Identifier: AGPL-3.0-or-later

//! The boundary between Weave's decisions and its infrastructure: narrow contracts for
//! storage, object storage, cache, wake, queue, search, clock, identity, secrets,
//! observability, effects and event publication. It builds on `weave-core`.
//! `weave-application` and `weave-host` use these contracts, and every adapter in
//! `adapters/` implements one of them. Concrete providers stay outside this crate.
//!
//! None of this is implemented yet.
