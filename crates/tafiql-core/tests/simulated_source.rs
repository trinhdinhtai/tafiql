//! Tests for the simulated 10M-row source, driven through the public API.
//!
//! This is the fixture the Windowed Fetch engine, the criterion benchmarks
//! and the CI memory gate all run against (ADR 0004), so its windowing
//! semantics are the specification: keyset windows over ascending ids,
//! never materializing the whole table.

use tafiql_core::{Row, RowKey, SimulatedSource};

#[test]
fn ten_million_rows_fixture_reports_ten_million_rows() {
    let source = SimulatedSource::ten_million_rows();
    assert_eq!(source.row_count(), 10_000_000);
}

#[test]
fn first_window_returns_rows_in_ascending_id_order_up_to_limit() {
    let source = SimulatedSource::with_row_count(100);
    let rows = source.window(None, 10);
    assert_eq!(rows.len(), 10);
    assert_eq!(rows[0].id, 1);
    assert_eq!(rows[9].id, 10);
}

#[test]
fn window_after_a_key_returns_only_rows_strictly_after_it() {
    let source = SimulatedSource::with_row_count(100);
    let rows = source.window(Some(RowKey(40)), 10);
    assert_eq!(rows[0].id, 41);
    assert_eq!(rows.len(), 10);
}

#[test]
fn successive_keyset_windows_tile_the_row_space_without_gaps_or_overlap() {
    let source = SimulatedSource::with_row_count(97);
    let limit = 30;
    let mut seen_ids = Vec::new();
    let mut after = None;
    loop {
        let rows: Vec<Row> = source.window(after, limit);
        if rows.is_empty() {
            break;
        }
        seen_ids.extend(rows.iter().map(|r| r.id));
        after = Some(RowKey(rows.last().unwrap().id));
    }
    let expected: Vec<u64> = (1..=97).collect();
    assert_eq!(seen_ids, expected);
}

#[test]
fn window_past_the_end_is_empty() {
    let source = SimulatedSource::with_row_count(5);
    assert!(source.window(Some(RowKey(5)), 10).is_empty());
    assert!(source.window(Some(RowKey(999)), 10).is_empty());
}

#[test]
fn last_shorter_window_returns_the_remaining_rows() {
    let source = SimulatedSource::with_row_count(25);
    let rows = source.window(Some(RowKey(20)), 30);
    assert_eq!(rows.len(), 5);
    assert_eq!(rows.last().unwrap().id, 25);
}

#[test]
fn deep_keyset_window_reaches_the_far_end_of_ten_million_rows() {
    // Keyset windows make a window at ~10M depth as cheap as the first
    // one; this also proves the fixture never materializes its rows.
    let source = SimulatedSource::ten_million_rows();
    let rows = source.window(Some(RowKey(9_999_990)), 10);
    let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
    assert_eq!(ids, (9_999_991..=10_000_000).collect::<Vec<u64>>());
}

#[test]
fn rows_are_deterministic_for_a_given_id() {
    let a = SimulatedSource::with_row_count(10);
    let b = SimulatedSource::with_row_count(10_000);
    let row_a = a.window(Some(RowKey(6)), 1).pop().unwrap();
    let row_b = b.window(Some(RowKey(6)), 1).pop().unwrap();
    assert_eq!(row_a, row_b);
}

#[test]
fn zero_limit_returns_no_rows() {
    let source = SimulatedSource::with_row_count(10);
    assert!(source.window(None, 0).is_empty());
}
