use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    },
};

use crate::states::{
    donate_pool::{DonatePoolAccount, DONATE_POOL_PREFIX},
    reward::{RewardAccount, REWARD_PREFIX},
};

#[derive(Accounts)]
#[instruction(_donate_pool_bump:u8, _reward_bump:u8)]

pub struct ClaimRewardAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            DONATE_POOL_PREFIX.as_bytes()
        ],
        bump,
    )]
    pub donate_pool: Box<Account<'info, DonatePoolAccount>>,
    #[account(
        mut,
        seeds = [
            REWARD_PREFIX.as_bytes(),
            fee_and_rent_payer.key().as_ref(),
        ],
        bump = _reward_bump
    )]
    pub reward: Box<Account<'info, RewardAccount>>,
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_claim_reward(
    ctx: Context<ClaimRewardAccounts>,
    _donate_pool_bump: u8,
    _reward_bump: u8,
) -> Result<()> {
    // check ?? authority ?? redo ??
    let reward_account: &mut Box<Account<RewardAccount>> = &mut ctx.accounts.reward;
    let reward_amount: u64 = reward_account.reward_amount;
    reward_account.reward_amount = 0;
    **ctx
        .accounts
        .donate_pool
        .to_account_info()
        .try_borrow_mut_lamports()? -= reward_amount;
    **ctx.accounts.fee_and_rent_payer.try_borrow_mut_lamports()? += reward_amount;
    Ok(())
}
