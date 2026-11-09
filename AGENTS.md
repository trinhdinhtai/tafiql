# AGENTS.md

## Agent skills

### Issue tracker

Issues live in GitHub Issues of trinhdinhtai/tafiql, via the gh CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

The five canonical roles, label string = role name. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: GLOSSARY.md + docs/adr/ at the repo root. See `docs/agents/domain.md`.

### Branch per ticket

Before starting work on any ticket, create a new branch from an up-to-date `main`. Never commit ticket work directly on `main`. Name it `<type>/<issue#>-<short-slug>`; drop the number when there is no issue.
