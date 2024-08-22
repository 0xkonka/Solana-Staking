use anchor_lang::prelude::*;

use instructions::*;

mod instructions;
mod state;
mod utils;
mod error;
mod events;

declare_id!("9X5si3xhU4nFVh7FkGaC3n251xoN5JBoys9AEnrfkzxh");

#[program]
pub mod staking_on_solana {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        deploy_fee: u64,
        performance_fee: u64
    ) -> Result<()> {
        instructions::initialize::handler(ctx, deploy_fee, performance_fee)
    }

    pub fn create_pool(
        ctx: Context<CreatePool>,
        pool_id: String,
        stake_fee: u16,
        unstake_fee: u16,
        initial_funding: u64,
        reward_per_slot: u64,
        duration: u16
    ) -> Result<()> {
        instructions::create_pool::handler(
            ctx,
            pool_id,
            stake_fee,
            unstake_fee,
            initial_funding,
            reward_per_slot,
            duration
        )
    }

    pub fn stake(ctx: Context<Stake>, stake_amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, stake_amount)
    }

    pub fn unstake(ctx: Context<Unstake>, unstake_amount: u64) -> Result<()> {
        instructions::unstake::handler(ctx, unstake_amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward::handler(ctx)
    }

    pub fn start_reward(ctx: Context<StartReward>) -> Result<()> {
        instructions::start_reward::handler(ctx)
    }

    pub fn stop_reward(ctx: Context<StopReward>) -> Result<()> {
        instructions::stop_reward::handler(ctx)
    }

    pub fn compound_reward(ctx: Context<CompoundReward>) -> Result<()> {
        instructions::compound_reward::handler(ctx)
    }

    pub fn pending_reward(ctx: Context<PendingReward>) -> Result<u64> {
        instructions::pending_reward::handler(ctx)
    }
}
