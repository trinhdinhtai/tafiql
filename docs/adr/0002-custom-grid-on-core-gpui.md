# Hand-rolled Result Grid on core gpui (no gpui-kit)

The Result Grid is tafiql's differentiator, so we build it as a custom gpui Element with manual row and column virtualization, on `gpui` 0.2.x pinned from crates.io (not git, not gpui-kit), bridged to a background tokio runtime via `gpui-tokio-bridge`. The rest of the UI also uses core gpui primitives.

## Considered Options

- gpui-kit/gpui-component's Table — rejected: pre-1.0, pins its own gpui version lifecycle outside our control, and is unproven at 10M rows; delegating the core competency to it contradicts the perf-first positioning.
- Hybrid (gpui-kit chrome + custom grid) — rejected: two gpui version domains in one process.

## Consequences

- Largest single chunk of M1 engineering work.
- gpui pre-1.0 churn is contained by the crates.io pin and by keeping all gpui usage inside `tafiql-app` (ADR 0003).
