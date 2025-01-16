import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/5jc8otNBcs1SL1ABuLdhoMVLtg5mSeYKwG8Mez6xgEp7"
        const metadata = {
            name: "PinkRug",
            symbol: "PNKRUG",
            description: "Get Rugged By Beautiful Pink RUGGGGGGG",
            image: image,
            attributes: [
                {trait_type: 'Rug-Type', value: 'Pixel-Persian'},
                {trait_type: 'ConstructionRug-Type', value: 'Pink-Knotted'},
                {trait_type: 'Materials', value: 'Pixel-Wool'},
                {trait_type: 'Style', value: 'Traditional'},
                {trait_type: 'Features', value: 'Geometric-Pixel-Patterns'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// https://arweave.net/AibDf1BjUZL2okSYCeWeKWRNXLWzoSVYtqJCuujSg665
// https://devnet.irys.xyz/H5YXxebLKfGpMWGNKQKWyppSnzaqN8sDF84hAKRWiomB
// https://devnet.irys.xyz/9A5iJ6etbX1sCDXU855Vv6PmnDdPGb7hDN5RSvMs6nKe