import bs58 from 'bs58';
import promptSync from 'prompt-sync';

const prompt = promptSync();

function base58ToWallet() {
    console.log("Enter Base58 string:");
    const base58 = prompt({});
    try {
        const wallet = bs58.decode(base58);
        console.log("Wallet format (byte array):", Array.from(wallet));
    } catch (error: any) {
        console.error("Invalid Base58 string:", error.message);
    }
}

function walletToBase58() {
    console.log("Enter wallet byte array (comma-separated):");
    const walletInput = prompt({});
    try {
        const wallet = walletInput.split(",").map(Number);
        const base58 = bs58.encode(Buffer.from(wallet));
        console.log("Base58 string:", base58);
    } catch (error: any) {
        console.error("Invalid wallet format:", error.message);
    }
}

console.log("Choose an option:");
console.log("1: Convert Base58 to Wallet");
console.log("2: Convert Wallet to Base58");
const choice = prompt("Enter choice (1/2): ");

if (choice === '1') {
    base58ToWallet();
} else if (choice === '2') {
    walletToBase58();
} else {
    console.log("Invalid choice.");
}
