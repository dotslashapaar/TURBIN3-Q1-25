import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowSolA } from "../target/types/escrow_sol_a";
import { seed } from "@coral-xyz/anchor/dist/cjs/idl";
import { BN } from "bn.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAccount, createMint, getAccount, getAssociatedTokenAddressSync, getMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { associatedAddress } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("escrow-sol-a", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.EscrowSolA as Program<EscrowSolA>;

  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let makerAtaA: anchor.web3.PublicKey;
  let makerAtaB: anchor.web3.PublicKey;
  let takerAtaA: anchor.web3.PublicKey;
  let takerAtaB: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let escrow: anchor.web3.PublicKey;

  const maker = Keypair.generate();
  const taker = Keypair.generate();
  const seed = new anchor.BN(1);
  const depositAmount = new anchor.BN(50);
  const receiveAmount = new anchor.BN(50);
  
  before(async () => {
    const makerAirdrop = await connection.requestAirdrop(maker.publicKey, 7 * LAMPORTS_PER_SOL);
    const takerAirdrop = await connection.requestAirdrop(taker.publicKey, 7 * LAMPORTS_PER_SOL);

    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: makerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    await connection.confirmTransaction({
      signature: takerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

    mintA = await createMint(connection, maker, maker.publicKey, null, 6);
    mintB = await createMint(connection, taker, taker.publicKey, null, 6);

    makerAtaA = await createAccount(provider.connection, maker, mintA, maker.publicKey);
    // makerAtaA = await createAccount(connection, maker, mintA, maker.publicKey);
    makerAtaB = await createAccount(connection, maker, mintB, maker.publicKey);

    takerAtaA = await createAccount(connection, taker, mintA, taker.publicKey);
    takerAtaB = await createAccount(connection, taker, mintB, taker.publicKey);
    
    await mintTo(connection, maker, mintA, makerAtaA, maker, 1000);
    await mintTo(connection, taker, mintB, takerAtaB, taker, 1000);

    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toBuffer("le", 8)],
      program.programId
    );

    vault = await anchor.utils.token.associatedAddress({
      mint: mintA,
      owner: escrow
    });

  });
  
  
  it("Lets make an Escrow", async () => {
    // Add your test here.
   
    await program.methods
    .make(
      seed,
      depositAmount,
      receiveAmount,
    )
    .accountsPartial({
      maker: maker.publicKey,
      mintA, 
      mintB, 
      makerAtaA,
      escrow, 
      vault, 
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc();
    
    // const escrowAccount = await program.account.escrow.fetch(escrow);
    // assert.ok(escrowAccount.maker.equals(maker.publicKey));
    // assert.ok(escrowAccount.mintA.equals(mintA));
    // assert.ok(escrowAccount.mintB.equals(mintB));
    // assert.ok(escrowAccount.receive.eq(receiveAmount));

    const vaultAccount = await getAccount(provider.connection, vault);
    assert.ok(vaultAccount.amount === BigInt(depositAmount.toString()));
  });

  it("Take Escrow Offer!", async () => {
    const takerAtaABefore = await getAccount(provider.connection, takerAtaA);
    const takerAtaBBefore = await getAccount(provider.connection, takerAtaB);

    const mintAInfo = await getMint(provider.connection, mintA);
    const mintBInfo = await getMint(provider.connection, mintB);
  
    // console.log("Mint A decimals:", mintAInfo.decimals);
    // console.log("Mint B decimals:", mintBInfo.decimals);
    // console.log("Vault balance:", (await getAccount(provider.connection, vault)).amount.toString());
    // console.log("Taker ATA B balance:", (await getAccount(provider.connection, takerAtaB)).amount.toString());

    const escrowAccount = await program.account.escrow.fetch(escrow);
    console.log("Escrow state:", {
      maker: escrowAccount.maker.toString(),
      receiveAmount: escrowAccount.receive.toString(),
      mintA: escrowAccount.mintA.toString(),
      mintB: escrowAccount.mintB.toString()
    });


    await program.methods
    .take()
    .accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA,
      mintB,
      takerAtaB,
      makerAtaB,
      escrow,
      vault,
      takerAtaA,
      systemProgram :SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc()
    // console.log("Take Tx Signature: ", tx);
  });

  it("Refund Escrow: ", async() => {

    //Make offer
    await program.methods
    .make(
      seed,
      depositAmount,
      receiveAmount,
    )
    .accountsPartial({
      maker: maker.publicKey,
      mintA, 
      mintB, 
      makerAtaA,
      escrow, 
      vault, 
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc();

    // Refund
    await program.methods
    .refund()
    .accountsPartial({
      maker: maker.publicKey,
      mintA,
      makerAtaA,
      escrow,
      vault,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc()

  });

});


