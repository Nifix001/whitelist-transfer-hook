use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistError {

    #[msg("This address is already whitelisted")]
    AlreadyWhitelisted,
    
    #[msg("This address is not whitelisted")]
    NotWhitelisted,

    #[msg("Invalid user address provided")]
    InvalidUserAddress,
}