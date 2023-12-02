import * as anchor from "@coral-xyz/anchor";
import { Program,Wallet } from "@coral-xyz/anchor";
import { AnchorVoteSolb } from "../target/types/anchor_vote_solb";
import { Keypair,PublicKey ,LAMPORTS_PER_SOL,SystemProgram} from "@solana/web3.js";
// import { sha256 } from "@coral-xyz/anchor/dist/cjs/utils";
import {createHash} from "crypto";
// import { Program, Wallet, AnchorProvider, Address } from "@coral-xyz/anchor";

import wallet from "../votepLE9CVde7gM2K3sQrsA9FJveZRz4CfhSgwCCEGo.json";

describe("anchor-vote-solb", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.AnchorVoteSolb as Program<AnchorVoteSolb>;

  const provider = anchor.getProvider();

  // Import our dev wallet keypair from the wallet file
  const signer1 = Keypair.fromSecretKey(new Uint8Array(wallet));
  const signer = Keypair.generate();

  const site = "google.com";

  const hash = createHash('sha256');
  hash.update(Buffer.from(site));
  const seeds = [hash.digest()];

  const vote = PublicKey.findProgramAddressSync(seeds, program.programId)[0];

  const confirm = async (signature: string): Promise<string> => {
    const block = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature,
      ...block
    })
    return signature
  }

  // https://explorer.solana.com/?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899
  // anchor build
  // solana-test-validator
  // anchor test --skip-local-validator
  // anchor deploy
  // solana-keygen grind --starts-with=vote:1

  const log = async(signature: string) => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    return signature;
  }

  it("Airdrop", async () => {
    await provider.connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log)
  })

  it("Airdrop", async () => {
    await provider.connection.requestAirdrop(signer1.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log)
  })

  it("Initialize", async () => {
    const tx = await program.methods
    .initialize(site)
    .accounts({
      signer: signer.publicKey,
      vote
    })
    .signers([
      signer
    ])
    .rpc()
    .then(confirm)
    .then(log);
    console.log("Your transaction signature", tx);
  });
});
