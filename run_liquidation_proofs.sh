#!/bin/bash
echo "=== Testing All 13 Liquidation Proofs ===" 
for i in {1..13}; do
    case $i in
        1) proof="l1_progress_if_any_liquidatable" ;;
        2) proof="l2_noop_when_none_liquidatable" ;;
        3) proof="l3_liquidatable_count_never_increases" ;;
        4) proof="l4_only_liquidatable_account_is_touched" ;;
        5) proof="l5_non_interference_unrelated_accounts" ;;
        6) proof="l6_authorization_required_for_liquidation" ;;
        7) proof="l7_conservation_preserved_by_liquidation" ;;
        8) proof="l8_principal_never_cut_by_liquidation" ;;
        9) proof="l9_no_new_liquidatables_under_snapshot_prices" ;;
        10) proof="l10_admissible_selection_when_any_exist" ;;
        11) proof="l11_atomic_progress_or_noop" ;;
        12) proof="l12_socialize_then_liquidate_does_not_increase_liq_count" ;;
        13) proof="l13_withdraw_pnl_no_new_liquidation_same_snapshot" ;;
    esac
    echo ""
    echo "L$i: $proof"
    timeout 120 cargo kani -p proofs-kani --harness $proof 2>&1 | grep -E "(VERIFICATION|Verification Time)" | tail -2
done
