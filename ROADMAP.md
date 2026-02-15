# Roadmap

## v0.1 — Core Engine (in progress)

- [x] Prediction market domain types (`Market`, `Pools`, `Settlement`, `TokenSnapshot`)
- [x] Eligibility gate — only PumpSwap-migrated tokens
- [x] Deterministic settlement with coverage ratio `h`
- [x] Unit tests for invariants (eligibility, full coverage, stress, vault bound)
- [ ] Oracle schema and freshness enforcement
- [ ] Per-account payout breakdown (individual `h`-scaled profit)
- [ ] Market lifecycle state machine (open → closed → resolved → settled)

## v0.2 — Oracle Hardening

- [ ] Multi-source reconciliation (primary + secondary indexer)
- [ ] Dispute-mode flow with deterministic tie-break
- [ ] Snapshot hashing and replay tool
- [ ] Integration with Pump.fun / PumpSwap on-chain data

## v0.3 — Formal Verification

- [ ] Kani proof harnesses for payout conservation
- [ ] Kani proof harnesses for capital seniority under stress
- [ ] Regression suite for edge-case settlement scenarios

## v0.4 — Additional Market Rules

- [ ] `PriceAtCloseAtLeast` — price target markets
- [ ] `VolumeInWindowAtLeast` — volume threshold markets
- [ ] `MigrationWithinWindow` — migration timing markets
- [ ] Configurable market duration and close conditions

## Non-Goals (Current Phase)

- Frontend / UI
- Wallet integration
- Token issuance
