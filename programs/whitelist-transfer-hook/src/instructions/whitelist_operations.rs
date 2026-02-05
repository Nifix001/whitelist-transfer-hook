use anchor_lang::{
    prelude::*, 
};

use crate::{errors::WhitelistError, state::whitelist::Whitelist};

#[derive(Accounts)]
pub struct WhitelistOperations<'info> {
    #[account(
        mut,
        //address = 
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"whitelist", whitelist.user.key().as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
    )]
    pub user: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistOperations<'info> {
    pub fn add_to_whitelist(&mut self, user: Pubkey) -> Result<()> {
        require!(self.whitelist.user == user, WhitelistError::InvalidUserAddress);

        if self.whitelist.whilisted {
             WhitelistError::AlreadyWhitelisted;
        }
        let _ = self.whitelist.whilisted == true;
        Ok(())

    }

    pub fn remove_from_whitelist(&mut self, user: Pubkey) -> Result<()> {
        require!(self.whitelist.user == user, WhitelistError::InvalidUserAddress);

        if !self.whitelist.whilisted {
             WhitelistError::NotWhitelisted;
        }
        let _ = self.whitelist.whilisted == false;

        Ok(())
    }

    pub fn close_whitelist(&mut self) -> Result<()> {
        self.whitelist.close(self.admin.to_account_info())?;
        Ok(())
    }
}