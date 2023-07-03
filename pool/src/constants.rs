use ethers::types::Address;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FACTORY_ADDRESS: Address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();
}
