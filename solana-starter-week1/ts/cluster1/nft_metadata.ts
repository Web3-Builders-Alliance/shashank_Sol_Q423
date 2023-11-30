import {
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import wallet from "../wba-wallet.json";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
  try {
    const image =
      "https://arweave.net/oHwpYNaKnGOa2ZqmVNtgPwTdL-KJaWF--cJGfb6vMaY";
    const metadata = {
      name: "Generug #1",
      symbol: "RUG",
      description: "An extremely rare and exotic rug",
      image: image,
      attributes: [
        {
          trait_type: "Background",
          value: "Off-white",
        },
        {
          trait_type: "Rarity",
          value: "Godlike",
        },
        {
          trait_type: "Pixellation",
          value: "Ridiculous",
        },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: image
            ,
          },
        ],
      },
      creators: [],
    };
    const myUri = await bundlrUploader.uploadJson(metadata);
    console.log("Your image URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();