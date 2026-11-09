# tafiql

A native database client built in Rust + GPUI, defined by three promises: instant startup, low memory, and smooth scrolling of very large result sets.

## Language

**Large Table**:
A table or query result of up to ~10 million rows whose size is not known before opening; the client must open it instantly regardless of size.
_Avoid_: huge table, big table, dataset

**Windowed Fetch**:
The guarantee that a Large Table is never loaded whole: only the rows needed for the current view are retrieved.
_Avoid_: lazy loading, pagination, infinite scroll

**Result Grid**:
The virtualized spreadsheet-like view of the rows and columns returned by a query.
_Avoid_: table view, data table, result table

**Performance Budget**:
The agreed ceilings on startup time, memory, and scroll smoothness that every change must respect; measured on Windows release builds.
_Avoid_: SLA, requirements

**Connection Profile**:
A saved description of how to reach a database (host, port, user, database name, sslmode); its password is never part of the profile and lives in the OS keyring.
_Avoid_: connection, saved connection, datasource

**Connection**:
A live, authenticated session to a database, opened from a Connection Profile.
_Avoid_: session, link

**Staged Change**:
A cell edit held for review before application; staged changes are applied together as generated SQL inside one transaction. Tables without a primary key cannot stage changes.
_Avoid_: pending edit, dirty cell, draft

**Export**:
Writing a query result to a file (CSV, JSON); guaranteed streaming — memory stays flat regardless of row count.
_Avoid_: dump, backup

**Read-Only Profile**:
A Connection Profile flagged to refuse every data-modifying statement; profiles for production databases default to read-only.
_Avoid_: safe mode, locked connection

**Row Estimate**:
The approximate row count of a Large Table, shown instantly from catalog statistics; exact counting happens only on explicit request.
_Avoid_: row count, total rows

**Cell Viewer**:
The overlay for inspecting a single oversized or structured value (long text, JSON, bytea) without widening the Result Grid column.
_Avoid_: detail popup, inspector

**Query History**:
The lazy-loaded, searchable log of executed statements.
_Avoid_: recent queries, log

**Snippet**:
A named query saved for reuse; stored locally and loaded lazily.
_Avoid_: saved query, template
