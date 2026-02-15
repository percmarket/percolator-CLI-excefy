# Percolator Prediction Engine

Binary prediction markets for Pump.fun tokens that have migrated to PumpSwap.

Fork of [aeyakovenko/percolator](https://github.com/aeyakovenko/percolator) — repurposed from perpetual futures risk accounting to deterministic prediction market settlement.

> No market can ever pay out more value than exists in the vault.

## How It Works

Users place capital into `YES` or `NO` pools on binary outcomes for PumpSwap-migrated tokens. At market close, an oracle snapshot determines the outcome. Winners receive their capital back (senior claim) plus a share of the losing side's capital (junior claim), bounded by the global coverage ratio `h`.

### Settlement Math

```
Residual = max(0, vault_funds - winner_capital)

                min(Residual, loser_capital)
    h       =  ----------------------------
                      loser_capital

payout_i  = capital_i + floor(profit_i × h)
```

When `h = 1`, winners collect full profit. When `h < 1` (stress), profit is haircut proportionally. Capital is always returned first.

### Why This Matters

Traditional prediction markets can become insolvent under extreme winner concentration. This engine adapts Percolator's two-claim-class model to guarantee bounded payouts:

- **Capital (senior)**: always returned to winners first
- **Profit (junior)**: paid pro-rata via `h`, never exceeds available residual
- **No insolvency**: mathematically impossible to over-pay

## Token Eligibility

Only tokens satisfying all of:

1. Launched on Pump.fun
2. Successfully migrated to PumpSwap (`migrated_to_pumpswap = true`)
3. Oracle data source reachable and fresh

Non-migrated tokens are rejected at market creation.

## Market Types

Currently implemented:

| Rule | Example |
|------|---------|
| `MarketCapAtCloseAtLeast` | "Will $TOKEN exceed $1M market cap by close?" |

Planned:

- `PriceAtCloseAtLeast` — price target markets
- `VolumeInWindowAtLeast` — volume threshold markets
- `MigrationWithinWindow` — migration timing markets

## Architecture

```
src/
├── percolator.rs    # Core risk engine (from upstream)
├── i128.rs          # BPF-safe 128-bit arithmetic
└── prediction.rs    # Prediction market module
    ├── types        # Market, Pools, Settlement, TokenSnapshot
    ├── create_market()   # Eligibility-gated market creation
    ├── resolve_outcome() # Deterministic oracle resolution
    └── settle_market()   # Bounded payout with h-ratio
```

## Development

```bash
# Format
cargo fmt --all -- --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test (uses MAX_ACCOUNTS=64)
cargo test --features test

# Formal verification (requires Kani)
cargo install --locked kani-verifier
cargo kani setup
cargo kani
```

## Tests

The prediction module includes invariant-driven tests:

- **Eligibility gate**: non-migrated tokens are rejected
- **Full coverage**: when vault is solvent, `h = 1` and winners get full profit
- **Stressed settlement**: when vault is underfunded, `h < 1` and profit is haircut
- **Vault bound**: total payout never exceeds available vault funds

## Documentation

- [`docs/spec.md`](docs/spec.md) — Market rules and settlement specification
- [`docs/oracle.md`](docs/oracle.md) — Oracle data requirements and dispute handling
- [`docs/risk.md`](docs/risk.md) — Risk model and safety invariants
- [`ROADMAP.md`](ROADMAP.md) — Development phases
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — Contribution guidelines

## Roadmap

See [`ROADMAP.md`](ROADMAP.md) for full details.

**v0.1** (current): Core prediction market primitives, settlement engine, oracle schema, unit tests

**v0.2**: Multi-source oracle reconciliation, dispute flow, snapshot replay

**v0.3**: Formal verification harnesses for payout bounds and capital seniority

## References

- [aeyakovenko/percolator](https://github.com/aeyakovenko/percolator) — upstream risk engine
- Tarun Chitra, *Autodeleveraging: Impossibilities and Optimization*, arXiv:2512.01112, 2025

## License

Apache-2.0
