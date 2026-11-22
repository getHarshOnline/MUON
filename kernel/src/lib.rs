//! The adaptive learning unit of Causality's research stack, behind the common interface
//! that HENA swarms compose.
//!
//! A swarm needs units it can swap and compare, which takes one slot contract with explicit
//! capabilities and limits. This crate builds on MONAD's formal types for a unit's state,
//! observations and resource budget, and re-exports them today; Q-MUON keeps the same
//! boundary for quantum units. The draft slot contract is in `docs/`; the slot conformance suite,
//! then the model adapters, are the work in front of it.

pub use monad_kernel;
