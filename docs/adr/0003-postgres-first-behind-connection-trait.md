# PostgreSQL first, behind a Connection trait

The MVP speaks PostgreSQL only, via `tokio-postgres` running on the background tokio runtime. All database capability flows through a `Connection` trait owned by `tafiql-core`; new databases are added by implementing that trait and never by touching UI code. Code layout follows: `tafiql-core` is headless (profiles, Windowed Fetch, Staged Changes — testable and benchmarkable without a window) and `tafiql-app` holds all gpui code.

## Considered Options

- sqlx now for multi-database reach — rejected: heavier dependency, its compile-time-checked-query feature is unused here, and its abstraction doesn't model keyset windowed fetch anyway.
- Many micro-crates — rejected: premature boundaries for a pre-M1 codebase.
