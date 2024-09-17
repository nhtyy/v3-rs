use alloy::primitives::{address, Address};
use alloy::providers::ProviderBuilder;
use v3_rs::{constants::MAINNET, math::Price, AlloyFactory, FeeTier, PriceExt, V3Pool};

const WETH: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
const DAI: Address = address!("6b175474e89094c44da98b954eedeac495271d0f");

#[tokio::main]
async fn main() {
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http("https://cloudflare-eth.com".parse().unwrap());

    let factory = AlloyFactory::new(MAINNET.factory, &provider);

    let pool = factory.pool(WETH, DAI, FeeTier::Mid).await.unwrap();

    let numeraire_idx = pool.position_of(&DAI).expect("Token should be in the pool");

    let price = pool.pool_price(numeraire_idx).await.unwrap();

    println!("Pool Price: {}", price);

    // lets scale down the price 5%
    let new_price = rug::Float::from(price) * 0.95;

    // lets find the optimal swap for this price
    let optimal = pool
        .optimal_swap_for_price(Price::new(new_price).unwrap())
        .await
        .unwrap();

    println!("Optimal Swap: {:?}", optimal);
    println!("fee needed: {}", optimal.fee_amount())
}