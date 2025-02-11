use anchor_lang::prelude::*;

declare_id!("FEa5qGsXBBRdsp9DTnqR4ReKzjV1YLf6xqNDcQJAL7bh");
mod instructions;
use instructions::*;
mod state;
use state::*;

#[program]
pub mod escrow_sol_a {
    use super::*;

    pub fn make(ctx: Context<Make>,seed: u64, receive: u64,deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>)->Result<()>{
        ctx.accounts.send()?;
        ctx.accounts.withdraw()?;
        ctx.accounts.close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>)->Result<()>{
        ctx.accounts.refund()?;
        ctx.accounts.close_refund()?;
        Ok(())
    }
}

// Morally Correct Way xD - with impl as single fn

// #[program]
// pub mod escrow_practice {
//     use super::*;

//     pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
//         ctx.accounts.init_escrow(seed, receive, ctx.bumps)?;
//         ctx.accounts.deposit(deposit)?;
//         Ok(())
//     }
    
//     pub fn take(ctx: Context<Take>) -> Result<()> {
//         ctx.accounts.send_and_close()?;
//         Ok(())
//     }

//     pub fn refund(ctx: Context<Refund>) -> Result<()> {
//         ctx.accounts.refund_and_close()?;
//         Ok(())
//     }


// }

// 3WuQpvjpGhAUDfCWGdSSLdwGZ6qg4ZxdJB48P432FBe47gyCqfE4dnBX3ShnBaxujEFkJtMMZB6ay5xznyEnvcnp
