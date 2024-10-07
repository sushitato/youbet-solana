use anchor_lang::prelude::*;

use crate::states::project::{ProjectAccount, PROJECT_PREFIX};

#[derive(Accounts)]
#[instruction(project_id: String, name: String)]

pub struct CreateProjectAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,
    #[account(
        init,
        payer = fee_and_rent_payer,
        space = ProjectAccount::space(),
        seeds = [
            PROJECT_PREFIX.as_bytes(),
            project_id.as_ref(),
        ],
        bump
    )]
    pub project: Box<Account<'info, ProjectAccount>>,
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_project(
    ctx: Context<CreateProjectAccounts>,
    project_id: String,
    name: String,
) -> Result<()> {
    // check ?? authority ?? redo ??
    let project: &mut Box<Account<ProjectAccount>> = &mut ctx.accounts.project;
    project.project_id = project_id;
    project.name = name;
    Ok(())
}
