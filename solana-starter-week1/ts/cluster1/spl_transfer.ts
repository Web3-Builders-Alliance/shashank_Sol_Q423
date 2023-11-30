import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("vRe9T4vc4HXsnUq7viE7iAtHtkj8p9mT38EbCgLEDDZ");

// Recipient address
const to = new PublicKey("HDaVYzEeTsu5v3YzojroWt3GLAhHNvDja6VnCjURwJHk");

(async () => {
    try {
        // Create an ATA
        const fromAta = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            keypair.publicKey

        )
        console.log(`Your ata is: ${fromAta.address.toBase58()}`);

        // Create an ATA
        const toAta = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            to

        )
        console.log(`Your to ata is: ${toAta.address.toBase58()}`);

        // 
        const transferTx = await transfer(
            connection, 
            keypair,  
            fromAta.address, 
            toAta.address,
            keypair, 
            1_000_000 
        )
        console.log(`Your transfer txid: ${transferTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()