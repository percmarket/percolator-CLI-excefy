# Risk and Settlement Notes

## Main Invariant

Total withdrawals from a resolved market must never exceed available vault value.

## Failure Modes Addressed

- Winner-side overcrowding
- Temporary data source inconsistency
- Profit over-promising under thin residual liquidity

## Why `h` Exists

Without `h`, a market can become insolvent when winning claims exceed realizable residual value.

With `h`, profit claims are scaled proportionally, while capital remains senior.

## Settlement Ordering

1. Validate resolution snapshot
2. Determine winning side
3. Return winner capital claims
4. Compute residual
5. Apply `h` to profit claims
6. Finalize payouts

## Determinism Requirements

Given the same snapshot and market state, settlement output must be byte-for-byte identical.

## Verification Targets

- Conservation of value
- Capital seniority
- Bounded payout under stress
- No account receives negative payout
