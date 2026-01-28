use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub owner: Pubkey,
    #[max_len(100)]
    pub name: String,
    pub id: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Note {
    pub journal_key: Pubkey,
    pub owner: Pubkey,
    pub note_id: u64,
    #[max_len(100)]
    pub title: String,
    #[max_len(300)]
    pub contents: String,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub owner: Pubkey,
    pub val: u64,
}
