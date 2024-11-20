use anchor_lang::prelude::*;

#[error_code]
pub enum PunkyError {
    #[msg("This account has already been initialized")]
    AlreadyInitialized,
}
