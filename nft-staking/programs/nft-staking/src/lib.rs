use anchor_lang::prelude::*;

mod errors;
mod state;
use state::*;
mod instructions;
use instructions::*;

declare_id!("6fc11zfjvqNDoMK9eWTCQGf28S8qrK1xW6M8HakJGQM7");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
