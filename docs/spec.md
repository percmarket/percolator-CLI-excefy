# Prediction Market Spec

## 1. Scope

This engine defines binary prediction markets for Pump.fun-origin tokens that have already migrated to PumpSwap.

Out of scope:

- Frontend/UI
- Wallet UX
- Marketing pages
- Non-migrated tokens

## 2. Market Type

- Binary outcomes: `YES` or `NO`
- Single settlement event per market
- Fixed open and close timestamps
- Deterministic resolution rule at close

## 3. Eligibility

A market can be created only if the target token satisfies:

1. Origin traceable to Pump.fun launch metadata
2. Migration status confirmed as `migrated_to_pumpswap = true`
3. Oracle source reachable and freshness checks pass

## 4. Capital and Profit Classes

Each participant has two accounting components:

- `capital`: principal claim
- `profit`: winning upside claim

Settlement always prioritizes capital safety before profit conversion.

## 5. Coverage Ratio `h`

Let:

- `V` = vault funds available for settlement
- `C_tot` = aggregate capital due to winning side
- `P_pos_tot` = aggregate positive profit claims

Then:

`Residual = max(0, V - C_tot)`

`h = min(Residual, P_pos_tot) / P_pos_tot` (if `P_pos_tot > 0`, else `h = 1`)

Final payout per winner:

`payout_i = capital_i + floor(profit_i * h)`

## 6. Resolution

Resolution is deterministic:

1. Freeze market state at close timestamp
2. Fetch oracle snapshot within freshness window
3. Evaluate predicate
4. Mark outcome (`YES` or `NO`)
5. Settle all claims

## 7. Safety Goals

- No over-withdrawal beyond vault value
- No unbounded payout promise
- Deterministic replay from snapshots
- Auditable settlement transcript
