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

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)?;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, amount: u64, roll: u8) -> Result<()> {
        ctx.accounts.create_bet(seed, &ctx.bumps, amount, roll)?;
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)?;
        Ok(())
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&ctx.bumps, &sig)?;
        Ok(())
    }

}

// Devnet Signature: 27ycvfXbVinKLx6R3DRUHKYoZ3XYR3wVG3MCQzB89AwgHUF6qHUN9jxRDgSm1NxpEnzZ3w6a4Dbi7cngyivkBpu9
