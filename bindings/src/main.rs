use ethers::prelude::Abigen;
use eyre::Result;

fn rust_file_generation(source: &str, destination: &str, name: &str) -> Result<()> {
    let abi_source = source;
    let out_file = std::env::current_dir()?.join(destination);
    if out_file.exists() {
        std::fs::remove_file(&out_file)?;
    }
    Abigen::new(name, abi_source)?
        .generate()?
        .write_to_file(out_file)?;
    Ok(())
}

fn main() {
    print!("hello!");

    rust_file_generation(
        "src/abis/UniswapV3Factory.json",
        "src/IUniswapV3Factory.rs",
        "IUniswapV3Factory",
    )
    .expect("Failed to generate bindings");

    rust_file_generation(
        "src/abis/UniswapV3Pool.json",
        "src/IUniswapV3Pool.rs",
        "IUniswapV3Pool",
    )
    .expect("Failed to generate bindings");

    rust_file_generation("src/abis/ERC20.json", "src/IERC20.rs", "IERC20")
        .expect("Failed to generate bindings");

    rust_file_generation("src/abis/OFTV2.json", "src/IOFTV2.rs", "IOFTV2")
        .expect("Failed to generate bindings");

    rust_file_generation("src/abis/OFTV2.json", "src/IProxyOFTV2.rs", "IProxyOFTV2")
        .expect("Failed to generate bindings");
}
