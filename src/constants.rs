use alloy::primitives::{Address, address};

pub const MULTICALL3: Address = address!("cA11bde05977b3631167028862bE2a173976CA11");

pub struct ChainConstants {
    pub factory: Address,
    pub manager: Address,
    pub quoter_v2: Address,
    pub supports_multicall: bool,
}

pub const MAINNET: ChainConstants = ChainConstants {
    factory: address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
    manager: address!("C36442b4a4522E871399CD717aBDD847Ab11FE88"),
    quoter_v2: address!("61fFE014bA17989E743c5F6cB21bF9697530B21e"),
    supports_multicall: true,
};

pub const ARBITRUM: ChainConstants = ChainConstants {
    factory: address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
    manager: address!("C36442b4a4522E871399CD717aBDD847Ab11FE88"),
    quoter_v2: address!("61fFE014bA17989E743c5F6cB21bF9697530B21e"),
    supports_multicall: true,

};

pub const OPTIMISM: ChainConstants = ChainConstants {
    factory: address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
    manager: address!("C36442b4a4522E871399CD717aBDD847Ab11FE88"),
    quoter_v2: address!("61fFE014bA17989E743c5F6cB21bF9697530B21e"),
    supports_multicall: true,
};

pub const POLYGON: ChainConstants = ChainConstants {
    factory: address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
    manager: address!("C36442b4a4522E871399CD717aBDD847Ab11FE88"),
    quoter_v2: address!("61fFE014bA17989E743c5F6cB21bF9697530B21e"),
    supports_multicall: true,
};

pub const BASE: ChainConstants = ChainConstants {
    factory: address!("33128a8fC17869897dcE68Ed026d694621f6FDfD"),
    manager: address!("03a520b32C04BF3bEEf7BEb72E919cf822Ed34f1"),
    quoter_v2: address!("3d4e44Eb1374240CE5F1B871ab261CD16335B76a"),
    supports_multicall: true,
};

// todo check multicall
pub const BNB: ChainConstants = ChainConstants {
    factory: address!("dB1d10011AD0Ff90774D0C6Bb92e5C5c8b4461F7"),
    manager: address!("7b8A01B39D58278b5DE7e48c8449c9f4F5170613"),
    quoter_v2: address!("78D78E420Da98ad378D7799bE8f4AF69033EB077"),
    supports_multicall: false,
};

// todo check multicall
pub const AVAX: ChainConstants = ChainConstants {
    factory: address!("740b1c1de25031C31FF4fC9A62f554A55cdC1baD"),
    manager: address!("655C406EBFa14EE2006250925e54ec43AD184f8B"),
    quoter_v2: address!("be0F5544EC67e9B3b2D979aaA43f18Fd87E6257F"),
    supports_multicall: false,
};

// todo check multicall
pub const CELO: ChainConstants = ChainConstants {
    factory: address!("AfE208a311B21f13EF87E33A90049fC17A7acDEc"),
    manager: address!("3d79EdAaBC0EaB6F08ED885C05Fc0B014290D95A"),
    quoter_v2: address!("82825d0554fA07f7FC52Ab63c961F330fdEFa8E8"),
    supports_multicall: false,
};

pub static NETWORKS: phf::Map<u64, ChainConstants> = phf::phf_map! {
    1_u64 => MAINNET,
    42161_u64 => ARBITRUM,
    10_u64 => OPTIMISM,
    137_u64 => POLYGON,
    8453_u64 => BASE,
    56_u64 => BNB,
    43114_u64 => AVAX,
    42220_u64 => CELO,
};
