# Contributing

Thank you for contributing.

## Principles

- Keep changes deterministic and auditable
- Preserve payout safety invariants
- Prefer small, reviewable pull requests
- Document all rule changes in `docs/`

## Setup

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## Pull Request Checklist

- [ ] Scope is clearly described
- [ ] Tests added/updated
- [ ] `docs/spec.md` updated if behavior changed
- [ ] No unrelated refactors
- [ ] CI passes

## Commit Style

Use concise commit messages, for example:

- `engine: add deterministic market close checks`
- `oracle: validate migration status freshness`
- `docs: clarify payout ratio h semantics`
