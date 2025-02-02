# Anchor Vault Project

This project is an Anchor-based vault program on Solana. It provides a simple interface for users to initialize a vault, deposit lamports (SOL) into the vault, withdraw lamports from the vault, and close the vault. The program leverages Program Derived Addresses (PDAs) to securely manage state and vault accounts.

---

## 3D Overview

- **Decentralized:**  
  The program operates on Solana’s high-performance blockchain without a central authority. It uses PDAs to ensure that the vault and state accounts are securely derived and managed.

- **Dynamic:**  
  Users can:
  - **Initialize** a new vault with a corresponding state account.
  - **Deposit** SOL into the vault.
  - **Withdraw** SOL from the vault.
  - **Close** the vault and reclaim any remaining funds.

- **Distributed:**  
  The program integrates with Solana’s system program to perform secure SOL transfers through cross-program invocations (CPIs). This ensures that operations are handled in a trustless and decentralized manner.

---

## Project Structure & Explanation

### lib.rs

**Purpose:**  
Acts as the entry point of the program. This file declares the program ID and exposes the following instructions:
- **initialize:** Sets up the vault by creating a state account and deriving the vault address.
- **deposit:** Allows a user to deposit SOL into the vault.
- **withdraw:** Allows a user to withdraw SOL from the vault.
- **close:** Closes the vault by transferring its lamports back to the user and cleaning up the state account.

---

### vault_state.rs

**Purpose:**  
Defines the `VaultState` account structure. This state holds the bump values used for deriving the vault and state PDAs. It is a simple account that only stores:
- The bump for the vault account.
- The bump for the state account.

---

### initialize.rs

**Purpose:**  
Handles the initialization of the vault. During initialization:
- The state account is created with a PDA derived from a constant string and the signer’s public key.
- The vault is set as a system account derived from the state account.
- The bump values for both the state and vault are stored in the `VaultState` account.

---

### payment.rs

**Purpose:**  
Manages both deposit and withdrawal functionalities.
- **Deposit:**  
  Transfers a specified amount of SOL from the signer’s account to the vault.
- **Withdraw:**  
  Transfers a specified amount of SOL from the vault back to the signer’s account.  
  The withdrawal process uses the stored bump value to correctly sign the transfer.

---

### close.rs

**Purpose:**  
Handles closing the vault. In this process:
- The vault’s lamports are transferred back to the user.
- The state account is closed and its rent-exempt balance is returned to the user.
- The operation uses the appropriate bump seeds to authorize the transfer.

---

## Getting Started

1. **Prerequisites:**  
   - Ensure you have the Solana CLI installed.
   - Ensure you have the Anchor CLI installed.
   - Install Rust and set up your development environment.

2. **Build the Project:**  
   Compile the program using:
   ```bash
   anchor build

3. **Deploy the Program:**
  Deploy the program to your desired Solana cluster:
   ```bash
   anchor deploy

4. **Interact with the Program:**
Use Anchor tests or your own client scripts to:

- Initialize a vault.
- Deposit SOL into the vault.
- Withdraw SOL from the vault.
- Close the vault and reclaim any remaining funds.

