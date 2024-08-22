use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{ self, TokenAccount, Transfer };

use crate::state::*;
use crate::utils::*;
use crate::error::*;
use crate::events::*;

pub fn handler(ctx: Context<ClaimReward>) -> Result<()> {
    let pool_config = &ctx.accounts.pool_config_account;
    let pool_state = &mut ctx.accounts.pool_state_account;
    let user_info = &mut ctx.accounts.user_info;
    let platform = &ctx.accounts.platform;

    // Transfer Performance Fee from user to treasury
    let user_balance = ctx.accounts.claimer.to_account_info().lamports();
    require!(user_balance > platform.performance_fee, BrewStakingError::InsufficientDeployFee);

    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = system_program::Transfer {
        from: ctx.accounts.claimer.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    system_program::transfer(cpi_ctx, platform.performance_fee)?;

    let _ = update_pool(pool_config, pool_state);

    if user_info.staked_amount == 0 {
        return Ok(());
    }

    let precision_factor = get_precision_factor(pool_config);

    // Transfer the user his reward so far
    let pending =
        (user_info.staked_amount * pool_state.acc_token_per_share) / precision_factor -
        user_info.reward_debt;

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

        pool_state.total_earned = if pool_state.total_earned > pending {
            pool_state.total_earned - pending
        } else {
            0
        };
        pool_state.paid_rewards += pending;

        emit!(RewardClaim {
            claimer: ctx.accounts.claimer.key(),
            amount: pending,
        });
    }

    user_info.reward_debt =
        (user_info.staked_amount * pool_state.acc_token_per_share) / precision_factor;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    /// CHECK:
    #[account(mut)]
    pub claimer: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    #[account(mut)]
    pub user_info: Account<'info, UserInfo>,

    pub pool_config_account: Account<'info, PoolConfig>,

    #[account(mut)]
    pub pool_state_account: Account<'info, PoolState>,

    pub platform: Account<'info, PlatformInfo>,

    #[account(mut)]
    pub user_reward_token_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_reward_token_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, token::Token>,

    pub system_program: Program<'info, System>,
}
