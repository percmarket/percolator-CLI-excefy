//! Prediction market primitives built on Percolator-style payout safety.
//!
//! Scope:
//! - Binary markets (`YES` / `NO`)
//! - Only tokens that have already migrated from Pump.fun to PumpSwap
//! - Deterministic settlement with bounded payout via global ratio `h`

#![allow(clippy::module_name_repetitions)]

use core::cmp::min;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Yes,
    No,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarketRule {
    MarketCapAtCloseAtLeast { target_quote_units: u128 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenStatus {
    pub mint: [u8; 32],
    pub migrated_to_pumpswap: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TokenSnapshot {
    pub mint: [u8; 32],
    pub migrated_to_pumpswap: bool,
    pub market_cap_quote_units: u128,
    pub snapshot_slot: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Market {
    pub market_id: u64,
    pub token_mint: [u8; 32],
    pub created_slot: u64,
    pub close_slot: u64,
    pub rule: MarketRule,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pools {
    pub yes_capital: u128,
    pub no_capital: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settlement {
    pub outcome: Outcome,
    pub winner_capital_total: u128,
    pub loser_capital_total: u128,
    pub residual: u128,
    pub profit_claim_total: u128,
    pub h_num: u128,
    pub h_den: u128,
    pub winner_profit_paid: u128,
    pub winner_payout_total: u128,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PredictionError {
    TokenNotMigrated,
    InvalidCloseSlot,
    SnapshotBeforeClose,
    SnapshotTokenMismatch,
    SnapshotNotMigrated,
    EmptyWinnerSide,
    MathOverflow,
}

pub fn create_market(
    token: TokenStatus,
    market_id: u64,
    created_slot: u64,
    close_slot: u64,
    rule: MarketRule,
) -> Result<Market, PredictionError> {
    if !token.migrated_to_pumpswap {
        return Err(PredictionError::TokenNotMigrated);
    }
    if close_slot <= created_slot {
        return Err(PredictionError::InvalidCloseSlot);
    }

    Ok(Market {
        market_id,
        token_mint: token.mint,
        created_slot,
        close_slot,
        rule,
    })
}

pub fn resolve_outcome(
    market: &Market,
    snapshot: &TokenSnapshot,
) -> Result<Outcome, PredictionError> {
    if snapshot.snapshot_slot < market.close_slot {
        return Err(PredictionError::SnapshotBeforeClose);
    }
    if snapshot.mint != market.token_mint {
        return Err(PredictionError::SnapshotTokenMismatch);
    }
    if !snapshot.migrated_to_pumpswap {
        return Err(PredictionError::SnapshotNotMigrated);
    }

    let outcome = match market.rule {
        MarketRule::MarketCapAtCloseAtLeast { target_quote_units } => {
            if snapshot.market_cap_quote_units >= target_quote_units {
                Outcome::Yes
            } else {
                Outcome::No
            }
        }
    };
    Ok(outcome)
}

/// Settle market-wide payouts using Percolator-style bounded profit conversion.
///
/// `available_vault_funds` can be lower than total pooled capital in stress
/// scenarios. Senior winner capital is paid first; winner profit is paid
/// pro-rata with ratio `h`.
pub fn settle_market(
    market: &Market,
    snapshot: &TokenSnapshot,
    pools: Pools,
    available_vault_funds: u128,
) -> Result<Settlement, PredictionError> {
    let outcome = resolve_outcome(market, snapshot)?;

    let (winner_capital_total, loser_capital_total) = match outcome {
        Outcome::Yes => (pools.yes_capital, pools.no_capital),
        Outcome::No => (pools.no_capital, pools.yes_capital),
    };

    if winner_capital_total == 0 {
        return Err(PredictionError::EmptyWinnerSide);
    }

    // Potential positive profit for winners in a binary market
    let profit_claim_total = loser_capital_total;

    let residual = available_vault_funds.saturating_sub(winner_capital_total);
    let funded_profit = min(residual, profit_claim_total);

    let (h_num, h_den) = if profit_claim_total == 0 {
        (1, 1)
    } else {
        (funded_profit, profit_claim_total)
    };

    let winner_payout_total = winner_capital_total
        .checked_add(funded_profit)
        .ok_or(PredictionError::MathOverflow)?;

    Ok(Settlement {
        outcome,
        winner_capital_total,
        loser_capital_total,
        residual,
        profit_claim_total,
        h_num,
        h_den,
        winner_profit_paid: funded_profit,
        winner_payout_total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mint(byte: u8) -> [u8; 32] {
        [byte; 32]
    }

    #[test]
    fn rejects_non_migrated_token_on_market_creation() {
        let token = TokenStatus {
            mint: mint(7),
            migrated_to_pumpswap: false,
        };

        let err = create_market(
            token,
            1,
            100,
            200,
            MarketRule::MarketCapAtCloseAtLeast {
                target_quote_units: 1_000_000,
            },
        )
        .unwrap_err();

        assert_eq!(err, PredictionError::TokenNotMigrated);
    }

    #[test]
    fn full_coverage_settlement_pays_full_profit() {
        let token = TokenStatus {
            mint: mint(1),
            migrated_to_pumpswap: true,
        };
        let market = create_market(
            token,
            42,
            100,
            200,
            MarketRule::MarketCapAtCloseAtLeast {
                target_quote_units: 1_000_000,
            },
        )
        .unwrap();
        let snapshot = TokenSnapshot {
            mint: mint(1),
            migrated_to_pumpswap: true,
            market_cap_quote_units: 2_000_000,
            snapshot_slot: 201,
        };
        let pools = Pools {
            yes_capital: 100,
            no_capital: 40,
        };

        let settlement = settle_market(&market, &snapshot, pools, 140).unwrap();
        assert_eq!(settlement.outcome, Outcome::Yes);
        assert_eq!(settlement.h_num, 40);
        assert_eq!(settlement.h_den, 40);
        assert_eq!(settlement.winner_payout_total, 140);
    }

    #[test]
    fn stressed_settlement_applies_haircut() {
        let token = TokenStatus {
            mint: mint(2),
            migrated_to_pumpswap: true,
        };
        let market = create_market(
            token,
            7,
            10,
            20,
            MarketRule::MarketCapAtCloseAtLeast {
                target_quote_units: 5_000_000,
            },
        )
        .unwrap();
        let snapshot = TokenSnapshot {
            mint: mint(2),
            migrated_to_pumpswap: true,
            market_cap_quote_units: 6_000_000,
            snapshot_slot: 21,
        };
        let pools = Pools {
            yes_capital: 100,
            no_capital: 80,
        };

        // Only 130 units are available, so winner profit is partially funded.
        let settlement = settle_market(&market, &snapshot, pools, 130).unwrap();
        assert_eq!(settlement.h_num, 30);
        assert_eq!(settlement.h_den, 80);
        assert_eq!(settlement.winner_profit_paid, 30);
        assert_eq!(settlement.winner_payout_total, 130);
    }

    #[test]
    fn never_pays_more_than_available_vault() {
        let token = TokenStatus {
            mint: mint(3),
            migrated_to_pumpswap: true,
        };
        let market = create_market(
            token,
            99,
            5,
            15,
            MarketRule::MarketCapAtCloseAtLeast {
                target_quote_units: 10_000,
            },
        )
        .unwrap();
        let snapshot = TokenSnapshot {
            mint: mint(3),
            migrated_to_pumpswap: true,
            market_cap_quote_units: 100,
            snapshot_slot: 16,
        };
        let pools = Pools {
            yes_capital: 70,
            no_capital: 30,
        };
        let available = 20;

        let settlement = settle_market(&market, &snapshot, pools, available).unwrap();
        assert!(settlement.winner_payout_total <= available);
    }
}


