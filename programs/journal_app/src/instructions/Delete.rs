use anchor_lang::prelude::*;

use crate::state::Note;

#[derive(Accounts)]
#[instruction(note_id:u8)]
pub struct DeleteNote<'info> {
    pub owner: Signer<'info>,

    #[account(mut,close=owner,seeds=[b"note",owner.key().as_ref(),note_id.to_le_bytes().as_ref()],bump)]
    pub note_account: Account<'info, Note>,
    pub system_program: Program<'info, System>,
}

pub fn delete_note(ctx: Context<DeleteNote>, note_id: u8) -> Result<()> {
    Ok(())
}
