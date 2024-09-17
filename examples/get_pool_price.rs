use v3_rs::{AlloyFactory, FeeTier, constants::MAINNET, V3Pool};
use alloy::providers::ProviderBuilder;
use alloy::primitives::{address, Address};

const WETH: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const DAI: Address = address!("6b175474e89094c44da98b954eedeac495271d0f");

#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new().with_recommended_fillers().on_http("https://cloudflare-eth.com".parse().unwrap());

    let factory = AlloyFactory::new(MAINNET.factory, &provider);

    let pool = factory.pool(WETH, DAI, FeeTier::Mid.into()).await.unwrap();

    let numeraire_idx = pool.position_of(&DAI).expect("Token should be in the pool");

    let price = pool.pool_price(numeraire_idx).await.unwrap();

    println!("Pool Price: {}", price);
}