use anchor_lang::prelude::*;
mod state;

mod instructions;
use instructions::*;

declare_id!("6MV7ZqomXJ2gYCJVcPEG5BaR2H7ad9Mb7jhkWRkYwxQ9");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()>{
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()>{
        ctx.accounts.close()?;
        Ok(())
    }
}

// 2zfa9kjrAqn3qN85tNM1EkvzSB2qcDC8Xwr9Ros9GKRXQmBuwjtvaA7bWeqHKVvQZfRbt3v6FgZQwTd8baFjLxCt
