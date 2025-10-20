//! Commit fill instruction - v0 single-instruction orderbook interaction

use crate::state::{SlabState, FillReceipt};
use percolator_common::*;
use pinocchio::{account_info::AccountInfo, msg, pubkey::Pubkey};

/// Side of the order
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy = 0,
    Sell = 1,
}

/// Process commit_fill instruction (v0 - atomic fill)
///
/// This is the single CPI endpoint for v0. Router calls this to fill orders.
///
/// # Arguments
/// * `slab` - The slab state account
/// * `receipt_account` - Account to write fill receipt
/// * `router_signer` - Router authority (must match slab.header.router_id)
/// * `side` - Buy or Sell
/// * `qty` - Desired quantity (1e6 scale, positive)
/// * `limit_px` - Worst acceptable price (1e6 scale)
///
/// # Returns
/// * Writes FillReceipt to receipt_account
/// * Updates slab state (book, seqno, quote_cache)
pub fn process_commit_fill(
    slab: &mut SlabState,
    receipt_account: &AccountInfo,
    router_signer: &Pubkey,
    side: Side,
    qty: i64,
    limit_px: i64,
) -> Result<(), PercolatorError> {
    // Verify router authority
    if &slab.header.router_id != router_signer {
        msg!("Error: Invalid router signer");
        return Err(PercolatorError::Unauthorized);
    }

    // Capture seqno at start
    let seqno_start = slab.header.seqno;

    // TODO: Match against book, respect limit_px
    // For now, stub implementation
    let filled_qty = 0i64;
    let vwap_px = 0i64;
    let notional = 0i64;
    let fee = 0i64;

    // Write receipt
    let receipt = unsafe { percolator_common::borrow_account_data_mut::<FillReceipt>(receipt_account)? };
    receipt.write(seqno_start, filled_qty, vwap_px, notional, fee);

    // Increment seqno (book changed)
    slab.header.increment_seqno();

    // TODO: Update quote_cache

    msg!("CommitFill executed successfully");
    Ok(())
}
