use anchor_lang::prelude::*;

use crate::states::{
    admin_config::{AdminConfigAccount, ADMIN_CONFIG_PREFIX},
    github::{GithubAccount, GITHUB_PREFIX},
    wallet::{WalletAccount, WALLET_PREFIX},
};

#[derive(Accounts)]
#[instruction(wallet: Pubkey, github: String, _admin_config_bump: u8)]
pub struct LinkWalletAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,
    #[account(
        seeds = [
            ADMIN_CONFIG_PREFIX.as_bytes()
        ],
        bump = _admin_config_bump,
        constraint = fee_and_rent_payer.key() == admin_config.authority,
    )]
    pub admin_config: Box<Account<'info, AdminConfigAccount>>,
    #[account(
        init,
        payer = fee_and_rent_payer,
        space = WalletAccount::space(),
        seeds = [
            WALLET_PREFIX.as_bytes(),
            wallet.as_ref(),
        ],
        bump
    )]
    pub wallet_account: Box<Account<'info, WalletAccount>>,
    #[account(
        init,
        payer = fee_and_rent_payer,
        space = GithubAccount::space(),
        seeds = [
            GITHUB_PREFIX.as_bytes(),
            github.as_ref(),
        ],
        bump
    )]
    pub github_account: Box<Account<'info, GithubAccount>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_link_wallet(
    ctx: Context<LinkWalletAccounts>,
    wallet: Pubkey,
    github: String,
    _admin_config_bump: u8,
) -> Result<()> {
    // check ?? authority ?? redo ??
    let wallet_account: &mut Box<Account<WalletAccount>> = &mut ctx.accounts.wallet_account;
    wallet_account.github = github;
    let github_account: &mut Box<Account<GithubAccount>> = &mut ctx.accounts.github_account;
    github_account.wallet = wallet;
    Ok(())
}
