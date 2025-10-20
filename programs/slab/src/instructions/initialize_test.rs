#[cfg(test)]
mod tests {
    use crate::state::{SlabHeader, SlabState};
    use pinocchio::pubkey::Pubkey;

    #[test]
    fn test_slab_header_initialization() {
        // Validate SlabHeader initialization logic
        let program_id = Pubkey::default();
        let lp_owner = Pubkey::from([1; 32]);
        let router_id = Pubkey::from([2; 32]);

        let imr = 500; // 5%
        let mmr = 250; // 2.5%
        let maker_fee = -5; // -0.05% rebate
        let taker_fee = 20; // 0.2%
        let batch_ms = 100;
        let bump = 255;

        let header = SlabHeader::new(
            program_id,
            lp_owner,
            router_id,
            imr,
            mmr,
            maker_fee,
            taker_fee,
            batch_ms,
            bump,
        );

        // Verify magic bytes and version
        assert_eq!(header.magic, *SlabHeader::MAGIC);
        assert_eq!(header.version, SlabHeader::VERSION);
        assert!(header.validate());

        // Verify parameters
        assert_eq!(header.program_id, program_id);
        assert_eq!(header.lp_owner, lp_owner);
        assert_eq!(header.router_id, router_id);
        assert_eq!(header.imr, imr);
        assert_eq!(header.mmr, mmr);
        assert_eq!(header.maker_fee, maker_fee);
        assert_eq!(header.taker_fee, taker_fee);
        assert_eq!(header.batch_ms, batch_ms);
        assert_eq!(header.bump, bump);

        // Verify anti-toxicity defaults
        assert_eq!(header.freeze_levels, 3);
        assert_eq!(header.kill_band_bps, 100);
        assert_eq!(header.as_fee_k, 50);
        assert!(header.jit_penalty_on);
        assert_eq!(header.maker_rebate_min_ms, 100);

        // Verify monotonic IDs start at 1
        assert_eq!(header.next_order_id, 1);
        assert_eq!(header.next_hold_id, 1);
        assert_eq!(header.book_seqno, 0);
    }

    // Note: PDA derivation tests are skipped because they require Solana syscalls
    // which are not available in unit tests. PDA derivation is tested in the
    // existing pda.rs test module with #[cfg(target_os = "solana")]

    #[test]
    fn test_slab_state_size() {
        // Verify SlabState size is reasonable for orderbook data
        use core::mem::size_of;
        let actual_size = size_of::<SlabState>();

        // SlabState should be large enough for accounts, orders, positions, etc.
        // It's about 7MB with current pool sizes
        const ONE_MB: usize = 1024 * 1024;
        const TEN_MB: usize = 10 * ONE_MB;

        // Should be at least 5MB for the pool data
        assert!(actual_size > 5 * ONE_MB, "SlabState too small: {} bytes", actual_size);
        // Should not exceed 10MB (Solana account size limit)
        assert!(actual_size <= TEN_MB, "SlabState too large: {} bytes", actual_size);
    }

    #[test]
    fn test_header_size_matches() {
        use core::mem::size_of;
        let actual_size = size_of::<SlabHeader>();
        assert_eq!(actual_size, SlabHeader::LEN);
    }
}
