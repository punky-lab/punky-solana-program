use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct GameAccount {
    pub initialized: bool,

    pub health: u16,
    pub fitness: u16,
    pub loyalty: u16,
    pub balance: u64,
}