mod programs;
use crate::programs::Turbin3_prereq::{CompleteArgs, WbaPrereqProgram, UpdateArgs};



#[cfg(test)]

mod tests {
    use bs58;
    use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{self, read_keypair_file, Keypair, Signer},
        signer::{self, keypair},
        system_program,
        transaction::Transaction,
    };
    use std::io::{self, stdin, BufRead};
    use std::str::FromStr;
    
    use crate::programs::Turbin3_prereq::{CompleteArgs, WbaPrereqProgram};
    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana Wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file: ");
        println!("{:?}", kp.to_bytes());
        
    }
    
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58: ");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is: ");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here: ");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("2CquYcQoBGv8MiiMfP3Lgut79oLCtDbCTrB6fnQm1WeG").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get ");
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance-fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );

    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("turbine.json").expect("Couldn't find wallet file");
        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        let args = CompleteArgs {
            github: b"dotslashapaar".to_vec(),
        };
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Checkout your TX here: https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }

}