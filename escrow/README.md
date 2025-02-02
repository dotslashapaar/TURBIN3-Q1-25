# Escrow Solana Program

A Solana escrow program built using the [Anchor framework](https://www.anchor-lang.com/docs). This program allows a maker to create an escrow, deposit tokens into a vault, and later have a taker complete the exchange by sending tokens to the maker. Alternatively, the maker can refund the escrow if the exchange is not completed.

---

## 3D Overview

- **Decentralized:**  
  Utilizes Solana’s high-performance blockchain and Program Derived Addresses (PDAs) to securely manage escrow accounts and associated vaults without a central authority.

- **Dynamic:**  
  Provides three primary instructions:
  - `Make:` The maker creates an escrow account and deposits tokens.
  - `Take:` The taker fulfills the escrow by transferring tokens to the maker and withdrawing the escrowed tokens.
  - `Refund:` The maker reclaims tokens from the escrow if the exchange is not fulfilled.

- **Distributed:**  
  Integrates with the Associated Token Program and the SPL Token program via Anchor’s cross-program invocation (CPI) mechanism to manage token transfers and account closures in a trustless manner.

---

## Project Structure & Explanation

### `lib.rs`

**Purpose:**  
This is the entry point of the program. It declares the program ID and exposes the public instructions:  
- `make:` Initializes an escrow and deposits tokens into a vault.  
- `take:` Allows a taker to complete the escrow by sending tokens to the maker and withdrawing the escrowed tokens.  
- `refund:` Allows the maker to reclaim tokens if the escrow is not fulfilled.

---

### `escrow.rs`

**Struct: Escrow**  
**Fields:**  
- `seed:` A unique number to differentiate escrows.  
- `maker:` The public key of the escrow maker.  
- `mint_a:` The mint for the token being deposited.  
- `mint_b:` The mint for the token expected to be received.  
- `receive:` The amount the maker expects to receive.  
- `bump:` The PDA bump used for escrow account derivation.

**Purpose:**  
This account holds the state of an escrow. It stores all key details needed to verify and execute token transfers.

---

### `make.rs`

**Context: Make**  
**Accounts:**  
- `maker:` The signer who creates the escrow.  
- `mint_a & mint_b:` The token mints involved in the exchange.  
- `maker_ata_a:` The maker’s associated token account (ATA) for the token being deposited.  
- `escrow:` The PDA escrow account, derived using seeds that include a constant string, the maker’s key, and the provided seed value.  
- `vault:` The associated token account (ATA) that holds the deposited tokens.  
- `Programs:` Includes the Associated Token Program, Token Program, and System Program.

**Functions:**  
- **init_escrow:** Initializes the escrow state with parameters such as the seed, expected receive amount, and bump.  
- **deposit:** Transfers tokens from the maker’s account into the vault.

---

### `take.rs`

**Context: Take**  
**Accounts:**  
- `taker:` The signer fulfilling the escrow.  
- `maker:` The original escrow maker (as a system account to receive tokens).  
- `mint_a & mint_b:` The token mints involved in the exchange.  
- `taker_ata_a:` The taker’s token account to receive the escrowed tokens.  
- `taker_ata_b & maker_ata_b:` Token accounts for transferring the expected tokens.  
- `escrow:` The escrow account (a PDA) which is validated and later closed after a successful exchange.  
- `vault:` The vault holding the escrowed tokens.  
- `Programs:` Includes the Associated Token Program, Token Program, and System Program.

**Functions:**  
- **send:** Transfers the required tokens from the taker to the maker.  
- **withdraw:** Moves the escrowed tokens from the vault to the taker’s account.  
- **close:** Closes the escrow account and vault after the exchange is complete.

---

### `refund.rs`

**Context: Refund**  
**Accounts:**  
- `maker:` The signer who originally created the escrow.  
- `mint_a:` The token mint for the token being deposited.  
- `maker_ata_a:` The maker’s associated token account for the token being deposited.  
- `escrow:` The escrow account (a PDA) which is validated and later closed during the refund process.  
- `vault:` The vault holding the escrowed tokens.  
- `Programs:` Includes the Associated Token Program, Token Program, and System Program.

**Functions:**  
- **refund:** Transfers tokens from the vault back to the maker’s token account.  
- **close_refund:** Closes the escrow account and the vault after the refund is processed.

---

## Getting Started

1. **Install Dependencies:**  
   Ensure you have the Solana CLI and Anchor installed on your system.
   
2. **Build the Project:**  
   Use Anchor’s build command to compile the program:

3. **Deploy the Program:**  
Deploy the program to the Solana cluster using:

4. **Interact with the Program:**  
Use the provided tests or your own client code to call the `make`, `take`, and `refund` instructions.

---
