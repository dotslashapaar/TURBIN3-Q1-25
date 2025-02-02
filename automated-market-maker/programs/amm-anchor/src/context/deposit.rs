use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer}};
use constant_product_curve::ConstantProduct;

use crate::Config;

#[derive(Accounts)]
pub struct Deposit<'info>{
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
        mint::decimals = 6,
        mint::authority = config,
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

impl <'info> Deposit<'info> {

    pub fn deposit(&mut self,amount: u64, max_x: u64, max_y: u64)->Result<()>{
        
        let (x,y) = match self.vault_x.amount == 0 && self.vault_y.amount == 0 && self.mint_lp.supply == 0 {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount, 
                    self.vault_y.amount, 
                    self.mint_lp.supply, 
                    amount, 
                    6
                ).unwrap();
                (amounts.x, amounts.y)
            }
        };

        self.deposit_token(true, x)?;
        self.deposit_token(false, y)?;
        self.mint_lp_tokens(amount)?;

        Ok(())
    }

    fn deposit_token(&mut self, is_x: bool, amount: u64)->Result<()>{
        
        let (from, to) = match is_x {
            true => (self.lp_provider_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.lp_provider_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from,
            to,
            authority: self.lp_provider.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    fn mint_lp_tokens(&mut self, amount: u64)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo{
            mint: self.mint_lp.to_account_info(),
            to: self.lp_provider_mint_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[
            b"config", 
            &self.config.seed.to_le_bytes()[..],
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}
