use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut, 
        seeds=[vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self,amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_cpx = CpiContext::new(system_program, accounts);

        transfer(cpi_cpx, amount)?;
        
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.vault_state.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = &[
            b"state",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}