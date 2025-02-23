use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;
pub mod errors;

pub use states::*;
pub use instructions::*;
pub use errors::*;

declare_id!("Bj9k7wcTXaB2tkLnxa1WaDuuvHz9KzDZuwY296xpJQR2");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

