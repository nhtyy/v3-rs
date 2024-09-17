use alloy::primitives::address;
use alloy::primitives::Address;

pub struct ChainConstants {
    pub factory: Address,
    pub manager: Address,
    pub quoter_v2: Address,
}

pub const MAINNET: ChainConstants = ChainConstants {
    factory: address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
    manager: address!("C36442b4a4522E871399CD717aBDD847Ab11FE88"),
    quoter_v2: address!("61fFE014bA17989E743c5F6cB21bF9697530B21e")
};

pub static NETWORKS: phf::Map<u64, ChainConstants> = phf::phf_map! {
    1_u64 => MAINNET
};
