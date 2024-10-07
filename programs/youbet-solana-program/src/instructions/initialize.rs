use anchor_lang::prelude::*;

use crate::states::{
    AdminConfigAccount, DonatePoolAccount, ADMIN_CONFIG_PREFIX, DONATE_POOL_PREFIX,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = fee_and_rent_payer,
        space = AdminConfigAccount::space(),
        seeds = [
            ADMIN_CONFIG_PREFIX.as_bytes()
        ],
        bump,
    )]
    pub admin_config: Box<Account<'info, AdminConfigAccount>>,

    #[account(
        init,
        payer = fee_and_rent_payer,
        space = DonatePoolAccount::space(),
        seeds = [
            DONATE_POOL_PREFIX.as_bytes()
        ],
        bump,
    )]
    pub donate_pool: Box<Account<'info, DonatePoolAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;

    let config: &mut Box<Account<AdminConfigAccount>> = &mut ctx.accounts.admin_config;
    config.authority = ctx.accounts.authority.key();
    config.last_block_timestamp = timestamp;
    Ok(())
}

#[derive(Accounts)]
pub struct ChangeAdminConfig<'info> {
    #[account(mut,constraint = admin.key() == admin_config.authority)]
    pub admin: Signer<'info>,
    #[account(mut,seeds = [ADMIN_CONFIG_PREFIX.as_bytes()], bump,)]
    pub admin_config: Box<Account<'info, AdminConfigAccount>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_change_admin_config(
    ctx: Context<ChangeAdminConfig>,
    new_admin: Pubkey,
) -> Result<()> {
    let timestamp = Clock::get().unwrap().unix_timestamp;
    let config: &mut Box<Account<AdminConfigAccount>> = &mut ctx.accounts.admin_config;
    config.authority = new_admin;
    config.last_block_timestamp = timestamp;
    Ok(())
}
