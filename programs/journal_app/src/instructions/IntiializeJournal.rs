use anchor_lang::prelude::*;

use crate::state::{Counter, Journal};

#[derive(Accounts)]
pub struct IntitJournal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,payer=signer,space=8+Journal::INIT_SPACE,seeds=[b"journal",signer.key().as_ref()],bump)]
    pub journal: Account<'info, Journal>,
    #[account(init_if_needed,payer=signer,space=8+Counter::INIT_SPACE,seeds=[b"counter",signer.key().as_ref()],bump)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_journal(ctx: Context<IntitJournal>, name: String) -> Result<()> {
    let journal_account = &mut ctx.accounts.journal;
    let counter_val = ctx.accounts.counter.val;
    ctx.accounts.counter.val = counter_val + 1;
    ctx.accounts.counter.owner = ctx.accounts.signer.key();
    journal_account.name = name;
    journal_account.owner = ctx.accounts.signer.key();
    Ok(())
}
