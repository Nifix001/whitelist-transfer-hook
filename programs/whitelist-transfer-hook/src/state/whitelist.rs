use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct Whitelist {
    pub address: Pubkey,
    pub user : Pubkey,
    pub whilisted: bool,
    pub bump: u8,
}