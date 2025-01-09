mod programs;
use solana_sdk::system_program;
use solana_sdk::signature::Signer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::read_keypair_file;
use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new("https://api.devnet.solana.com");
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs {
            github: b"NIXBLACK11".to_vec()
        };
        let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recentblockhash");
        let transaction = Turbin3PrereqProgram::complete(&[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], blockhash );
        let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed to send transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }
}