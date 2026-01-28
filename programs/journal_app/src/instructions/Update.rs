use anchor_lang::prelude::*;

use crate::state::Note;

#[derive(Accounts)]
#[instruction(note_id:u64)]
pub struct UpdateNote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut,constraint=signer.key()==note_account.owner,seeds=[b"note",signer.key().as_ref(),note_id.to_le_bytes().as_ref()],bump)]
    pub note_account: Account<'info, Note>,
    pub system_program: Program<'info, System>,
}

pub fn update_note(
    ctx: Context<UpdateNote>,
    note_id: u64,
    title: String,
    contents: String,
) -> Result<()> {
    let note_account = &mut ctx.accounts.note_account;
    note_account.contents = contents;
    note_account.title = title;
    Ok(())
}
