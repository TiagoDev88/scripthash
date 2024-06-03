use dotenv::dotenv;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::*;
use ethers_providers::Ws;
use futures::{SinkExt, StreamExt};
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::Write;
use tokio::main;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

async fn create_ws_provider() -> Result<Provider<Ws>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let WS_CONNECT = env::var("WS_CONNECT").expect("WS_CONNECT must be set");
    let ws = Ws::connect(WS_CONNECT).await?;
    let provider = Provider::new(ws);
    Ok(provider)
}

async fn query_ethereum_blocks(
    start_block: u64,
    end_block: u64,
    transaction_prefixes: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = create_ws_provider().await.unwrap();

    for block_number in start_block..=end_block {
        println!("{}", block_number);

        match provider
            .get_block(BlockId::Number(block_number.into()))
            .await
        {
            Ok(Some(block)) => {
                for tx_hash in &block.transactions {
                    match provider.get_transaction(*tx_hash).await {
                        Ok(Some(tx)) => {
                            let tx_hash_str = format!("{:?}", tx.hash);

                            if transaction_prefixes
                                .iter()
                                .any(|prefix| tx_hash_str.starts_with(prefix))
                            {
                                let mut file = OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open("ethereum_transactions.txt")?;

                                file.write_all(format!("{}\n", tx_hash_str).as_bytes())?;
                            }
                        }
                        Ok(None) => eprintln!("No transaction found for hash {:?}", tx_hash),
                        Err(e) => eprintln!("Error getting transaction: {:?}", e),
                    }
                }
            }
            Ok(None) => eprintln!("No block found for number {}", block_number),
            Err(e) => eprintln!("Error getting block: {:?}", e),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let start_block: u64 = 17248518;
    let end_block: u64 = 17254452;
    /*
       // Definir sequências de caracteres para pesquisar
        let transaction_prefixes = vec![
        "ac1ddb0fbe","1b491610bd","9f55de09b7","0xce4bfc28","3b8e3e24a3","0x21015f3d",
        "0xc622f1b5","639ac3e1eb","0x62d2ddb7","0xd6ae6798","0xf3cae9d0","0x498b764a",
        "0xc5471dc7","0x2c69ba9d","0x57bd3d22","0xdb8018d0","0x7f2748f7","0x6c89cc43",
        "0x0119e969","0x2707ee59","0xe07690e1","0x58297a19","0xe081b6d8","0xfc6b824f",
        "0x1593786d","0xa711d328","0x1419d86d","0x871ceafe","0x42b14d81","0x23260899",
        "0x86b02ca5","a3f4a09f95","0x9b9ed144","0x4fbe12fa","0x863bd845","0x72ffcecb",
        "0xb56c054a","0x1bacfefd","0x0ca7fa0c","0xf65af99a","0xd927889d","0x8df811fa",
        "0xdfb36fea","0x8e8904d8","0x7e1f0c25","0x7169ecf9","0xb0f3f2e9","eb5fda5aa3",
        "0x2ff35e69","0x8b3f0876","0x6d2311f6","0x72353bc6","0x29d49fd0","0xd2c619fd",
        "0x29ef788c","0xa489addf","0xd3294fce","0x3ca3e7b6","b0e3e2485a","0x6271f6fb",
        "77df581e68","0x1aef3d00","0x020e76e9","6cc5ab6396","0xa88c7129","0x35bcaafa",
        "0x9f92a287","3TXad5PhHa","0x6bc15ff0","0x56ac0879","0xf9ebe61e","0x602d3ebb",
        "0xbd13bb28","0x7c62446b","0x221707cf","0x9e265602","0xf621dda2","38VmZB8AYP",
        "0x469b3d25","0x27e066da","0xadd0dbb2","0x10014086","0xa4d95c32","0xa9c3ade3",
        "ad79c52b7b","0x4fb49876","a954bb8fc0","0x1b5e51b4","0x89e6439b","0x46409c0c",
        "0x00595223","0xb7af29bc","0x4393f817","fe776ded7a","b1c2b2560a","0xd16c2a13",
        "1252a2907e","0xa952d33e","93bf22760e","5Srgiex2gs","0xdd620e3b","225aab9242",
        "0x4ea6f770","0xa200a359","72f8a16f53","0x80e01ec9","0xd6e5c451","0x58550217",
        "0x60d9aa39","0xc2dce076","0xdbc0c48b","0x016b516a","0xdd42b073","4HC4Sgw9NY",
        "4LwTkJ9SQH","2asrPeXze5","0x20892f9d","0x733a0fa2","0x4826f23a","0xc0eda4ec",
        "0x0c9a3a91","0xfca59a73","0xe7360dbf","0x7b099e0f","0x7ada39b8","0xb3ec8836",
        "0xa021f3e5","0x3653bbfb","0x9ae71e46","0x98e4c00f","0xd8450316","0x5a9ca045",
        "0x93990012","0x39e2cb05","0x3ba16205","0x34aab2c7","0x13152e9f","0x48d6a553",
        "0x90f41014","0x100e3fd1","0x70de24c2","0xfb6ad0bd",
        ];
    */
    let transaction_prefixes = vec!["0xd927889d", "0x8df811fa"];

    query_ethereum_blocks(start_block, end_block, &transaction_prefixes).await;
}
