use anchor_lang::prelude::*;

use crate::state::Note;

#[derive(Accounts)]
#[instruction(note_id:u64)]
pub struct ReadNote<'info> {
    pub owner: Signer<'info>,
    #[account(has_one=owner,seeds=[b"note",owner.key().as_ref(),note_id.to_le_bytes().as_ref()],bump)]
    pub note_account: Account<'info, Note>,
    pub system_program: Program<'info, System>,
}

pub fn read_note(ctx: Context<ReadNote>, note_id: u64) -> Result<()> {
    let note = &ctx.accounts.note_account;
    msg!("{}", note.contents);
    Ok(())
}
