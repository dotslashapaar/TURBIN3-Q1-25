# Marketplace Project

A decentralized NFT marketplace built on Solana using the [Anchor framework](https://www.anchor-lang.com/docs). This project enables users to initialize a marketplace, list their NFTs for sale, delist them, or purchase them—with reward tokens minted for buyers.

---

## 3D Overview

- **Decentralized:**  
  Uses Solana's high-performance blockchain with Program-Derived Addresses (PDAs) for secure, deterministic account management.

- **Dynamic:**  
  Users can list NFTs, cancel listings, and purchase NFTs. Each transaction involves transferring tokens, SOL, and minting rewards, all coordinated using PDAs and cross-program invocations (CPIs).

- **Distributed:**  
  Integrates with external programs like the Associated Token Program and Metaplex Metadata Program to handle token accounts and NFT metadata, ensuring seamless interaction with the Solana ecosystem.

---

## Project Structure & Explanation

### `lib.rs`

- **Purpose:**  
  Serves as the entry point for the Anchor program. It declares the program ID and exposes the public instructions: `initialize`, `listing`, `delist`, and `purchase`.

- **Key Functions:**  
  - **initialize:**  
    Sets up the marketplace by creating the marketplace account (PDA), treasury PDA, and rewards mint PDA.
  - **listing:**  
    Creates a listing for an NFT, transferring the NFT into a vault (an associated token account owned by a listing PDA) and storing listing details.
  - **delist:**  
    Allows the maker to cancel a listing by transferring the NFT back to their account and closing the listing PDA.
  - **purchase:**  
    Facilitates the purchase by transferring SOL (splitting payment between the maker and the treasury), transferring the NFT to the buyer, minting reward tokens, and cleaning up the listing.

---

### `marketplace.rs`

- **Struct:** `Marketplace`

  **Fields:**
  - `admin`: The marketplace administrator (creator).
  - `fee`: Transaction fee charged by the marketplace.
  - `bump`: Bump seed for the marketplace PDA.
  - `treasury_bump`: Bump seed for the treasury PDA.
  - `rewards_bump`: Bump seed for the rewards mint PDA.
  - `name`: A string identifier for the marketplace.

- **Purpose:**  
  Holds global state for the marketplace. This account is a PDA derived with the seed `[b"marketplace", name.as_str().as_bytes()]`.

- **Space Allocation:**  
  Defined by the `INIT_SPACE` constant, which accounts for the discriminator and the sizes of each field.

---

### `listing.rs`

- **Struct:** `Listing`

  **Fields:**
  - `maker`: The NFT owner who created the listing.
  - `maker_mint`: The NFT’s mint address.
  - `price`: The sale price for the NFT.
  - `bump`: Bump seed for the listing PDA.

- **Purpose:**  
  Stores details for a specific NFT listing. The account is created as a PDA using the seeds `[marketplace.key().as_ref(), maker_mint.key().as_ref()]`.

- **Space Allocation:**  
  Defined by the `INIT_SPACE` constant.

---

### `initialize.rs`

- **Context Struct:** `Initialize`

  **Accounts:**
  - `admin`: The signer who initializes the marketplace.
  - `marketplace`: The PDA for marketplace state, created using the seed `[b"marketplace", name.as_str().as_bytes()]`.
  - `treasury`: A system account PDA for fee collection, derived with the seed `[b"treasury", marketplace.key().as_ref()]`.
  - `reward_mint`: A mint PDA for reward tokens, derived with the seed `[b"rewards", marketplace.key().as_ref()]`.

- **Functionality:**  
  The `init` method sets the initial values for the marketplace state (admin, fee, name, and bump seeds). The computed bumps (accessible via `ctx.bumps`) are stored explicitly in the respective fields of the `Marketplace` struct.

---

### `list.rs`

- **Context Struct:** `List`

  **Accounts:**
  - `maker`: The signer listing the NFT.
  - `marketplace`: The marketplace state account (validated via its PDA).
  - `maker_mint`: The mint for the NFT being listed.
  - `maker_ata`: The maker’s associated token account (ATA) that holds the NFT.
  - `vault`: A new ATA initialized to act as a vault for the NFT while it is listed.
  - `listing`: The PDA that stores listing details, created using seeds `[marketplace.key().as_ref(), maker_mint.key().as_ref()]`.
  - `collection_mint`: The mint for the NFT’s collection.
  - `metadata` & `master_edition`: Accounts provided by the Metaplex metadata program to verify NFT metadata.

- **Functions:**  
  - **create_listing:**  
    Initializes the `Listing` account with details such as maker, NFT mint, price, and bump.
  - **deposit_nft:**  
    Transfers the NFT from the maker’s ATA to the vault.

---

### `delist.rs`

- **Context Struct:** `Delist`

  **Accounts:**
  - `maker`: The signer who originally listed the NFT.
  - `marketplace`: The marketplace state account.
  - `maker_mint`: The NFT's mint address.
  - `maker_ata`: The maker’s token account for receiving the NFT back.
  - `vault`: The associated token account holding the NFT during listing.
  - `listing`: The PDA for the listing, which will be closed on delisting.

- **Functions:**  
  - **delist:**  
    Transfers the NFT from the vault back to the maker’s ATA.
  - **close_mint_vault:**  
    Closes the vault account using PDA-derived signer seeds.

---

### `purchase.rs`

- **Context Struct:** `Purchase`

  **Accounts:**
  - `taker`: The buyer (signer) purchasing the NFT.
  - `maker`: The seller who originally listed the NFT.
  - `marketplace`: The marketplace state account.
  - `maker_mint`: The NFT's mint address.
  - `taker_ata`: The taker’s ATA to receive the NFT.
  - `taker_ata_reward`: The taker’s ATA to receive reward tokens.
  - `listing`: The listing PDA that will be closed after purchase.
  - `vault`: The vault holding the NFT.
  - `treasury`: The treasury PDA that receives the fee.
  - `rewards_mint`: The rewards mint PDA used for minting reward tokens.

- **Functions:**  
  - **send_sol:**  
    Transfers SOL from the taker to the maker and treasury (fee).
  - **receive_nft:**  
    Transfers the NFT from the vault to the taker’s ATA.
  - **receive_rewards:**  
    Mints reward tokens to the taker using the rewards mint PDA.
  - **close_mint_vault:**  
    Closes the vault account after NFT transfer.

---

## PDA Overview in the Project

- **Custom PDAs:**  
  - **Marketplace PDA:**  
    Derived with seed `[b"marketplace", name.as_str().as_bytes()]` and holds global state.
  - **Treasury PDA:**  
    Derived with seed `[b"treasury", marketplace.key().as_ref()]` for fee collection.
  - **Rewards Mint PDA:**  
    Derived with seed `[b"rewards", marketplace.key().as_ref()]` to mint reward tokens.
  - **Listing PDA:**  
    Derived with seed `[marketplace.key().as_ref(), maker_mint.key().as_ref()]` to store listing details.

- **Associated Token Accounts (ATAs):**  
  These are also PDAs (derived by the Associated Token Program) used for holding NFTs (vault) and tokens (maker’s, taker’s, and reward accounts).

- **External PDAs:**  
  The Metadata and Master Edition accounts are PDAs derived by the Metaplex Metadata Program.

---

## Getting Started

1. **Install Dependencies:**  
   Ensure you have the Solana CLI and Anchor installed:
   ```bash
   solana --version
   anchor --version

2. **Build the Project:**  
   ```bash
   anchor build

3. **Deploy the Program:**  
   ```bash
   anchor deploy

4. **Interact with the Marketplace:**  
Use the provided Anchor tests or your own client code to call the `initialize`, `listing`, `delist`, and `purchase` instructions.
