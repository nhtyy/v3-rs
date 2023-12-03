use ethers::types::Address;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref FACTORY_ADDRESS: Address = "0x1F98431c8aD98523631AE4a59f267346ea31F984"
        .parse::<Address>()
        .unwrap();

    pub static ref POSITION_ADDRESS: Address = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88"
        .parse::<Address>()
        .unwrap();
}
