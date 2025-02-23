# NFT Staking Project

## Description
This project implements an NFT staking mechanism using the Anchor framework on Solana. Users can stake their NFTs to earn rewards, and the system manages the staking process, including initialization, staking, unstaking, and claiming rewards.

## NFT Freezing
The project includes functionality to freeze NFTs during the staking process. When an NFT is staked, it is temporarily frozen to prevent any transfers or modifications until the user unstakes it. This ensures that the NFT remains secure while it is being staked.

### Freezing and Unfreezing NFTs
- **Freeze NFT**: The `freeze` function is called when an NFT is staked, locking it in place.
- **Thaw NFT**: The `thaw` function can be called to unlock the NFT when it is unstaked.

## Installation

**To install the necessary dependencies, run the following command:**

  ```bash
  yarn install
  ```
  or
  ```bash
  npm install
  ```

## Usage

**To deploy the project, use the following command:**
  ```bash
  anchor deploy
  ```

**To run tests, use:**
  ```bash
  yarn test
  ```
  or
  ```bash
  npm test
  ```
