//! The simulated 10M-row source.
//!
//! A deterministic, in-process stand-in for a Large Table (see GLOSSARY):
//! it can serve keyset windows (ADR 0001) at any depth without ever
//! materializing its rows — row content is derived from the id. The
//! Windowed Fetch engine, the criterion benchmarks and the CI memory gate
//! all run against this fixture (ADR 0004).

/// The keyset cursor for a window: the id of the last row already seen.
///
/// Windows return rows with an id strictly greater than the key, in
/// ascending id order — the same `(pk)` keyset discipline the Windowed
/// Fetch engine applies to tables with a primary key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RowKey(pub u64);

/// One row of the simulated table.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub id: u64,
    pub name: String,
    pub score: f64,
}

/// A simulated Large Table with a configurable row count.
///
/// Holding a [`SimulatedSource`] costs nothing proportional to the row
/// count; rows are computed on demand.
#[derive(Debug, Clone)]
pub struct SimulatedSource {
    row_count: u64,
}

impl SimulatedSource {
    /// The fixture used by benches and tests: 10,000,000 rows.
    pub fn ten_million_rows() -> Self {
        Self::with_row_count(10_000_000)
    }

    /// A simulated table with `row_count` rows, ids `1..=row_count`.
    pub fn with_row_count(row_count: u64) -> Self {
        Self { row_count }
    }

    /// Total rows the table pretends to hold. This is the stand-in for a
    /// Row Estimate: callers never count rows to learn the size.
    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    /// Serve the keyset window after `after` (`None` = from the top), at
    /// most `limit` rows, in ascending id order.
    ///
    /// Cost is proportional to the window, never to the table: a window at
    /// 10M-row depth is as cheap as the first window.
    pub fn window(&self, after: Option<RowKey>, limit: usize) -> Vec<Row> {
        let start = match after {
            Some(RowKey(key)) => key.saturating_add(1),
            None => 1,
        };
        if start > self.row_count {
            return Vec::new();
        }
        let remaining = self.row_count - start + 1;
        let count = (limit as u64).min(remaining) as usize;
        (start..start + count as u64).map(Self::row).collect()
    }

    /// Deterministic row content for an id. The same id always yields the
    /// same row, on every run and for every source size.
    fn row(id: u64) -> Row {
        Row {
            id,
            name: format!("user_{id:08}"),
            score: ((id % 10_000) as f64) / 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_content_is_derived_from_the_id() {
        let row = SimulatedSource::row(7);
        assert_eq!(row.name, "user_00000007");
        assert!((row.score - 0.07).abs() < 1e-12);
    }
}
