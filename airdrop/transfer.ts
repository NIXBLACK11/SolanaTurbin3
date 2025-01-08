import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js"
import wallet from "./dev-wallet.json"

const connection = new Connection("https://api.devnet.solana.com");

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("6yaxBaZMnkhiP7NHgZzweUjHE1wcsVy2nxWo3VMSLKNE");

(async () => {
    try {
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL / 10,
            })
        );

        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;

        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        );

        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();