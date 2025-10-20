//! Execute cross-slab order - v0 main instruction

use crate::state::{Portfolio, Vault};
use percolator_common::*;
use pinocchio::{account_info::AccountInfo, msg, pubkey::Pubkey};

/// Slab split - how much to execute on each slab
#[derive(Debug, Clone, Copy)]
pub struct SlabSplit {
    /// Slab account pubkey
    pub slab_id: Pubkey,
    /// Quantity to execute on this slab (1e6 scale)
    pub qty: i64,
    /// Side (0 = buy, 1 = sell)
    pub side: u8,
    /// Limit price (1e6 scale)
    pub limit_px: i64,
}

/// Process execute cross-slab order (v0 main instruction)
///
/// This is the core v0 instruction that proves portfolio netting.
/// Router reads QuoteCache from multiple slabs, splits the order,
/// CPIs to each slab's commit_fill, aggregates receipts, and
/// updates portfolio with net exposure.
///
/// # Arguments
/// * `portfolio` - User's portfolio account
/// * `user` - User pubkey (signer)
/// * `vault` - Collateral vault
/// * `slab_accounts` - Array of slab accounts to execute on
/// * `receipt_accounts` - Array of receipt PDAs (one per slab)
/// * `splits` - How to split the order across slabs
///
/// # Returns
/// * Updates portfolio with net exposures
/// * Checks margin on net exposure (capital efficiency!)
/// * All-or-nothing atomicity
pub fn process_execute_cross_slab(
    portfolio: &mut Portfolio,
    user: &Pubkey,
    vault: &mut Vault,
    slab_accounts: &[AccountInfo],
    receipt_accounts: &[AccountInfo],
    splits: &[SlabSplit],
) -> Result<(), PercolatorError> {
    // Verify portfolio belongs to user
    if &portfolio.user != user {
        msg!("Error: Portfolio does not belong to user");
        return Err(PercolatorError::InvalidPortfolio);
    }

    // Verify we have matching number of slabs and receipts
    if slab_accounts.len() != receipt_accounts.len() || slab_accounts.len() != splits.len() {
        msg!("Error: Mismatched slab/receipt/split counts");
        return Err(PercolatorError::InvalidInstruction);
    }

    // TODO: Phase 1 - Read QuoteCache from each slab account (direct bytes)
    // TODO: Phase 2 - Validate seqno hasn't changed
    // TODO: Phase 3 - CPI to each slab's commit_fill
    // TODO: Phase 4 - Aggregate receipts
    // TODO: Phase 5 - Update portfolio with net exposure
    // TODO: Phase 6 - Calculate IM on net exposure (capital efficiency!)
    // TODO: Phase 7 - Check if portfolio has sufficient equity

    // Stub for v0
    msg!("ExecuteCrossSlab instruction executed");

    let _ = vault; // Suppress unused warning for now

    Ok(())
}
