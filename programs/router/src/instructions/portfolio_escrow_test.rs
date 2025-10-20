#[cfg(test)]
mod tests {
    use crate::state::{Portfolio, Escrow};
    use pinocchio::pubkey::Pubkey;

    #[test]
    fn test_portfolio_initialization() {
        let program_id = Pubkey::default();
        let user = Pubkey::from([1; 32]);
        let bump = 255; // Use a test bump value

        // Initialize portfolio using new()
        let portfolio = Portfolio::new(program_id, user, bump);

        // Verify initial state
        assert_eq!(portfolio.router_id, program_id);
        assert_eq!(portfolio.user, user);
        assert_eq!(portfolio.equity, 0);
        assert_eq!(portfolio.im, 0);
        assert_eq!(portfolio.mm, 0);
        assert_eq!(portfolio.free_collateral, 0);
        assert_eq!(portfolio.exposure_count, 0);
        assert_eq!(portfolio.bump, bump);

        // Verify exposures are zero-initialized
        for i in 0..(percolator_common::MAX_SLABS * percolator_common::MAX_INSTRUMENTS) {
            assert_eq!(portfolio.exposures[i], (0, 0, 0));
        }
    }

    #[test]
    fn test_escrow_initialization() {
        let program_id = Pubkey::default();
        let user = Pubkey::from([1; 32]);
        let slab = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let bump = 255; // Use a test bump value

        // Initialize escrow manually
        let escrow = Escrow {
            router_id: program_id,
            slab_id: slab,
            user,
            mint,
            balance: 0,
            nonce: 0,
            frozen: false,
            bump,
            _padding: [0; 6],
        };

        // Verify initial state
        assert_eq!(escrow.router_id, program_id);
        assert_eq!(escrow.slab_id, slab);
        assert_eq!(escrow.user, user);
        assert_eq!(escrow.mint, mint);
        assert_eq!(escrow.balance, 0);
        assert_eq!(escrow.nonce, 0);
        assert!(!escrow.frozen);
        assert_eq!(escrow.bump, bump);
    }

    // Note: PDA derivation tests are skipped because they require Solana syscalls
    // which are not available in unit tests. PDA derivation is tested in the
    // existing pda.rs test module with #[cfg(target_os = "solana")]

    #[test]
    fn test_portfolio_size() {
        use core::mem::size_of;
        let actual_size = size_of::<Portfolio>();
        assert_eq!(actual_size, Portfolio::LEN);
    }

    #[test]
    fn test_escrow_size() {
        use core::mem::size_of;
        let actual_size = size_of::<Escrow>();
        assert_eq!(actual_size, Escrow::LEN);
    }
}
