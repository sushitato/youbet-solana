use anchor_lang::prelude::*;

use crate::states::{
    project::{ProjectAccount, PROJECT_PREFIX},
    task::{TaskAccount, TASK_PREFIX},
};

#[derive(Accounts)]
#[instruction(task_id: String, name: String, project_id: String, _project_bump: u8)]

pub struct CreateTaskAccounts<'info> {
    #[account(mut)]
    pub fee_and_rent_payer: Signer<'info>,
    #[account(
        init,
        payer = fee_and_rent_payer,
        space = TaskAccount::space(),
        seeds = [
            TASK_PREFIX.as_bytes(),
            task_id.as_ref(),
        ],
        bump
    )]
    pub task: Box<Account<'info, TaskAccount>>,
    #[account(
        seeds = [
            PROJECT_PREFIX.as_bytes(),
            project_id.as_ref(),
        ],
        bump = _project_bump,
    )]
    pub project: Box<Account<'info, ProjectAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_task(
    ctx: Context<CreateTaskAccounts>,
    task_id: String,
    name: String,
    project_id: String,
    _project_bump: u8,
) -> Result<()> {
    // check ?? authority ?? redo ??
    let task: &mut Box<Account<TaskAccount>> = &mut ctx.accounts.task;
    task.task_id = task_id;
    task.name = name;
    task.completed = false;
    task.project_id = project_id;
    task.project_bump = _project_bump;
    Ok(())
}
