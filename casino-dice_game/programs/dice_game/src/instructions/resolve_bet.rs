use anchor_lang::{ prelude::*, solana_program, system_program::{ transfer, Transfer } };
use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use solana_program::{
    sysvar::instructions::load_instruction_at_checked,
    ed25519_program,
    hash::hash,
};

use crate::{ states::Bet, errors::DiceError };

pub const HOUSE_EDGE: u16 = 150; // 1.5% House edge

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(
        mut
    )]
    ///CHECK: This is safe
    pub player: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(address = solana_program::sysvar::instructions::ID)]
    /// CHECK: This is safe
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // Get the Ed25519 signature instruction
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?; // Retrieves the first instruction (0 index) from the transaction.( the Ed25519 signature verification instruction)

        //ensures that the instruction was intended for the correct program. (Ed25519 program is responsible for verifying signatures)
        require_keys_eq!(ix.program_id, ed25519_program::ID, DiceError::Ed25519Program);

        // Make sure there are no accounts present if there are instruction might have been tampered with
        //Ed25519 program does not require any accounts to be listed in its instruction
        require_eq!(ix.accounts.len(), 0, DiceError::Ed25519Accounts);

        //Extract the Signature Data
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        //There should be only one signature in the instruction.
        require_eq!(signatures.len(), 1, DiceError::Ed25519DataLength);
        let signature = &signatures[0];

        // The is_verifiable flag ensures that the signature was properly structured.
        require!(signature.is_verifiable, DiceError::Ed25519Header);

        // Check That the Public Key Matches the House Account. The signature must have been created by the house account.
        require_keys_eq!(
            signature.public_key.ok_or(DiceError::Ed25519Pubkey)?,
            self.house.key(),
            DiceError::Ed25519Pubkey
        );

        //This confirms that the provided signature (sig) matches what was stored in the transaction.
        require!(
            &signature.signature.ok_or(DiceError::Ed25519Signature)?.eq(sig),
            DiceError::Ed25519Signature
        );

        // Ensure the Message Matches the Bet Data stored in the program.
        require!(
            &signature.message
                .as_ref()
                .ok_or(DiceError::Ed25519Signature)?
                .eq(&self.bet.to_slice()),
            DiceError::Ed25519Signature
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, bumps: &ResolveBetBumps, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes(); //Hash the signiture in to bytes. Output: 32-byte hash ([u8; 32])
        let mut hash_16: [u8; 16] = [0; 16]; //create an empty array with the half the signature hash in to bytes
        hash_16.copy_from_slice(&hash[0..16]); //Takes the first 16 bytes ([0..16]) of the hash.
        let lower = u128::from_le_bytes(hash_16); //Converts them into a 128-bit unsigned integer
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        //Adds the two 128-bit numbers together (wrap around if there's an overflow instead of crashing)
        // Computes the modulo 100 of the result. Ensures the final value is between 0-99.
        //Converts the result to an u8 (ensuring it's small enough). Adds 1 to shift the range from 0-99 to 1-100, ensuring a dice-like roll
        let roll = (lower.wrapping_add(upper).wrapping_rem(100) as u8) + 1;

        if self.bet.roll > roll {
            // Payout minus house edge
            let payout = (
                self.bet.amount as u128 //converted to u128 to prevent overflows
            )
                .checked_mul(10000 - (HOUSE_EDGE as u128)) // 100% (base pointes) - 1.5% (house hedge) - applying house cut
                .ok_or(DiceError::Overflow)?
                .checked_div((self.bet.roll as u128) - 1) // payout multiplier. We divide by roll - 1 instead of roll because:The probability of rolling less than 50 is 49/100.
                .ok_or(DiceError::Overflow)?
                .checked_div(100) //divide by 100 to get the result in basis points
                .ok_or(DiceError::Overflow)? as u64;

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];
            let signer_seeds = &[&seeds[..]][..];

            let ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds
            );
            transfer(ctx, payout)?;
        }
        Ok(())
    }
}