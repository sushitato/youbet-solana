use anchor_lang::prelude::*;

use crate::states::{
    github::{GithubAccount, GITHUB_PREFIX},
    project::{ProjectAccount, ProjectUserPointAccount, PROJECT_PREFIX, PROJECT_USER_POINT_PREFIX},
    task::{TaskAccount, TASK_PREFIX},
    wallet::{WalletAccount, WALLET_PREFIX},
    RewardAccount, REWARD_PREFIX,
};

#[derive(Accounts)]
#[instruction(task_id: String, github: String, points: u32, _task_bump:u8, _github_bump:u8, _wallet_bump:u8)]
pub struct ConfirmTaskAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            TASK_PREFIX.as_bytes(),
            task_id.as_ref(),
        ],
        bump = _task_bump,
    )]
    pub task: Box<Account<'info, TaskAccount>>,
    #[account(
        mut,
        seeds = [
            PROJECT_PREFIX.as_bytes(),
            task.project_id.as_ref(),
        ],
        bump = task.project_bump,
    )]
    pub project: Box<Account<'info, ProjectAccount>>,
    #[account(
        seeds = [
            GITHUB_PREFIX.as_bytes(),
            github.as_ref(),
        ],
        bump = _github_bump,
    )]
    pub github_account: Box<Account<'info, GithubAccount>>,
    #[account(
        mut,
        seeds = [
            WALLET_PREFIX.as_bytes(),
            github_account.wallet.as_ref(),
        ],
        bump = _wallet_bump,
    )]
    pub wallet_account: Box<Account<'info, WalletAccount>>,
    #[account(
        init_if_needed,
        payer = fee_and_rent_payer,
        space = ProjectUserPointAccount::space(),
        seeds = [
            PROJECT_USER_POINT_PREFIX.as_bytes(),
            task.project_id.as_ref(),
            github_account.wallet.as_ref(),
        ],
        bump
    )]
    pub project_user_point: Box<Account<'info, ProjectUserPointAccount>>,

    #[account(
        init_if_needed,
        payer = fee_and_rent_payer,
        space = RewardAccount::space(),
        seeds = [
            REWARD_PREFIX.as_bytes(),
            github_account.wallet.as_ref(),
        ],
        bump
    )]
    pub reward: Box<Account<'info, RewardAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_confirm_task(
    ctx: Context<ConfirmTaskAccounts>,
    task_id: String,
    github: String,
    points: u32,
    _task_bump: u8,
    _github_bump: u8,
    _wallet_bump: u8,
) -> Result<()> {
    // check ?? authority ?? redo ??
    let task: &mut Box<Account<TaskAccount>> = &mut ctx.accounts.task;
    let github_account: &mut Box<Account<GithubAccount>> = &mut ctx.accounts.github_account;
    let project: &mut Box<Account<ProjectAccount>> = &mut ctx.accounts.project;
    let wallet_account: &mut Box<Account<WalletAccount>> = &mut ctx.accounts.wallet_account;
    let project_user_point: &mut Box<Account<ProjectUserPointAccount>> =
        &mut ctx.accounts.project_user_point;

    project.total_project_points += points;
    project.participaints.push(github_account.wallet);

    task.completed = true;
    task.task_completer = github_account.wallet;

    project_user_point.user_point += points;

    wallet_account.user_point += points;
    wallet_account.completed_tasks.push(task_id);
    Ok(())
}
