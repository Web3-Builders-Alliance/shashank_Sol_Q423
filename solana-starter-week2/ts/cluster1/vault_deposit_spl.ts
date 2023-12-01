import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "./wallet/wba-wallet.json";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

const program = new Program<WbaVault>(
  IDL,
  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
  provider
);
// Create a random keypair
const vaultState = new PublicKey("2SAziqFY7VY5187gRzvRjc6kZxMzn2GwratsyeWE4cbF");
// Create the PDA for our enrollment account
// Seeds are "auth", vaultState
const [vaultAuth, _vaultAuth_bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("auth"), Buffer.from(vaultState.toBuffer())],
  program.programId
);

// Create the vault key
// Seeds are "vault", vaultAuth
const [vault_key, _vault_bump] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), vaultAuth.toBuffer()],
  program.programId
);
const token_decimals = 1000000;

// Mint address
//get the spl_int address and  then mint it
const mint = new PublicKey("4PQkf4qi6DL9ETDEwWwMS6T5K5j6FJfwqE5kyjNxPQtm");

// Execute our enrollment transaction
(async () => {
  try {
    console.log("log here");
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const ownerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    console.log(ownerAta.address.toBase58());
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const vaultAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      vaultAuth,
      true,
      commitment
    );

    console.log(vaultAta.address.toBase58());
    const signature = await program.methods
      .depositSpl(new BN(100 * token_decimals))
      .accounts({
        owner: keypair.publicKey,
        vaultAuth,
        vaultState,
        systemProgram: SystemProgram.programId,
        ownerAta: ownerAta.address,
        vaultAta: vaultAta.address,
        tokenMint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([keypair])
      .rpc();
    console.log(
      `Deposit success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();