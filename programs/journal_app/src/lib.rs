use anchor_lang::prelude::*;
mod instructions;
mod state;
declare_id!("6iX4fpymT5iHYYaY6JVag7nhXCG91qGCAF98AGfEs2jA");

#[program]
pub mod journal_app {

    pub use super::instructions::*;
    use super::*;
    pub fn initialize(ctx: Context<IntitJournal>, name: String) -> Result<()> {
        initialize_journal(ctx, name)?;
        Ok(())
    }
    pub fn update_journal(
        ctx: Context<UpdateNote>,
        note_id: u64,
        title: String,
        contents: String,
    ) -> Result<()> {
        update_note(ctx, note_id, title, contents)?;
        Ok(())
    }

    pub fn delete_from_journal(ctx: Context<DeleteNote>, note_id: u64) -> Result<()> {
        delete_note(ctx, note_id)?;
        Ok(())
    }
}
