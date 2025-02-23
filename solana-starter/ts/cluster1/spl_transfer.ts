import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("GpxXfJR94cwFxZhKxrMnwqXxdur5YKR8qoUamjTLAK2j");

// Recipient address
const to = new PublicKey("pYb31c3sYzCnaEUhgRW9jM3ontTmmRKX6SF3o7T1tfy");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);
        // Transfer the new token to the "toTokenAccount" we just created
        const decimals = 6; 
        const amount = 1 * 10 ** decimals;
        const tx = await transfer(connection, keypair, fromAta.address, toAta.address, keypair.publicKey, amount);
        console.log(`Transfer successful! Transaction ID: ${tx}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// Transfer successful! Transaction ID: xkibKyE6sWYK2rt2e8EAqyjYY3rhQZSYkYTQkVXtgFmdaPbu1V6HVFEYSVdxqkfGUx68Lt5YHX61sNg8YTA3fQV

