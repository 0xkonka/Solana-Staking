use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{ self, TokenAccount, Transfer };

use crate::state::*;
use crate::utils::*;
use crate::error::*;
use crate::events::*;

pub fn handler(ctx: Context<Stake>, stake_amount: u64) -> Result<()> {
    let pool_config = &mut ctx.accounts.pool_config_account;
    let pool_state = &mut ctx.accounts.pool_state_account;
    let user_info = &mut ctx.accounts.user_info;
    let platform = &ctx.accounts.platform;

    let clock = Clock::get()?;

    // msg!("@current slot {}", clock.slot);

    require!(
        pool_config.start_slot > 0 && pool_config.start_slot < clock.slot,
        BrewStakingError::PoolNotStarted
    );

    // Transfer Performance Fee from user to treasury
    let user_balance = ctx.accounts.staker.to_account_info().lamports();
    require!(user_balance > platform.performance_fee, BrewStakingError::InsufficientDeployFee);

    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.staker.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    system_program::transfer(cpi_ctx, platform.performance_fee)?;

    let _ = update_pool(pool_config, pool_state);

    let precision_factor = get_precision_factor(pool_config);
    // msg!("@precision_factor :  {}", precision_factor);
    // msg!("@user_info.staked_amount :  {}", user_info.staked_amount);

    // If user already staked before
    if user_info.staked_amount > 0 {
        // msg!("@@@ user staked before, claim reward");
        // Transfer the user his reward so far
        let pending =
            (user_info.staked_amount * pool_state.acc_token_per_share) / precision_factor -
            user_info.reward_debt;
        // msg!("@@@acc_token_per_share :  {}", pool_state.acc_token_per_share);
        // msg!("@@@reward_debt :  {}", user_info.reward_debt);
        // msg!("@@@pending :  {}", pending);

        if pending > 0 {
            require!(
                available_reward_tokens(pool_config, pool_state) >= pending,
                BrewStakingError::InsufficientReward
            );

            let cpi_accounts = Transfer {
                from: ctx.accounts.pool_reward_token_vault.to_account_info(),
                to: ctx.accounts.user_reward_token_vault.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, pending)?;

            pool_state.reward_amount -= pending;

            // msg!("@@@total_earned before :  {}", pool_state.total_earned);
            pool_state.total_earned = if pool_state.total_earned > pending {
                pool_state.total_earned - pending
            } else {
                0
            };
            // msg!("@@@total_earned after :  {}", pool_state.total_earned);
            // msg!("@@@paid_rewards before :  {}", pool_state.paid_rewards);
            pool_state.paid_rewards += pending;
            // msg!("@@@paid_rewards after :  {}", pool_state.paid_rewards);
            emit!(RewardClaim {
                claimer: ctx.accounts.staker.key(),
                amount: pending,
            });
        }
    }

    // Transfer Token from staker to pool account
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_stake_token_vault.to_account_info(),
        to: ctx.accounts.pool_stake_token_vault.to_account_info(),
        authority: ctx.accounts.staker.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, stake_amount)?;

    // Transfer stake fee from pool to pool owner
    let stake_fee = (stake_amount * (pool_config.stake_fee as u64)) / PERCENT_PRECISION;

    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_stake_token_vault.to_account_info(),
        to: ctx.accounts.creator_stake_token_vault.to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, stake_fee)?;

    // Update user and pool info
    msg!("@stake_amount :  {}", stake_amount);
    msg!("@stake_fee :  {}", stake_fee);
    let real_amount = stake_amount - stake_fee;

    user_info.staked_amount += real_amount;
    msg!("@user_info.staked_amount :  {}", user_info.staked_amount);
    user_info.reward_debt =
        (user_info.staked_amount * pool_state.acc_token_per_share) / precision_factor;
    msg!("@pool_state.acc_token_per_share :  {}", pool_state.acc_token_per_share);
    msg!("@user_info.reward_debt :  {}", user_info.reward_debt);
    pool_state.total_staked += real_amount;
    msg!("@pool_state.total_staked :  {}", pool_state.total_staked);
    emit!(Deposit {
        staker: ctx.accounts.staker.key(),
        amount: real_amount,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        init_if_needed,
        payer = staker,
        space = USER_INFO_SIZE,
        seeds = [pool_config_account.key().as_ref(), staker.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub staker: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    pub platform: Account<'info, PlatformInfo>,

    #[account(mut)]
    pub pool_config_account: Account<'info, PoolConfig>,

    #[account(mut)]
    pub pool_state_account: Account<'info, PoolState>,

    #[account(mut)]
    pub user_stake_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_reward_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_stake_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_reward_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub creator_stake_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub treasury_stake_token_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, token::Token>,
}

// impl<'info> Stake<'info> {
//     fn into_transfer_to_pool_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
//         let cpi_accounts = Transfer {
//             from: self.staker_stake_token_vault.to_account_info(),
//             to: self.pool_stake_token_vault.to_account_info(),
//             authority: self.staker.to_account_info(),
//         };
//         CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
//     }
// }
