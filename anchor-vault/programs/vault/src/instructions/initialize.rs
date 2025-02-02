use anchor_lang::prelude::*;

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = VaultState::INIT_SPACE + 8,
        seeds = [b"state", signer.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds=[vault_state.key().as_ref()],bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self,bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        
        Ok(())
    }
}