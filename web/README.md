# Website MVP (Frontend Only)

This folder contains the standalone market UI prototype.

## Scope

- Minimalist card-grid UI (Polymarket-like layout)
- Market cards for Pump.fun tokens migrated to PumpSwap
- Aggregated pools and implied odds only
- No per-user bet history

## Data Source

Frontend tries:

1. `http://localhost:8787/markets` (real-time feed)
2. `./data/sample-markets.json` (fallback)

Only records with `migrated_to_pumpswap === true` are rendered.

## Run locally

From repository root:

```bash
cd web
python -m http.server 5500
```

Open:

- `http://localhost:5500`

## Next

- Wire to Rust prediction engine API
- Add market detail page and settlement view
- Add sorting/filters and search
