use anchor_lang::prelude::*;

mod config;
mod errors;
mod game_account;

use errors::PunkyError;
use game_account::GameAccount;

declare_id!("6YmNaSBGPwjxnxAFQePz7Z4R9YUMEoaCJGE2JakDrY7D");

#[program]
pub mod punky_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account_data = &mut ctx.accounts.game_account;

        // Check if account is already initialized
        require!(!account_data.initialized, PunkyError::AlreadyInitialized);

        account_data.initialized = true;
        account_data.health = config::INITIAL_HEALTH;
        account_data.fitness = config::INITIAL_FITNESS;
        account_data.loyalty = config::INITIAL_LOYALTY;
        account_data.balance = config::INITIAL_BALANCE;

        msg!(
            "Initialized game account for {:?}",
            ctx.accounts.signer.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + GameAccount::INIT_SPACE,
        seeds = [config::GAME_ACCOUNT_SEED, signer.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,

    pub system_program: Program<'info, System>,
}
