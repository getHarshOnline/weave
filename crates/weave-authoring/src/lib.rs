// SPDX-License-Identifier: AGPL-3.0-or-later

//! The way a domain defines its business world in Weave: builders that author domain
//! definitions as inert, digest-bound data. It builds on `weave-core` alone. The `weave`
//! facade re-exports it, and `weave-commercial-work` is written with it. A domain built
//! here can reach records, definitions and operations, never the ports or stores that run
//! them.
//!
//! None of this is implemented yet.
