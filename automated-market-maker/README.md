# Automated Market Maker (AMM) Anchor Project

A decentralized automated market maker (AMM) built on Solana using the [Anchor framework](https://project-serum.github.io/anchor/). This project implements an AMM based on a constant product curve, allowing users to initialize a liquidity pool, deposit tokens to provide liquidity, swap tokens, and withdraw liquidity.

---

## 3D Overview

- **Decentralized:**  
  Utilizes Solana’s high-performance blockchain and Program Derived Addresses (PDAs) to manage state and token vaults without any central authority.

- **Dynamic:**  
  Supports multiple key actions:
  - **Initialize:** Set up the AMM pool with configuration parameters including a fee structure and optional authority.
  - **Deposit:** Allow liquidity providers to deposit tokens into the pool while receiving LP (liquidity provider) tokens.
  - **Swap:** Enable users to swap between two tokens based on a constant product formula.
  - **Withdraw:** Permit liquidity providers to withdraw their liquidity, receiving tokens proportional to their share of the pool.

- **Distributed:**  
  Integrates with the SPL Token program and the Associated Token Program via Anchor’s cross-program invocations (CPIs) to securely handle token transfers and account management in a trustless manner.

---

## Project Structure & Explanation

### lib.rs

**Purpose:**  
This file serves as the entry point for the AMM program. It declares the program ID and exposes the following public instructions:
- **initialize:** Initializes the AMM pool by setting up configuration, LP token mint, and vault accounts for the two tokens.
- **deposit:** Handles liquidity deposits into the pool and mints LP tokens accordingly.
- **swap:** Facilitates token swaps using a constant product curve model.

---

### initialize.rs

**Purpose:**  
Sets up the AMM configuration and associated token accounts. Key tasks include:
- Initializing the configuration account using a PDA derived from a seed value.
- Creating the LP token mint with the configuration account as its authority.
- Setting up vaults for the two tokens (mint_x and mint_y) that will be used to store liquidity.
- Storing configuration parameters such as the fee, seed, and optional authority.

---

### deposit.rs

**Purpose:**  
Manages the liquidity deposit process. This involves:
- Receiving deposits from liquidity providers.
- Calculating the required deposit amounts for tokens X and Y using a constant product formula.
- Transferring tokens from the liquidity provider’s accounts to the respective vaults.
- Minting LP tokens for the provider based on the deposited amounts and current pool state.

---

### swap.rs

**Purpose:**  
Handles token swaps between the two tokens in the pool. Key functions include:
- Accepting swap requests from users.
- Using the constant product curve model to determine how many tokens to deposit into the pool and how many tokens to withdraw.
- Transferring tokens between user accounts and vaults, ensuring that the swap meets a specified minimum output amount.

---

### withdraw.rs

**Purpose:**  
Allows liquidity providers to withdraw their share of liquidity. The process involves:
- Calculating withdrawal amounts for tokens X and Y based on the LP tokens being burned.
- Transferring tokens from the vaults back to the liquidity provider’s associated token accounts.
- Burning the corresponding amount of LP tokens from the provider’s account.
- Ensuring that the withdrawn amounts meet minimum expected values.

---

## Getting Started

1. **Prerequisites:**  
   - Install the Solana CLI.
   - Install the Anchor CLI.
   - Ensure Rust is installed.

2. **Build the Project:**  
   Run the following command to build the program:
   ```bash
   anchor build

3. **Deploy the Program:**
    Deploy the program to your chosen Solana cluster:
    ```bash
    anchor deploy

4. **Interact with the AMM:**
    Use the provided tests or your own client scripts to:

- Initialize the AMM pool.
- Deposit liquidity.
- Perform token swaps.
- Withdraw liquidity.