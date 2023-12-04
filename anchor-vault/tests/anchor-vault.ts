import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Keypair,PublicKey ,LAMPORTS_PER_SOL,SystemProgram} from "@solana/web3.js";
// import { AnchorVault } from "../target/types/anchor_vault";
import { AnchorVault } from "../target/types/anchor_vault";

describe("anchor-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorVault as Program<AnchorVault>;
  const provider = anchor.getProvider();
  const signer = Keypair.generate();

  const vault=PublicKey.findProgramAddressSync([Buffer.from("vault"),signer.publicKey.toBuffer()],program.programId)[0]

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
  // anchor idl init --filepath target/idl/anchor_vault.json voteLp9rMPNKRRWwt7Eya81zyFCjzwk2T5XYRJQYtLf

  const log = async(signature: string) => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
    return signature;
  }

  it("Airdrop", async () => {
    await provider.connection.requestAirdrop(signer.publicKey, LAMPORTS_PER_SOL * 10)
    .then(confirm)
    .then(log)
  })

  it("Deposit", async () => {
    await program.methods.deposit(new BN(1e9))
    .accounts({
      signer: signer.publicKey,
      vault,
      systemProgram:SystemProgram.programId
    })
    .signers([signer])
    .rpc()
    .then(confirm)
    .then(log)
  })

  it("Close", async () => {
    await program.methods.close()
    .accounts({
      signer: signer.publicKey,
      vault,
      systemProgram:SystemProgram.programId
    })
    .signers([signer])
    .rpc()
    .then(confirm)
    .then(log)
  })


  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
