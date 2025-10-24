#!/bin/bash
# Run only the fast minimal Kani proofs
# Total runtime: ~10 seconds

set -e

echo "Running fast minimal Kani proofs..."
echo "===================================="
echo ""

PROOFS=(
    "i1_concrete_single_user"
    "i3_concrete_unauthorized"
    "i6_concrete_matcher"
    "deposit_concrete"
    "withdrawal_concrete"
    "i1_bounded_deficit"
    "deposit_bounded_amount"
)

PASSED=0
FAILED=0

for proof in "${PROOFS[@]}"; do
    echo "Running: $proof"
    if cargo kani -p proofs-kani --harness "$proof" 2>&1 | grep -q "VERIFICATION:- SUCCESSFUL"; then
        echo "✅ $proof PASSED"
        ((PASSED++))
    else
        echo "❌ $proof FAILED"
        ((FAILED++))
    fi
    echo ""
done

echo "===================================="
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -eq 0 ]; then
    echo "✅ All fast proofs VERIFIED!"
    exit 0
else
    echo "❌ Some proofs failed"
    exit 1
fi
