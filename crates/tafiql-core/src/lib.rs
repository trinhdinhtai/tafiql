//! tafiql's headless core.
//!
//! Everything in this crate is GPUI-free: windowed fetch, stores, and the
//! `Connection` trait live here so they are testable and benchmarkable
//! without a window (ADR 0003).

mod simulated_source;

pub use simulated_source::{Row, RowKey, SimulatedSource};
