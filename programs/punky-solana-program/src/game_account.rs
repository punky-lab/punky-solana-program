use anchor_lang::prelude::*;

use crate::config::*;
use crate::utils::*;

#[account]
#[derive(InitSpace)]
pub struct GameAccount {
    pub initialized: bool,

    pub fitness: u16,
    pub loyalty: u16,
    pub happiness: u16,
    pub balance: u64,
}

impl GameAccount {
    pub fn initialize(&mut self) -> Result<()> {
        self.initialized = true;
        self.fitness = INITIAL_FITNESS;
        self.loyalty = INITIAL_LOYALTY;
        self.happiness = INITIAL_HAPPINESS;
        self.balance = INITIAL_BALANCE;
        Ok(())
    }

    pub fn pet_pet(&mut self) -> Result<()> {
        self.fitness = stat_clamp(self.fitness - 10);
        self.happiness = stat_clamp(self.happiness + 5);
        self.loyalty = stat_clamp(self.loyalty + 5);
        self.balance += 1;
        Ok(())
    }

    pub fn feed_pet(&mut self) -> Result<()> {
        self.fitness = stat_clamp(self.fitness + 10);
        self.happiness = stat_clamp(self.happiness + 5);
        self.loyalty = stat_clamp(self.loyalty + 5);
        self.balance -= 1;
        Ok(())
    }
}
