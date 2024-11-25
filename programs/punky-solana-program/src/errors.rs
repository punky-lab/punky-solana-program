use anchor_lang::prelude::*;

#[error_code]
pub enum GameError {
    #[msg("This account has already been initialized")]
    AlreadyInitialized,
}
