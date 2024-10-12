use alloy::primitives::{address, Address};
use alloy::providers::ProviderBuilder;
use v3_rs::{constants::MAINNET, Factory, FeeTier};

const TOKEN_A: Address = address!();
const TOKEN_B: Address = address!();
const LP: Address = address!();

#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http("https://cloudflare-eth.com".parse().unwrap());

    let factory = Factory::new(MAINNET.factory, &provider);

    let pool = factory.pool(TOKEN_A, TOKEN_B, FeeTier::Mid).await.unwrap();

    let balance = pool.lp_balance(LP).await.unwrap();

    println!("LP Balance: {:?}", balance);
}
