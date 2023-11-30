import { Commitment, Connection, Keypair, Transaction, sendAndConfirmTransaction, PublicKey, Signer } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import {createMetadataAccountV3, burnNft } from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { publicKey, signerIdentity, createSignerFromKeypair } from '@metaplex-foundation/umi';
import { publicKey as publicKeySerializer, string } from '@metaplex-foundation/umi/serializers';
import { base58 } from "@metaplex-foundation/umi/serializers";

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

//Create a Umi instance
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);


// Whilst Umi only relies on the Signer interface to request signatures from a wallet, it also defines a Keypair type and a KeypairSigner type that are explicitly aware of their secret key.
// We can generate new Keypair object with EdDSA interface with this function:
// From a secret key [our case]
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
// From a seed -> const myKeypair = umi.eddsa.createKeypairFromSeed(mySeed);
// Create a new random keypair -> const myKeypair = umi.eddsa.generateKeypair();

// In order to use these keypairs as signers throughout your application, you can use the createSignerFromKeypair helper method:
const signerKeypair = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signerKeypair));

//We can create a new valid public key from a variety of inputs using the publicKey helper method [NB this are just publickey and are NOT signers]:
// From a base58 string [Our case]
const mint =  publicKey('9iJARb4ZchofYTHzerUSqp1qm63kQ7GsyFpaZjp1TJLW')
const tokenMetadataProgramId = publicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
// From a 32-byte buffer -> publicKey(new Uint8Array(32));
// From a PublicKey or Signer type -> publicKey(someWallet as PublicKey | Signer);

// Each seed must be serialized as a Uint8Array to be utilized as seeds.
const seeds = 
  [string({ size: 'variable' }).serialize('metadata'),
  publicKeySerializer().serialize(tokenMetadataProgramId),
  publicKeySerializer().serialize(mint),
];
//To derive a new PDA, we can use the findPda method of the EdDSA interface.
const metadata_pda = umi.eddsa.findPda(tokenMetadataProgramId, seeds);

(async () => {
    let tx = createMetadataAccountV3(
        umi,
        {
            //metadata: metadata_pda, 
            mint: mint,
            mintAuthority: signerKeypair,
            //payer: signerKeypair,
            updateAuthority: keypair.publicKey,
            data: {
                name: "L0STE - Test Token #1",
                symbol: "TST",
                uri: "https://arweave.net/euAlBrhc3NQJ5Q-oJnP10vsQFjTV7E9CgHZcVm8cogo",
                sellerFeeBasisPoints: 1000,
                creators: [
                    {address: keypair.publicKey, verified: true, share: 100 }
                ],
                collection: null,
                uses: null,
            },
            isMutable: true,
            collectionDetails: null,
        }
    );
    // console.log(tx)
    
    let result = await tx.sendAndConfirm(umi);
    // console.log(result)
    const signature = base58.deserialize(result.signature);
    // console.log(signature)
    console.log(`tx hash: `, signature);
})();