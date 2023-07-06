use ethers::providers::{Http, Provider};
use numeraire::Numeraire;
use pool::V3Pool;

use std::sync::Arc;

pub mod constants;
pub mod ethers_pool;
pub mod numeraire;
pub mod pool;

const provider: &str = "";

const mainnet_usdc: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";

const mainnet_weth: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";

#[tokio::main]
async fn main() {
    let middleware =
        Arc::from(Provider::<Http>::try_from(provider).expect("Failed to parse provider"));

    let factory = Arc::from(ethers_pool::Factory::new(
        constants::FACTORY_ADDRESS.clone(),
        middleware.clone(),
    ));

    let handle = std::thread::spawn(move || {
        let factory = factory.clone();

        let pool = factory
            .pool(
                mainnet_usdc.parse().unwrap(),
                mainnet_weth.parse().unwrap(),
                pool::FeeTier::Mid,
            )
            .unwrap();

        if pool.token0() == mainnet_usdc.parse().unwrap() {
            return pool
                .pool_price()
                .expect("pool price")
                .price_in(numeraire::Token::Zero);
        } else {
            return pool
                .pool_price()
                .expect("pool price")
                .price_in(numeraire::Token::One);
        }
    });

    println!("{:?}", handle.join().unwrap())
}
