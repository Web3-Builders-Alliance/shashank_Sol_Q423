import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "./wallet/wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
// const mint = new PublicKey("9Yp4qPKnrns9cXSHQ3eQzpNKzvSEn21HWNAmaTsq1WrT");
const mint = new PublicKey("37jAcRv8mqCLnZ4HQ83bmkoJM2w9oF5MWqWptXRrXyE9");
// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

(async () => {
    try {
        const ata=await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        // Create an ATA
        // const ata = ???
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        const mintTx = await mintTo(connection, keypair, mint, ata.address, keypair.publicKey, token_decimals);
        // Mint to ATA
        // const mintTx = ???
        console.log(`Your mint txid: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
