use anchor_lang::prelude::*;

mod config;
mod errors;
mod game_account;
mod utils;

use errors::GameError;
use game_account::GameAccount;

declare_id!("6YmNaSBGPwjxnxAFQePz7Z4R9YUMEoaCJGE2JakDrY7D");

#[program]
pub mod punky_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;

        // Check if account is already initialized
        require!(!game_account.initialized, GameError::AlreadyInitialized);

        game_account.initialize()?;

        msg!(
            "Initialized game account for {:?}",
            ctx.accounts.signer.key()
        );
        Ok(())
    }

    pub fn pet_pet(ctx: Context<UpdateGameAccount>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        game_account.pet_pet()?;
        Ok(())
    }

    pub fn feed_pet(ctx: Context<UpdateGameAccount>) -> Result<()> {
        let game_account = &mut ctx.accounts.game_account;
        game_account.feed_pet()?;
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

#[derive(Accounts)]
pub struct UpdateGameAccount<'info> {
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>,
}
