use factom::*;
use sha2::{Digest, Sha256};

// Example will get the Entry Credit addresses for a certain chain in the last block
// In this case it will be the pegnet chain

// Pegnet Tx Chain
const TX_CHAIN: &str = "cffce0f409ebba4ed236d49d89c70e4bd1f1367d86402a3363366683265a242d";
// Address checksum length
const CHECKSUM_LENGTH: usize = 4;
// Entrycredit publickey prefix
const EC_PUB: [u8; 2] = [0x59, 0x2a];

#[tokio::main]
async fn main() {
    let mut ec_addresses = Vec::new();
    let client = Factom::open_node();
    let heights = factomd::heights(&client).await.expect("Fetching Heights");
    let e_height = heights.result.entryblockheight;
    let e_block = block::ecblock_by_height(&client, e_height as u32)
        .await
        .expect("Fetching Entry Credit Block by Height");
    let entries = e_block.result.ecblock.body.entries;
    for entry in entries {
        match entry.entryhash {
            Some(hash) => {
                let entryobj = entry::entry(&client, &hash).await.expect("Fetching Entry");
                match entryobj.result.chainid.as_str() {
                    TX_CHAIN => match entry.ecpubkey {
                        Some(key) => {
                            let ecpubbytes = hex::decode(&key).expect("Decoding String");
                            ec_addresses.push(readable(&EC_PUB, &ecpubbytes));
                        }
                        None => (),
                    },
                    _ => (),
                }
            }
            None => (),
        }
    }
    dbg!(ec_addresses);
}

// Will output a human readable String
pub fn readable(prefix: &[u8], raw: &[u8]) -> String {
    let (mut key, mut output) = (Vec::new(), Vec::new());
    key.extend_from_slice(prefix);
    key.extend_from_slice(&raw);
    let checksum = &double_sha(&key)[..CHECKSUM_LENGTH];
    output.extend_from_slice(&key);
    output.extend_from_slice(checksum);
    bs58::encode(output).into_string()
}

fn double_sha(input: &[u8]) -> [u8; 32] {
    let h1 = Sha256::digest(input);
    let h2 = Sha256::digest(&h1);
    slice_to_array(&h2)
}

fn slice_to_array(slice: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, byte) in slice.iter().enumerate() {
        out[i] = *byte;
    }
    out
}
