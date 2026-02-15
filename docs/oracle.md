# Oracle Rules (Pump.fun and PumpSwap)

## Objective

Provide deterministic and auditable resolution inputs for token migration and market predicates.

## Data Requirements

For each token market:

- `mint`
- `launch_source` (must map to Pump.fun)
- `migrated_to_pumpswap` (boolean)
- `migration_timestamp`
- `market_cap` and/or rule-specific metrics at close time
- `snapshot_timestamp`

## Freshness Policy

A snapshot is valid only if:

- `snapshot_timestamp` is within configured tolerance of market close
- source health checks pass
- schema checks pass

## Source Priority

1. Primary source (direct protocol/indexer feed)
2. Secondary source (independent indexer)
3. Fallback rejected if it causes ambiguity

If primary and secondary disagree on critical fields, market enters dispute mode.

## Dispute Mode

When data is inconsistent:

- market is paused for settlement
- discrepancy is recorded
- deterministic tie-break policy is applied
- final decision hash is stored for audit trail

## Minimal Audit Record

Every resolution writes:

- market id
- outcome
- input snapshot hash
- source ids
- resolver version
- settlement timestamp
