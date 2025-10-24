#!/bin/bash
echo "=== Testing All Minimal Proofs ===" 
for proof in i1_concrete_single_user i3_concrete_unauthorized i6_concrete_matcher deposit_concrete withdrawal_concrete i1_bounded_deficit deposit_bounded_amount; do
    echo ""
    echo "Testing: $proof"
    timeout 60 cargo kani -p proofs-kani --harness $proof 2>&1 | grep -E "(VERIFICATION|Verification Time)" | tail -2
done
