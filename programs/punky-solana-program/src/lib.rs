use anchor_lang::prelude::*;

declare_id!("6YmNaSBGPwjxnxAFQePz7Z4R9YUMEoaCJGE2JakDrY7D");

#[program]
pub mod punky_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
