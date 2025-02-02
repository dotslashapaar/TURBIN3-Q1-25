use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer}};
use constant_product_curve::ConstantProduct;

use crate::Config;

use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Withdraw<'info>{
    pub lp_provider: Signer<'info>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    //mints
    pub mint_x: Box<Account<'info, Mint>>,
    pub mint_y: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp: Box<Account<'info, Mint>>,

    // vault's
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,

    // ata's
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = lp_provider,
    )]
    pub lp_provider_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = lp_provider,
    )]
    pub lp_provider_y: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = lp_provider,
    )]
    pub lp_provider_mint_lp: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl <'info> Withdraw<'info> {

    pub fn withdraw(&mut self,amount: u64, min_x: u64, min_y: u64)->Result<()>{
        
        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount, 
            self.vault_y.amount, 
            self.mint_lp.supply, 
            amount, 
            6
        ).unwrap();
        
        require!(min_x <= amounts.x && min_y <= amounts.y, AmmError::SlippageExceeded);

        self.withdraw_token(true, amounts.x)?;
        self.withdraw_token(false, amounts.y)?;
        self.burn_lp_tokens(amount)?;

        Ok(())
    }

    fn withdraw_token(&mut self, is_x: bool, amount: u64)->Result<()>{
        
        let (from, to) = match is_x {
            true => (self.vault_x.to_account_info(), self.lp_provider_x.to_account_info()),
            false => (self.vault_y.to_account_info(), self.lp_provider_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from,
            to,
            authority: self.lp_provider.to_account_info(),
        };

        let seeds = &[
            b"config", 
            &self.config.seed.to_le_bytes()[..],
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    fn burn_lp_tokens(&mut self, amount: u64)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn{
            mint: self.mint_lp.to_account_info(),
            from: self.lp_provider_mint_lp.to_account_info(),
            authority: self.lp_provider.to_account_info(),
        };


        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)?;

        Ok(())
    }
}