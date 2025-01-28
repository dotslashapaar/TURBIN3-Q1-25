use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, TransferChecked},token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Escrow;


#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", escrow.maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        constraint = (maker.key() == escrow.maker.key()),
    )]
    pub escrow: Account<'info, Escrow>,
    
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info>Refund<'info> {
    pub fn refund(&mut self)-> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];


        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        let amount = self.vault.amount;

        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;

        Ok(())
    }

    pub fn close_refund(&mut self)-> Result<()>{
        let cpi_program =self.token_program.to_account_info();

        let cpi_accounts = CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];


        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}

// Morally Correct Way xD - with impl as single fn

// #[derive(Accounts)]
// pub struct Refund<'info>{
//     #[account(mut)]
//     pub maker: Signer<'info>,
//     pub mint_a: InterfaceAccount<'info,Mint>,

//     #[account(
//         mut,
//         close = maker,
//         seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
//         bump = escrow.bump,
//         constraint = (maker.key()== escrow.maker.key()),
//     )]
//     pub escrow: Account<'info, Escrow>,

//     #[account(
//         mut,
//         associated_token::mint = mint_a,
//         associated_token:: authority = maker,
//     )]
//     pub maker_a_ata: InterfaceAccount<'info, TokenAccount>,

//     #[account(
//         mut,
//         associated_token::mint = mint_a,
//         associated_token::authority = escrow,
//     )]
//     pub vault: InterfaceAccount<'info, TokenAccount>,

//     pub associated_token_program: Program<'info, AssociatedToken>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub system_program: Program<'info, System>,
// }

// impl <'info>Refund<'info> {
//     pub fn refund_and_close(&mut self) -> Result<()>{
//         let cpi_program = self.token_program.to_account_info();

//         // Refund Token A from vault to maker_a_ata
//         let cpi_accounts = TransferChecked{
//             from: self.vault.to_account_info(),
//             to: self.maker_a_ata.to_account_info(),
//             mint: self.mint_a.to_account_info(),
//             authority: self.escrow.to_account_info(),
//         };

//         let signer_seeds: [&[&[u8]]; 1] = [&[
//             b"escrow",
//             self.maker.to_account_info().key.as_ref(),
//             &self.escrow.seed.to_be_bytes()[..],
//             &[self.escrow.bump]
//         ]];

//         let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, &signer_seeds);

//         transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

//         // Closeing vault

//         let cpi_accounts = CloseAccount{
//             account: self.vault.to_account_info(),
//             destination: self.maker.to_account_info(),
//             authority: self.escrow.to_account_info(),
//         };

//         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

//         close_account(cpi_ctx)?;

//         Ok(())
//     }
// }


