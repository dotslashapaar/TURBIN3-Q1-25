# Final Project - WBA Program Completion

Transaction link: [View Transaction on Solana Explorer](https://explorer.solana.com/tx/4JYK8uPUoe9b96tAcGP5PsQThdDAyYqkUicnYYQ5DwHJ8d9iXH7ijEhft9V3rN8cFxHJT5bQzXy6vhZYWmNjpjss?cluster=devnet)
https://explorer.solana.com/tx/4JYK8uPUoe9b96tAcGP5PsQThdDAyYqkUicnYYQ5DwHJ8d9iXH7ijEhft9V3rN8cFxHJT5bQzXy6vhZYWmNjpjss?cluster=devnet

tx: 4JYK8uPUoe9b96tAcGP5PsQThdDAyYqkUicnYYQ5DwHJ8d9iXH7ijEhft9V3rN8cFxHJT5bQzXy6vhZYWmNjpjss

## Project Overview

This project interacts with a Solana program on the devnet. It submits a transaction to register completion of prerequisites using a GitHub username.

## Files in the Project

- `wba_prereq.ts`: Contains the Interface Definition Language (IDL) and types that describe the Solana program's structure.
- `enroll.ts`: The main TypeScript file that interacts with the Solana program. It submits a transaction to complete the program prerequisites by sending the user's GitHub account name.

## How to Run the Project

1. Install dependencies:
   ```
   yarn install
   ```

2. Compile and run the `enroll.ts` file:
   ```
   ts-node enroll.ts
   ```

3. The program will send a transaction to the Solana devnet and output the transaction hash.
