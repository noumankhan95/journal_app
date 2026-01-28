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
}
