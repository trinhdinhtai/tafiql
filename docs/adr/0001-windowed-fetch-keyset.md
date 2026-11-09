# Windowed Fetch: never materialize results

tafiql's core promise is instant open and flat memory on Large Tables (~10M rows, size unknown upfront). We decided the Result Grid only ever holds rows near the viewport, and the data layer fetches windows via keyset pagination: opening a table defaults to `ORDER BY` primary key; user sorts become keysets on `(sort column, pk)`. Tables without a primary key fall back to OFFSET windows with a visible warning and editing disabled. Totals show as a Row Estimate; exact counts run only on explicit request.

## Considered Options

- Load entire result then scroll — rejected: RAM scales with table size, violates the Performance Budget.
- OFFSET-only windows — rejected: O(depth) degradation and unstable row identity across fetches.

## Consequences

- RAM stays flat regardless of table size.
- Every future data source must be able to serve keyset windows through the Connection trait; sources that cannot (e.g. natural-order-only APIs) must degrade to OFFSET with the same warning.
