# Model to Production Code Mapping

This document maps formally verified functions in `model_safety` to their usage sites in production Solana programs.

## ✅ Integration Status

- **model_safety**: ✅ Now `no_std` compatible
- **Router dependency**: ✅ Added to `programs/router/Cargo.toml`
- **Proofs**: ✅ 47 total (34 existing + 13 liquidation)

## Critical: Use Verified Functions

**⚠️ IMPORTANT**: Production code MUST use `model_safety` functions where indicated below. This is the only way the formal proofs apply to production.

---

## 1. Safe Arithmetic (model_safety/math.rs)

### ✅ MUST USE: Saturating Arithmetic

**Location**: `programs/router/src/state/*.rs`

**Replace raw arithmetic with:**

```rust
use model_safety::math::*;

// ❌ DON'T: Raw arithmetic (can overflow/underflow)
let x = a + b;
let y = c - d;

// ✅ DO: Use verified saturating arithmetic
let x = add_u128(a, b);       // Saturates at u128::MAX
let y = sub_u128(c, d);       // Saturates at 0
let z = mul_u128(a, b);       // Saturates at u128::MAX
let w = div_u128(a, b);       // Returns 0 if b=0 (safe)

// For signed arithmetic:
let s1 = add_i128(x, y);
let s2 = sub_i128(x, y);
let pos = clamp_pos_i128(pnl);  // max(0, pnl)
let i = u128_to_i128(x);        // Safe conversion
```

**Why**: Prevents overflow/underflow bugs that could break conservation (I2).

**Files to update**:
- `programs/router/src/state/pnl_vesting.rs` (ALL arithmetic)
- `programs/router/src/state/insurance.rs` (fund calculations)
- `programs/router/src/state/portfolio.rs` (balance updates)

---

## 2. PnL Warm-up / Throttling (model_safety/warmup.rs)

### ✅ MUST USE: Withdrawable PnL Calculation

**Production location**: `programs/router/src/state/pnl_vesting.rs`

**Current production code** uses exponential vesting:
```rust
// Current: 1 - exp(-Δ/τ) with Taylor series
pub fn one_minus_exp_neg(dt: u64, tau: u64) -> i128 { ... }
```

**Verified model** uses linear vesting:
```rust
use model_safety::warmup::*;

// ✅ Verified function (I5: Throttle Safety proven)
pub fn withdrawable_pnl(
    account: &Account,
    steps_elapsed: u32,
    slope_per_step: u128,
) -> u128 {
    let cap = steps_elapsed as u128 * slope_per_step;
    min_u128(cap, clamp_pos_i128(account.pnl_ledger))
}
```

**Decision needed**:
1. **Option A**: Replace exponential vesting with linear (use verified function directly)
2. **Option B**: Keep exponential but add linear as a fallback/safety check
3. **Option C**: Prove exponential vesting separately (harder)

**Recommendation**: Option A for v0, then enhance model for exponential in v1.

**Files to update**:
- `programs/router/src/state/pnl_vesting.rs:one_minus_exp_neg` → Replace or wrap

---

## 3. Loss Socialization / Haircuts (model_safety/transitions.rs)

### ✅ MUST USE: Socialization Logic Core

**Production location**: `programs/router/src/state/pnl_vesting.rs:calculate_haircut_fraction`

**Current production**:
```rust
pub fn calculate_haircut_fraction(
    shortfall: u128,
    total_positive_pnl: u128,
    max_haircut_bps: u16,
) -> i128 {
    // Returns multiplicative index (FP_ONE - haircut)
    // Uses global haircut index pattern
}
```

**Verified model**:
```rust
use model_safety::transitions::socialize_losses;
use model_safety::helpers::sum_effective_winners;

// ✅ Verified (I1: Principal untouched, I4: Winners-only bounded haircut)
pub fn socialize_losses(mut s: State, deficit: u128) -> State {
    // 1. Calculate total effective winners
    // 2. Compute haircut fraction = min(deficit / total, 1.0)
    // 3. Apply proportionally to each winner
    // 4. Skip losers (I4)
    // 5. Never touch principal (I1)
}
```

**Integration strategy**:

```rust
// In production, wrap the verified logic:
pub fn apply_global_haircut(
    router_state: &mut RouterState,
    deficit: u128,
) -> Result<()> {
    // 1. Convert router state to model_safety::State
    let model_state = router_state.to_model();

    // 2. Call verified function
    let new_model = model_safety::transitions::socialize_losses(model_state, deficit);

    // 3. Apply changes back to router state
    router_state.apply_from_model(&new_model);

    // 4. Update global index for lazy application
    router_state.global_haircut.pnl_index = /* compute from change */;

    Ok(())
}
```

**Files to update**:
- `programs/router/src/state/pnl_vesting.rs` - Add wrapper using verified socialization
- Ensure production haircut matches verified algorithm

---

## 4. Principal vs PnL Separation (CRITICAL)

### ✅ MUST ENFORCE: Principal Inviolability (I1)

**Verified invariant**:
```rust
// I1: Principal NEVER decreases except via user withdrawal
∀ operations except withdraw_principal:
    after.principal == before.principal
```

**Production enforcement**:

```rust
// ❌ NEVER DO THIS:
user.principal -= haircut_amount;  // VIOLATES I1!

// ✅ ALWAYS DO THIS:
// Loss socialization ONLY affects PnL:
user.pnl_ledger -= haircut_amount;  // OK
user.principal unchanged;            // REQUIRED
```

**Audit checklist for ALL production functions**:

- [ ] `apply_haircut()` - ✅ Only touches PnL
- [ ] `settle_trade()` - ✅ Only touches PnL
- [ ] `withdraw_pnl()` - ✅ Only touches PnL (with throttle)
- [ ] `withdraw_principal()` - ⚠️ OK to reduce principal (user-initiated only)
- [ ] `deposit()` - ✅ Only increases principal

**Files to audit**:
- `programs/router/src/state/pnl_vesting.rs` - ALL functions touching user balances
- `programs/router/src/state/portfolio.rs` - Portfolio mutations

---

## 5. Conservation (I2)

### ✅ MUST MAINTAIN: Vault Balance Equation

**Verified invariant**:
```rust
// I2: Vault equals sum of principals + positive PnL + insurance
vault == Σ(principal) + Σ(max(0, pnl)) + insurance_fund - fees
```

**Production code MUST**:

```rust
// EVERY operation that changes user balances MUST update vault consistently

// ✅ Deposit:
user.principal += amount;
vault += amount;  // REQUIRED

// ✅ Withdraw:
user.principal -= amount;
vault -= amount;  // REQUIRED

// ✅ Settle trade (profit):
user.pnl_ledger += realized_pnl;
vault += realized_pnl;  // REQUIRED

// ✅ Settle trade (loss):
user.pnl_ledger -= realized_loss;
vault -= realized_loss;  // REQUIRED

// ✅ Socialization:
// Vault unchanged (redistributes existing PnL)
// sum(pnl) decreases but vault stays same
```

**Add conservation check**:

```rust
pub fn check_conservation(state: &RouterState) -> bool {
    use model_safety::helpers::conservation_ok;

    let model = state.to_model();
    conservation_ok(&model)
}

// Call this in tests and optionally in production (governance)
#[cfg(test)]
mod tests {
    #[test]
    fn test_conservation_maintained() {
        let mut state = create_test_state();
        assert!(check_conservation(&state));

        // Apply operation
        apply_operation(&mut state);

        // MUST still hold
        assert!(check_conservation(&state));
    }
}
```

---

## 6. Liquidation (NEW - From liquidation.rs)

### ✅ MUST USE: Liquidation Helpers

**Production needs**:
- Margin health checks
- Liquidation eligibility
- Position closing logic

**Use verified functions**:

```rust
use model_safety::helpers::{is_liquidatable, liquidatable_count};
use model_safety::transitions::{liquidate_one, liquidate_account};

// ✅ Check if account needs liquidation:
pub fn should_liquidate(
    user: &User,
    prices: &Prices,
    params: &Params,
) -> bool {
    let account = user.to_model_account();
    model_safety::helpers::is_liquidatable(&account, prices, params)
}

// ✅ Liquidate account (verified to preserve I1, I2):
pub fn execute_liquidation(
    state: &mut RouterState,
    user_id: usize,
    prices: &Prices,
) -> Result<()> {
    let model = state.to_model();
    let after = model_safety::transitions::liquidate_account(
        model,
        user_id,
        prices,
    );
    state.apply_from_model(&after);
    Ok(())
}
```

**Liquidation proofs guarantee**:
- L1: Progress (count decreases if any liquidatable)
- L5: Principal unchanged (I1 extends to liquidation)
- L6: Authorization required (I3 extends to liquidation)
- L8: Principal inviolability during liquidation

---

## 7. Authorization (I3)

### ✅ MUST ENFORCE: Authorized Router Only

**Verified invariant**:
```rust
// I3: Only authorized router can mutate balances
if !state.authorized_router {
    // All operations are no-ops
    return state;
}
```

**Production enforcement**:

```rust
// In Solana programs, use account ownership checks:

pub fn apply_haircut(ctx: Context<ApplyHaircut>, ...) -> Result<()> {
    // ✅ Verify signer is authorized router
    require_keys_eq!(
        ctx.accounts.router_authority.key(),
        ctx.accounts.router_state.authority,
        ErrorCode::Unauthorized
    );

    // Only then proceed with balance mutations
    ...
}
```

**Files to audit**:
- ALL instruction handlers in `programs/router/src/instructions/`
- Ensure every balance-mutating instruction checks authority

---

## 8. Matcher Isolation (I6)

### ✅ MUST ENFORCE: Matchers Cannot Move Funds

**Verified invariant**:
```rust
// I6: Matcher operations don't change balances
matcher_noise(state) == state  (for all balance fields)
```

**Production enforcement**:

```rust
// Matchers (order books/AMMs) CANNOT:
// ❌ Change user.principal
// ❌ Change user.pnl_ledger
// ❌ Change vault

// Matchers CAN:
// ✅ Update order book state
// ✅ Generate match quotes
// ✅ Emit events

// In Solana:
// - Matcher programs CANNOT be signers for user accounts
// - Router mediates ALL fund movements
// - Matchers only provide CPI calls with quotes, not transfers
```

---

## Conversion Functions Needed

To use `model_safety` in production, add these conversion helpers:

```rust
// In programs/router/src/state/mod.rs

use model_safety;

impl RouterState {
    /// Convert to verified model state
    pub fn to_model(&self) -> model_safety::State {
        let mut users = arrayvec::ArrayVec::new();

        for user in self.users.iter() {
            users.push(model_safety::Account {
                principal: user.principal,
                pnl_ledger: user.pnl_ledger,
                reserved_pnl: user.reserved_pnl,
                warmup_state: model_safety::Warmup {
                    started_at_slot: user.last_pnl_update_slot,
                    slope_per_step: self.params.pnl_withdraw_slope,
                },
                position_size: user.total_position_notional,
            });
        }

        model_safety::State {
            vault: self.vault_balance,
            insurance_fund: self.insurance_fund,
            fees_outstanding: self.accumulated_fees,
            users,
            params: model_safety::Params {
                max_users: self.params.max_users,
                withdraw_cap_per_step: self.params.withdraw_cap,
                maintenance_margin_bps: self.params.maintenance_margin_bps,
            },
            authorized_router: true,
        }
    }

    /// Apply changes from verified model back to production state
    pub fn apply_from_model(&mut self, model: &model_safety::State) {
        self.vault_balance = model.vault;
        self.insurance_fund = model.insurance_fund;
        self.accumulated_fees = model.fees_outstanding;

        for (i, model_user) in model.users.iter().enumerate() {
            if let Some(user) = self.users.get_mut(i) {
                user.principal = model_user.principal;
                user.pnl_ledger = model_user.pnl_ledger;
                user.reserved_pnl = model_user.reserved_pnl;
                user.total_position_notional = model_user.position_size;
            }
        }
    }
}
```

---

## Testing Strategy

### Unit Tests Must Use Verified Functions

```rust
#[cfg(test)]
mod tests {
    use model_safety::helpers::*;
    use model_safety::transitions::*;

    #[test]
    fn test_haircut_preserves_invariants() {
        let state = create_test_state();

        // ✅ Use verified function
        let after = socialize_losses(state.clone(), 1000);

        // ✅ Check verified invariants hold
        assert!(principals_unchanged(&state, &after));  // I1
        assert!(conservation_ok(&after));                // I2
        assert!(winners_only_haircut(&state, &after));  // I4
    }
}
```

---

## Audit Checklist

Before deploying, verify:

- [ ] All arithmetic uses `model_safety::math::*` functions
- [ ] Haircut logic uses or matches `socialize_losses`
- [ ] Principal never decreases except in `withdraw_principal`
- [ ] Every balance change updates vault (conservation)
- [ ] Authorization checked on all balance mutations
- [ ] Matchers isolated from fund movements
- [ ] Liquidation uses verified helpers
- [ ] Integration tests call `conservation_ok()` after each operation
- [ ] Conversion functions (`to_model`, `apply_from_model`) tested

---

## Summary

| Verified Component | Production Usage | Status |
|--------------------|------------------|--------|
| Safe math | Use in all arithmetic | 🟡 TODO |
| Warm-up throttle | Replace/wrap exponential vesting | 🟡 TODO |
| Socialization | Core of haircut logic | 🟡 TODO |
| Liquidation | Margin checks + execution | 🟡 TODO |
| Conservation check | Add to tests + optional runtime | 🟡 TODO |
| Authorization | Already enforced (Solana) | ✅ DONE |
| Matcher isolation | Already enforced (Solana) | ✅ DONE |

**Next steps**:
1. Add conversion functions (`to_model`, `apply_from_model`)
2. Replace arithmetic with `model_safety::math::*`
3. Wrap socialization to use verified core
4. Add conservation checks to test suite
5. Document any deviations with formal justification

**The formal proofs only cover code that uses `model_safety` functions.**
