use anchor_lang::prelude::*;

use instructions::*;

mod error;
mod events;
mod instructions;
mod states;

#[cfg(feature = "devnet")]
declare_id!("CuuiWq1ATi8XUgnv8tdiYNQKccs3iNw2uz9CDAZeuc15");

// #[cfg(feature = "localnet")]
// declare_id!("CuuiWq1ATi8XUgnv8tdiYNQKccs3iNw2uz9CDAZeuc15");

#[program]
pub mod youbet_solana_program {
    use instructions::create_project::handle_create_project;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }
    pub fn change_admin_config(ctx: Context<ChangeAdminConfig>, new_admin: Pubkey) -> Result<()> {
        handle_change_admin_config(ctx, new_admin)
    }
    pub fn create_project(
        ctx: Context<CreateProjectAccounts>,
        project_id: String,
        name: String,
    ) -> Result<()> {
        handle_create_project(ctx, project_id, name)
    }
    pub fn create_task(
        ctx: Context<CreateTaskAccounts>,
        task_id: String,
        name: String,
        project_id: String,
        _project_bump: u8,
    ) -> Result<()> {
        handle_create_task(ctx, task_id, name, project_id, _project_bump)
    }
    pub fn link_wallet(
        ctx: Context<LinkWalletAccounts>,
        wallet: Pubkey,
        github: String,
        _admin_config_bump: u8,
    ) -> Result<()> {
        handle_link_wallet(ctx, wallet, github, _admin_config_bump)
    }
    pub fn confirm_task(
        ctx: Context<ConfirmTaskAccounts>,
        task_id: String,
        github: String,
        points: u32,
        _task_bump: u8,
        _github_bump: u8,
        _wallet_bump: u8,
    ) -> Result<()> {
        handle_confirm_task(
            ctx,
            task_id,
            github,
            points,
            _task_bump,
            _github_bump,
            _wallet_bump,
        )
    }
    pub fn donate_to_project(
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
        handle_donate_to_project(
            ctx,
            value,
            project_id,
            _project_bump,
            _donate_pool_bump,
            account1,
            _reward_bump1,
            _project_user_point_bump1,
            account2,
            _reward_bump2,
            _project_user_point_bump2,
            account3,
            _reward_bump3,
            _project_user_point_bump3,
        )
    }

    pub fn claim_reward(
        ctx: Context<ClaimRewardAccounts>,
        _donate_pool_bump: u8,
        _reward_bump: u8,
    ) -> Result<()> {
        handle_claim_reward(ctx, _donate_pool_bump, _reward_bump)
    }
}
