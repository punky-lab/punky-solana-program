use anchor_lang::prelude::*;

declare_id!("6YmNaSBGPwjxnxAFQePz7Z4R9YUMEoaCJGE2JakDrY7D");

#[program]
pub mod punky_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account_data = &mut ctx.accounts.game_account;
        account_data.health = 500;
        account_data.fitness = 500;
        account_data.loyalty = 500;
        account_data.balance = 1000;
        msg!("Initialized game account for {:?}", ctx.accounts.signer.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        // Program derived account
        init,
        payer = signer,
        space = 8 + GameAccount::INIT_SPACE,
        seeds = [b"game_account", signer.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct GameAccount {
    pub health: u16,
    pub fitness: u16,
    pub loyalty: u16,
    pub balance: u64,
}
