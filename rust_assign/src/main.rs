// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     pubkey::Pubkey,
//     signature::{read_keypair_file, Signer, Keypair},
// };

// const RPC_URL: &str = "https://api.devnet.solana.com";

// fn main() {
//     let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

//     let client = RpcClient::new(RPC_URL);

//     match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
//         Ok(signature) => {
//             println!("Success! Check out your TX here:");
//             println!(
//                 "https://explorer.solana.com/tx/{}?cluster=devnet",
//                 signature.to_string()
//             );
//         }
//         Err(e) => {
//             println!("Oops, something went wrong: {}", e.to_string());
//         }
//     }
// }
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    let to_pubkey = Pubkey::from_str("6yaxBaZMnkhiP7NHgZzweUjHE1wcsVy2nxWo3VMSLKNE").unwrap();

    let rpc_client = RpcClient::new(RPC_URL);

    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    if balance == 0 {
        println!("The wallet balance is zero. No funds to transfer.");
        return;
    }

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to calculate fee");

    if balance <= fee {
        println!(
            "Insufficient funds. Wallet balance ({}) is less than the transaction fee ({} lamports).",
            balance, fee
        );
        return;
    }

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("All funds transferred successfully! Check out your TX here:");
            println!(
                "https://explorer.solana.com/tx/{}/?cluster=devnet",
                signature
            );
        }
        Err(e) => {
            println!("Failed to empty wallet: {}", e);
        }
    }
}

