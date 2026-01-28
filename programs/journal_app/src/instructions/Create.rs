use anchor_lang::prelude::*;

use crate::state::{Counter, Journal, Note};

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,constraint=journal.owner==signer.key())]
    pub journal: Account<'info, Journal>,
    #[account(init,payer=signer,space=8+Note::INIT_SPACE,seeds=[b"note",signer.key().as_ref(),&[counter.val.try_into().unwrap()]],bump)]
    pub note: Account<'info, Note>,
    #[account(mut,constraint=signer.key()==counter.owner)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

pub fn create_note(ctx: Context<CreateNote>, contents: String, title: String) -> Result<()> {
    let note = &mut ctx.accounts.note;
    note.contents = contents;
    note.journal_key = ctx.accounts.journal.key();
    let counter = &mut ctx.accounts.counter;
    let counter_val = counter.val;
    note.note_id = counter_val;
    note.owner = ctx.accounts.signer.key();
    note.title = title;
    Ok(())
}
