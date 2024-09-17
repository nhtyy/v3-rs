# v3-rs

### Motivation

v3-rs is a Rust library for interacting with Uniswap v3 Pools.

v3-rs currently has high level functionality for:
- computing the optimal swap amount for a given price/pool 
- computing the LP balances owned by a given address (NFT positions only)
and implements many low level helpers to make additional functionality easy

We also expose some useful [crate::types] that can help you seemlessly interact with on chain pools without having to think about token decimals or anything like that.

Please see the examples in the repo for more information.

#### Creating Pool prices from human readable prices
To create a pool price, (account for the decimals) you can use [PoolPrice::from_human_readable], which implements `Into<Price>`
 
#### Pool Price Example
```rust
    use v3_rs::{AlloyFactory, FeeTier, constants::MAINNET, V3Pool};
    use alloy::providers::ProviderBuilder;
    use alloy::primitives::{address, Address};

    const WETH: Address = address!("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
    const DAI: Address = address!("6b175474e89094c44da98b954eedeac495271d0f");

    #[tokio::main]
    async fn main() {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_http("https://cloudflare-eth.com".parse().unwrap());

        let factory = AlloyFactory::new(MAINNET.factory, &provider);

        let pool = factory.pool(WETH, DAI, FeeTier::Mid).await.unwrap();

        let numeraire_idx = pool.position_of(&DAI).expect("Token should be in the pool");

        let price = pool.pool_price(numeraire_idx).await.unwrap();

        println!("Pool Price: {}", price);
    }
```

# Note:
v3-rs doesnt actually help you *interact* with pools on chain, only extract useful information from them.

### Usage

see `cargo doc --open` and the examples in the repo.

### Plans
- add quoting and path building