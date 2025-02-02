use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>,
}

impl <'info> Close <'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let transfer_accounts = Transfer{
            from: self.vault_state.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"state",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds);

        let amount = self.vault.lamports();

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}