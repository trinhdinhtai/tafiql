# Performance Budget with CI hard gates

The Performance Budget is enforced, not aspirational: cold start < 500ms (warm < 250ms), idle RAM < 100MB with one connection, Result Grid RAM < 300MB and flat while scrolling a simulated 10M-row table, 60fps scroll. Criterion benchmarks plus a simulated-grid memory test fail CI when a ceiling is breached; startup timing is tracked manually because CI timer noise makes it unreliable as a gate. Any change to the ceilings requires superseding this ADR.

## Consequences

- Budget regressions block merges the same way test failures do.
- Startup numbers are reviewed per release rather than per commit.
