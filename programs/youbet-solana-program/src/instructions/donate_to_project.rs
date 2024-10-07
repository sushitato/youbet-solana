use anchor_lang::{
    prelude::*,
    solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    },
};

use crate::{
    error::YouBetError,
    states::{
        github::{GithubAccount, GITHUB_PREFIX},
        project::{
            ProjectAccount, ProjectUserPointAccount, PROJECT_PREFIX, PROJECT_USER_POINT_PREFIX,
        },
        reward::{RewardAccount, REWARD_PREFIX},
        task::{TaskAccount, TASK_PREFIX},
        wallet::{WalletAccount, WALLET_PREFIX},
        DonatePoolAccount, DONATE_POOL_PREFIX,
    },
};

#[derive(Accounts)]
#[instruction(
    value: u64,
    project_id: String, _project_bump:u8,
    _donate_pool_bump:u8,
    account1: Pubkey, _reward_bump1: u8, _project_user_point_bump1: u8,
    account2: Pubkey, _reward_bump2: u8, _project_user_point_bump2: u8,
    account3: Pubkey, _reward_bump3: u8, _project_user_point_bump3: u8,
)]
pub struct DonateToProjectAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,

    #[account(
        seeds = [
            PROJECT_PREFIX.as_bytes(),
            project_id.as_ref(),
        ],
        bump = _project_bump,
    )]
    pub project: Box<Account<'info, ProjectAccount>>,

    #[account(
        mut,
        seeds = [
            DONATE_POOL_PREFIX.as_bytes()
        ],
        bump = _donate_pool_bump,
    )]
    pub donate_pool: Box<Account<'info, DonatePoolAccount>>,

    #[account(
        seeds = [
            PROJECT_USER_POINT_PREFIX.as_bytes(),
            project_id.as_ref(),
            account1.as_ref(),
        ],
        bump = _project_user_point_bump1,
    )]
    pub project_user_point1: Option<Box<Account<'info, ProjectUserPointAccount>>>,

    #[account(
        seeds = [
            PROJECT_USER_POINT_PREFIX.as_bytes(),
            project_id.as_ref(),
            account2.as_ref(),
        ],
        bump = _project_user_point_bump2,
    )]
    pub project_user_point2: Option<Box<Account<'info, ProjectUserPointAccount>>>,

    #[account(
        seeds = [
            PROJECT_USER_POINT_PREFIX.as_bytes(),
            project_id.as_ref(),
            account3.as_ref(),
        ],
        bump = _project_user_point_bump3,
    )]
    pub project_user_point3: Option<Box<Account<'info, ProjectUserPointAccount>>>,

    #[account(
        mut,
        seeds = [
            REWARD_PREFIX.as_bytes(),
            account1.as_ref(),
        ],
        bump = _reward_bump1,
    )]
    pub reward1: Option<Box<Account<'info, RewardAccount>>>,

    #[account(
        mut,
        seeds = [
            REWARD_PREFIX.as_bytes(),
            account2.as_ref(),
        ],
        bump = _reward_bump2
    )]
    pub reward2: Option<Box<Account<'info, RewardAccount>>>,

    #[account(
        mut,
        seeds = [
            REWARD_PREFIX.as_bytes(),
            account3.as_ref(),
        ],
        bump = _reward_bump3
    )]
    pub reward3: Option<Box<Account<'info, RewardAccount>>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_donate_to_project(
    ctx: Context<DonateToProjectAccounts>,
    value: u64,
    project_id: String,
    _project_bump: u8,
    _donate_pool_bump: u8,

    account1: Pubkey,
    _reward_bump1: u8,
    _project_user_point_bump1: u8,
    account2: Pubkey,
    _reward_bump2: u8,
    _project_user_point_bump2: u8,
    account3: Pubkey,
    _reward_bump3: u8,
    _project_user_point_bump3: u8,
) -> Result<()> {
    // check ?? authority ?? redo ??
    if value <= 0 {
        return Err(YouBetError::ValueLessZero.into());
    }
    let project: &mut Box<Account<ProjectAccount>> = &mut ctx.accounts.project;
    let total_project_points = project.total_project_points;
    if let Some(reward) = &mut ctx.accounts.reward1 {
        if let Some(point) = &ctx.accounts.project_user_point1 {
            reward.reward_amount += point.user_point as u64 * value / total_project_points as u64;
            reward.accumulated_amount += reward.reward_amount;
        }
    }
    if let Some(reward) = &mut ctx.accounts.reward2 {
        if let Some(point) = &ctx.accounts.project_user_point2 {
            reward.reward_amount += point.user_point as u64 * value / total_project_points as u64;
            reward.accumulated_amount += reward.reward_amount;
        }
    }
    if let Some(reward) = &mut ctx.accounts.reward3 {
        if let Some(point) = &ctx.accounts.project_user_point3 {
            reward.reward_amount += point.user_point as u64 * value / total_project_points as u64;
            reward.accumulated_amount += reward.reward_amount;
        }
    }
    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.fee_and_rent_payer.key(),
        &ctx.accounts.donate_pool.key(),
        value,
    );
    invoke(
        &transfer_instruction,
        &[
            ctx.accounts.fee_and_rent_payer.to_account_info(),
            ctx.accounts.donate_pool.to_account_info(),
        ],
    )?;
    Ok(())
}
