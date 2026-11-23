// SPDX-License-Identifier: AGPL-3.0-or-later

//! The home of Weave's architecture test, which checks every other crate against the
//! intended layering. Repository checks use `rtepl-common` for mechanics and keep Weave's
//! dependency graph and source rules in that test. This crate builds on no Weave crate,
//! and none builds on it.
//!
//! The crate holds no code of its own; the checks run from the architecture test.
