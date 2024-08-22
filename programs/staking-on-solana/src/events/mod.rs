use anchor_lang::prelude::*;

#[event]
pub struct NewStartAndEndSlots {
    pub start_slot: u64,
    pub end_slot: u64,
}

#[event]
pub struct RewardsStop {
    pub end_slot: u64,
}

#[event]
pub struct Deposit {
    pub staker: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RewardClaim {
    pub claimer: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Compound {
    pub compounder: Pubkey,
    pub amount: u64,
}
// #[event]
// pub struct ChangeAuthorityEvent {
//     pub state: Pubkey,
//     pub admin_change: Option<PubkeyValueChange>,
//     pub validator_manager_change: Option<PubkeyValueChange>,
//     pub operational_sol_account_change: Option<PubkeyValueChange>,
//     pub treasury_msol_account_change: Option<PubkeyValueChange>,
//     pub pause_authority_change: Option<PubkeyValueChange>,
// }

// #[event]
// pub struct ConfigLpEvent {
//     pub state: Pubkey,
//     pub min_fee_change: Option<FeeValueChange>,
//     pub max_fee_change: Option<FeeValueChange>,
//     pub liquidity_target_change: Option<U64ValueChange>,
//     pub treasury_cut_change: Option<FeeValueChange>,
// }

// #[event]
// pub struct ConfigMarinadeEvent {
//     pub state: Pubkey,
//     pub rewards_fee_change: Option<FeeValueChange>,
//     pub slots_for_stake_delta_change: Option<U64ValueChange>,
//     pub min_stake_change: Option<U64ValueChange>,
//     pub min_deposit_change: Option<U64ValueChange>,
//     pub min_withdraw_change: Option<U64ValueChange>,
//     pub staking_sol_cap_change: Option<U64ValueChange>,
//     pub liquidity_sol_cap_change: Option<U64ValueChange>,
//     pub withdraw_stake_account_enabled_change: Option<BoolValueChange>,
//     pub delayed_unstake_fee_change: Option<FeeCentsValueChange>,
//     pub withdraw_stake_account_fee_change: Option<FeeCentsValueChange>,
//     pub max_stake_moved_per_epoch_change: Option<FeeValueChange>,
// }

// // TODO: ConfigValidatorSystemEvent?

// #[event]
// pub struct InitializeEvent {
//     pub state: Pubkey,
//     pub params: InitializeData,
//     pub stake_list: Pubkey,
//     pub validator_list: Pubkey,
//     pub msol_mint: Pubkey,
//     pub operational_sol_account: Pubkey,
//     pub lp_mint: Pubkey,
//     pub lp_msol_leg: Pubkey,
//     pub treasury_msol_account: Pubkey,
// }

// #[event]
// pub struct EmergencyPauseEvent {
//     pub state: Pubkey,
// }

// #[event]
// pub struct ResumeEvent {
//     pub state: Pubkey,
// }

// #[event]
// pub struct ReallocValidatorListEvent {
//     pub state: Pubkey,
//     pub count: u32,
//     pub new_capacity: u32,
// }

// #[event]
// pub struct ReallocStakeListEvent {
//     pub state: Pubkey,
//     pub count: u32,
//     pub new_capacity: u32,
// }
