pub use i_uniswap_v3_pool::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_uniswap_v3_pool {
    const _: () = {
        ::core::include_bytes!(
            "/Users/nuhhtyy/Desktop/alongside/index-system-v2/sdk/bindings/src/abis/UniswapV3Pool.json"
        );
    };
    #[rustfmt::skip]
    const __ABI: &str = "[\n  { \"inputs\": [], \"stateMutability\": \"nonpayable\", \"type\": \"constructor\" },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Burn\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"Collect\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"CollectProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"paid0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"paid1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Flash\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNextOld\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNextNew\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"IncreaseObservationCardinalityNext\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"Initialize\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Mint\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0Old\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1Old\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0New\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1New\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"SetFeeProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount0\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount1\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"Swap\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount\", \"type\": \"uint128\" }\n    ],\n    \"name\": \"burn\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount0\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount1\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0Requested\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1Requested\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collect\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"amount0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0Requested\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1Requested\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collectProtocol\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"amount0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"fee\",\n    \"outputs\": [{ \"internalType\": \"uint24\", \"name\": \"\", \"type\": \"uint24\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeGrowthGlobal0X128\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeGrowthGlobal1X128\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount0\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount1\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"flash\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNext\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"increaseObservationCardinalityNext\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint160\", \"name\": \"sqrtPriceX96\", \"type\": \"uint160\" }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"liquidity\",\n    \"outputs\": [{ \"internalType\": \"uint128\", \"name\": \"\", \"type\": \"uint128\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxLiquidityPerTick\",\n    \"outputs\": [{ \"internalType\": \"uint128\", \"name\": \"\", \"type\": \"uint128\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" },\n      { \"internalType\": \"uint128\", \"name\": \"amount\", \"type\": \"uint128\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"mint\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount0\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount1\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"name\": \"observations\",\n    \"outputs\": [\n      { \"internalType\": \"uint32\", \"name\": \"blockTimestamp\", \"type\": \"uint32\" },\n      { \"internalType\": \"int56\", \"name\": \"tickCumulative\", \"type\": \"int56\" },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityCumulativeX128\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"bool\", \"name\": \"initialized\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint32[]\", \"name\": \"secondsAgos\", \"type\": \"uint32[]\" }\n    ],\n    \"name\": \"observe\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int56[]\",\n        \"name\": \"tickCumulatives\",\n        \"type\": \"int56[]\"\n      },\n      {\n        \"internalType\": \"uint160[]\",\n        \"name\": \"secondsPerLiquidityCumulativeX128s\",\n        \"type\": \"uint160[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }],\n    \"name\": \"positions\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"liquidity\", \"type\": \"uint128\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthInside0LastX128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthInside1LastX128\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"uint128\", \"name\": \"tokensOwed0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"tokensOwed1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"protocolFees\",\n    \"outputs\": [\n      { \"internalType\": \"uint128\", \"name\": \"token0\", \"type\": \"uint128\" },\n      { \"internalType\": \"uint128\", \"name\": \"token1\", \"type\": \"uint128\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint8\", \"name\": \"feeProtocol0\", \"type\": \"uint8\" },\n      { \"internalType\": \"uint8\", \"name\": \"feeProtocol1\", \"type\": \"uint8\" }\n    ],\n    \"name\": \"setFeeProtocol\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"slot0\",\n    \"outputs\": [\n      { \"internalType\": \"uint160\", \"name\": \"sqrtPriceX96\", \"type\": \"uint160\" },\n      { \"internalType\": \"int24\", \"name\": \"tick\", \"type\": \"int24\" },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationIndex\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinality\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNext\",\n        \"type\": \"uint16\"\n      },\n      { \"internalType\": \"uint8\", \"name\": \"feeProtocol\", \"type\": \"uint8\" },\n      { \"internalType\": \"bool\", \"name\": \"unlocked\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"int24\", \"name\": \"tickLower\", \"type\": \"int24\" },\n      { \"internalType\": \"int24\", \"name\": \"tickUpper\", \"type\": \"int24\" }\n    ],\n    \"name\": \"snapshotCumulativesInside\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulativeInside\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityInsideX128\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"uint32\", \"name\": \"secondsInside\", \"type\": \"uint32\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"bool\", \"name\": \"zeroForOne\", \"type\": \"bool\" },\n      { \"internalType\": \"int256\", \"name\": \"amountSpecified\", \"type\": \"int256\" },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceLimitX96\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"swap\",\n    \"outputs\": [\n      { \"internalType\": \"int256\", \"name\": \"amount0\", \"type\": \"int256\" },\n      { \"internalType\": \"int256\", \"name\": \"amount1\", \"type\": \"int256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"int16\", \"name\": \"\", \"type\": \"int16\" }],\n    \"name\": \"tickBitmap\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"tickSpacing\",\n    \"outputs\": [{ \"internalType\": \"int24\", \"name\": \"\", \"type\": \"int24\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"int24\", \"name\": \"\", \"type\": \"int24\" }],\n    \"name\": \"ticks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidityGross\",\n        \"type\": \"uint128\"\n      },\n      { \"internalType\": \"int128\", \"name\": \"liquidityNet\", \"type\": \"int128\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthOutside0X128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthOutside1X128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulativeOutside\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityOutsideX128\",\n        \"type\": \"uint160\"\n      },\n      { \"internalType\": \"uint32\", \"name\": \"secondsOutside\", \"type\": \"uint32\" },\n      { \"internalType\": \"bool\", \"name\": \"initialized\", \"type\": \"bool\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token0\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token1\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n";
    ///The parsed JSON ABI of the contract.
    pub static IUNISWAPV3POOL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IUniswapV3Pool<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniswapV3Pool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniswapV3Pool<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniswapV3Pool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniswapV3Pool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUniswapV3Pool)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUniswapV3Pool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IUNISWAPV3POOL_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `burn` (0xa34123a7) function
        pub fn burn(
            &self,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([163, 65, 35, 167], (tick_lower, tick_upper, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collect` (0x4f1eb3d8) function
        pub fn collect(
            &self,
            recipient: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [79, 30, 179, 216],
                    (
                        recipient,
                        tick_lower,
                        tick_upper,
                        amount_0_requested,
                        amount_1_requested,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectProtocol` (0x85b66729) function
        pub fn collect_protocol(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash(
                    [133, 182, 103, 41],
                    (recipient, amount_0_requested, amount_1_requested),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fee` (0xddca3f43) function
        pub fn fee(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeGrowthGlobal0X128` (0xf3058399) function
        pub fn fee_growth_global_0x128(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([243, 5, 131, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeGrowthGlobal1X128` (0x46141319) function
        pub fn fee_growth_global_1x128(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([70, 20, 19, 25], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `flash` (0x490e6cbc) function
        pub fn flash(
            &self,
            recipient: ::ethers::core::types::Address,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 14, 108, 188], (recipient, amount_0, amount_1, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseObservationCardinalityNext` (0x32148f67) function
        pub fn increase_observation_cardinality_next(
            &self,
            observation_cardinality_next: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 20, 143, 103], observation_cardinality_next)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf637731d) function
        pub fn initialize(
            &self,
            sqrt_price_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 55, 115, 29], sqrt_price_x96)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidity` (0x1a686502) function
        pub fn liquidity(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([26, 104, 101, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxLiquidityPerTick` (0x70cf754a) function
        pub fn max_liquidity_per_tick(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([112, 207, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x3c8a7d8d) function
        pub fn mint(
            &self,
            recipient: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
            amount: u128,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [60, 138, 125, 141],
                    (recipient, tick_lower, tick_upper, amount, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observations` (0x252c09d7) function
        pub fn observations(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u32, i64, ::ethers::core::types::U256, bool),
        > {
            self.0
                .method_hash([37, 44, 9, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `observe` (0x883bdbfd) function
        pub fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<i64>, ::std::vec::Vec<::ethers::core::types::U256>),
        > {
            self.0
                .method_hash([136, 59, 219, 253], seconds_agos)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0x514ea4bf) function
        pub fn positions(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u128, ::ethers::core::types::U256, ::ethers::core::types::U256, u128, u128),
        > {
            self.0
                .method_hash([81, 78, 164, 191], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFees` (0x1ad8b03b) function
        pub fn protocol_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([26, 216, 176, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeeProtocol` (0x8206a4d1) function
        pub fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 6, 164, 209], (fee_protocol_0, fee_protocol_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slot0` (0x3850c7bd) function
        pub fn slot_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, i32, u16, u16, u16, u8, bool),
        > {
            self.0
                .method_hash([56, 80, 199, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `snapshotCumulativesInside` (0xa38807f2) function
        pub fn snapshot_cumulatives_inside(
            &self,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (i64, ::ethers::core::types::U256, u32),
        > {
            self.0
                .method_hash([163, 136, 7, 242], (tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x128acb08) function
        pub fn swap(
            &self,
            recipient: ::ethers::core::types::Address,
            zero_for_one: bool,
            amount_specified: ::ethers::core::types::I256,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::I256, ::ethers::core::types::I256),
        > {
            self.0
                .method_hash(
                    [18, 138, 203, 8],
                    (
                        recipient,
                        zero_for_one,
                        amount_specified,
                        sqrt_price_limit_x96,
                        data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tickBitmap` (0x5339c296) function
        pub fn tick_bitmap(
            &self,
            p0: i16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([83, 57, 194, 150], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tickSpacing` (0xd0c93a7c) function
        pub fn tick_spacing(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([208, 201, 58, 124], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ticks` (0xf30dba93) function
        pub fn ticks(
            &self,
            p0: i32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                i128,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                i64,
                ::ethers::core::types::U256,
                u32,
                bool,
            ),
        > {
            self.0
                .method_hash([243, 13, 186, 147], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token0` (0x0dfe1681) function
        pub fn token_0(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `token1` (0xd21220a7) function
        pub fn token_1(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Burn` event
        pub fn burn_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BurnFilter> {
            self.0.event()
        }
        ///Gets the contract's `Collect` event
        pub fn collect_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CollectFilter> {
            self.0.event()
        }
        ///Gets the contract's `CollectProtocol` event
        pub fn collect_protocol_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CollectProtocolFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Flash` event
        pub fn flash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FlashFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseObservationCardinalityNext` event
        pub fn increase_observation_cardinality_next_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IncreaseObservationCardinalityNextFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialize` event
        pub fn initialize_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Mint` event
        pub fn mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MintFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetFeeProtocol` event
        pub fn set_fee_protocol_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetFeeProtocolFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IUniswapV3PoolEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IUniswapV3Pool<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Burn", abi = "Burn(address,int24,int24,uint128,uint256,uint256)")]
    pub struct BurnFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Collect",
        abi = "Collect(address,address,int24,int24,uint128,uint128)"
    )]
    pub struct CollectFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CollectProtocol",
        abi = "CollectProtocol(address,address,uint128,uint128)"
    )]
    pub struct CollectProtocolFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: u128,
        pub amount_1: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Flash",
        abi = "Flash(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct FlashFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub paid_0: ::ethers::core::types::U256,
        pub paid_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "IncreaseObservationCardinalityNext",
        abi = "IncreaseObservationCardinalityNext(uint16,uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextFilter {
        pub observation_cardinality_next_old: u16,
        pub observation_cardinality_next_new: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialize", abi = "Initialize(uint160,int24)")]
    pub struct InitializeFilter {
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub tick: i32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Mint",
        abi = "Mint(address,address,int24,int24,uint128,uint256,uint256)"
    )]
    pub struct MintFilter {
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tick_lower: i32,
        #[ethevent(indexed)]
        pub tick_upper: i32,
        pub amount: u128,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetFeeProtocol", abi = "SetFeeProtocol(uint8,uint8,uint8,uint8)")]
    pub struct SetFeeProtocolFilter {
        pub fee_protocol_0_old: u8,
        pub fee_protocol_1_old: u8,
        pub fee_protocol_0_new: u8,
        pub fee_protocol_1_new: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,address,int256,int256,uint160,uint128,int24)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::I256,
        pub amount_1: ::ethers::core::types::I256,
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub liquidity: u128,
        pub tick: i32,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniswapV3PoolEvents {
        BurnFilter(BurnFilter),
        CollectFilter(CollectFilter),
        CollectProtocolFilter(CollectProtocolFilter),
        FlashFilter(FlashFilter),
        IncreaseObservationCardinalityNextFilter(
            IncreaseObservationCardinalityNextFilter,
        ),
        InitializeFilter(InitializeFilter),
        MintFilter(MintFilter),
        SetFeeProtocolFilter(SetFeeProtocolFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for IUniswapV3PoolEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BurnFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::BurnFilter(decoded));
            }
            if let Ok(decoded) = CollectFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::CollectFilter(decoded));
            }
            if let Ok(decoded) = CollectProtocolFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::CollectProtocolFilter(decoded));
            }
            if let Ok(decoded) = FlashFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::FlashFilter(decoded));
            }
            if let Ok(decoded)
                = IncreaseObservationCardinalityNextFilter::decode_log(log) {
                return Ok(
                    IUniswapV3PoolEvents::IncreaseObservationCardinalityNextFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::InitializeFilter(decoded));
            }
            if let Ok(decoded) = MintFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::MintFilter(decoded));
            }
            if let Ok(decoded) = SetFeeProtocolFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::SetFeeProtocolFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(IUniswapV3PoolEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IUniswapV3PoolEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BurnFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectProtocolFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FlashFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseObservationCardinalityNextFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeProtocolFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnFilter> for IUniswapV3PoolEvents {
        fn from(value: BurnFilter) -> Self {
            Self::BurnFilter(value)
        }
    }
    impl ::core::convert::From<CollectFilter> for IUniswapV3PoolEvents {
        fn from(value: CollectFilter) -> Self {
            Self::CollectFilter(value)
        }
    }
    impl ::core::convert::From<CollectProtocolFilter> for IUniswapV3PoolEvents {
        fn from(value: CollectProtocolFilter) -> Self {
            Self::CollectProtocolFilter(value)
        }
    }
    impl ::core::convert::From<FlashFilter> for IUniswapV3PoolEvents {
        fn from(value: FlashFilter) -> Self {
            Self::FlashFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseObservationCardinalityNextFilter>
    for IUniswapV3PoolEvents {
        fn from(value: IncreaseObservationCardinalityNextFilter) -> Self {
            Self::IncreaseObservationCardinalityNextFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for IUniswapV3PoolEvents {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<MintFilter> for IUniswapV3PoolEvents {
        fn from(value: MintFilter) -> Self {
            Self::MintFilter(value)
        }
    }
    impl ::core::convert::From<SetFeeProtocolFilter> for IUniswapV3PoolEvents {
        fn from(value: SetFeeProtocolFilter) -> Self {
            Self::SetFeeProtocolFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for IUniswapV3PoolEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(int24,int24,uint128)` and selector `0xa34123a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "burn", abi = "burn(int24,int24,uint128)")]
    pub struct BurnCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
    }
    ///Container type for all input parameters for the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `0x4f1eb3d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "collect", abi = "collect(address,int24,int24,uint128,uint128)")]
    pub struct CollectCall {
        pub recipient: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    ///Container type for all input parameters for the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "collectProtocol",
        abi = "collectProtocol(address,uint128,uint128)"
    )]
    pub struct CollectProtocolCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_0_requested: u128,
        pub amount_1_requested: u128,
    }
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `fee` function with signature `fee()` and selector `0xddca3f43`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    ///Container type for all input parameters for the `feeGrowthGlobal0X128` function with signature `feeGrowthGlobal0X128()` and selector `0xf3058399`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeGrowthGlobal0X128", abi = "feeGrowthGlobal0X128()")]
    pub struct FeeGrowthGlobal0X128Call;
    ///Container type for all input parameters for the `feeGrowthGlobal1X128` function with signature `feeGrowthGlobal1X128()` and selector `0x46141319`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feeGrowthGlobal1X128", abi = "feeGrowthGlobal1X128()")]
    pub struct FeeGrowthGlobal1X128Call;
    ///Container type for all input parameters for the `flash` function with signature `flash(address,uint256,uint256,bytes)` and selector `0x490e6cbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "flash", abi = "flash(address,uint256,uint256,bytes)")]
    pub struct FlashCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `increaseObservationCardinalityNext` function with signature `increaseObservationCardinalityNext(uint16)` and selector `0x32148f67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "increaseObservationCardinalityNext",
        abi = "increaseObservationCardinalityNext(uint16)"
    )]
    pub struct IncreaseObservationCardinalityNextCall {
        pub observation_cardinality_next: u16,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint160)` and selector `0xf637731d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint160)")]
    pub struct InitializeCall {
        pub sqrt_price_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `liquidity` function with signature `liquidity()` and selector `0x1a686502`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "liquidity", abi = "liquidity()")]
    pub struct LiquidityCall;
    ///Container type for all input parameters for the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxLiquidityPerTick", abi = "maxLiquidityPerTick()")]
    pub struct MaxLiquidityPerTickCall;
    ///Container type for all input parameters for the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `0x3c8a7d8d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mint", abi = "mint(address,int24,int24,uint128,bytes)")]
    pub struct MintCall {
        pub recipient: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub amount: u128,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `observations` function with signature `observations(uint256)` and selector `0x252c09d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "observations", abi = "observations(uint256)")]
    pub struct ObservationsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "observe", abi = "observe(uint32[])")]
    pub struct ObserveCall {
        pub seconds_agos: ::std::vec::Vec<u32>,
    }
    ///Container type for all input parameters for the `positions` function with signature `positions(bytes32)` and selector `0x514ea4bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "positions", abi = "positions(bytes32)")]
    pub struct PositionsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `protocolFees` function with signature `protocolFees()` and selector `0x1ad8b03b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees()")]
    pub struct ProtocolFeesCall;
    ///Container type for all input parameters for the `setFeeProtocol` function with signature `setFeeProtocol(uint8,uint8)` and selector `0x8206a4d1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setFeeProtocol", abi = "setFeeProtocol(uint8,uint8)")]
    pub struct SetFeeProtocolCall {
        pub fee_protocol_0: u8,
        pub fee_protocol_1: u8,
    }
    ///Container type for all input parameters for the `slot0` function with signature `slot0()` and selector `0x3850c7bd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "slot0", abi = "slot0()")]
    pub struct Slot0Call;
    ///Container type for all input parameters for the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "snapshotCumulativesInside",
        abi = "snapshotCumulativesInside(int24,int24)"
    )]
    pub struct SnapshotCumulativesInsideCall {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `0x128acb08`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swap", abi = "swap(address,bool,int256,uint160,bytes)")]
    pub struct SwapCall {
        pub recipient: ::ethers::core::types::Address,
        pub zero_for_one: bool,
        pub amount_specified: ::ethers::core::types::I256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `tickBitmap` function with signature `tickBitmap(int16)` and selector `0x5339c296`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tickBitmap", abi = "tickBitmap(int16)")]
    pub struct TickBitmapCall(pub i16);
    ///Container type for all input parameters for the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tickSpacing", abi = "tickSpacing()")]
    pub struct TickSpacingCall;
    ///Container type for all input parameters for the `ticks` function with signature `ticks(int24)` and selector `0xf30dba93`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ticks", abi = "ticks(int24)")]
    pub struct TicksCall(pub i32);
    ///Container type for all input parameters for the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    ///Container type for all input parameters for the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniswapV3PoolCalls {
        Burn(BurnCall),
        Collect(CollectCall),
        CollectProtocol(CollectProtocolCall),
        Factory(FactoryCall),
        Fee(FeeCall),
        FeeGrowthGlobal0X128(FeeGrowthGlobal0X128Call),
        FeeGrowthGlobal1X128(FeeGrowthGlobal1X128Call),
        Flash(FlashCall),
        IncreaseObservationCardinalityNext(IncreaseObservationCardinalityNextCall),
        Initialize(InitializeCall),
        Liquidity(LiquidityCall),
        MaxLiquidityPerTick(MaxLiquidityPerTickCall),
        Mint(MintCall),
        Observations(ObservationsCall),
        Observe(ObserveCall),
        Positions(PositionsCall),
        ProtocolFees(ProtocolFeesCall),
        SetFeeProtocol(SetFeeProtocolCall),
        Slot0(Slot0Call),
        SnapshotCumulativesInside(SnapshotCumulativesInsideCall),
        Swap(SwapCall),
        TickBitmap(TickBitmapCall),
        TickSpacing(TickSpacingCall),
        Ticks(TicksCall),
        Token0(Token0Call),
        Token1(Token1Call),
    }
    impl ::ethers::core::abi::AbiDecode for IUniswapV3PoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded)
                = <CollectCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Collect(decoded));
            }
            if let Ok(decoded)
                = <CollectProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CollectProtocol(decoded));
            }
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded)
                = <FeeGrowthGlobal0X128Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FeeGrowthGlobal0X128(decoded));
            }
            if let Ok(decoded)
                = <FeeGrowthGlobal1X128Call as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FeeGrowthGlobal1X128(decoded));
            }
            if let Ok(decoded)
                = <FlashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Flash(decoded));
            }
            if let Ok(decoded)
                = <IncreaseObservationCardinalityNextCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncreaseObservationCardinalityNext(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <LiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Liquidity(decoded));
            }
            if let Ok(decoded)
                = <MaxLiquidityPerTickCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxLiquidityPerTick(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <ObservationsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Observations(decoded));
            }
            if let Ok(decoded)
                = <ObserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Observe(decoded));
            }
            if let Ok(decoded)
                = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProtocolFees(decoded));
            }
            if let Ok(decoded)
                = <SetFeeProtocolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFeeProtocol(decoded));
            }
            if let Ok(decoded)
                = <Slot0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Slot0(decoded));
            }
            if let Ok(decoded)
                = <SnapshotCumulativesInsideCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SnapshotCumulativesInside(decoded));
            }
            if let Ok(decoded)
                = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded)
                = <TickBitmapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TickBitmap(decoded));
            }
            if let Ok(decoded)
                = <TickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TickSpacing(decoded));
            }
            if let Ok(decoded)
                = <TicksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ticks(decoded));
            }
            if let Ok(decoded)
                = <Token0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token0(decoded));
            }
            if let Ok(decoded)
                = <Token1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Token1(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUniswapV3PoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Collect(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollectProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeGrowthGlobal0X128(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeGrowthGlobal1X128(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Flash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseObservationCardinalityNext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxLiquidityPerTick(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Observations(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Observe(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeeProtocol(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slot0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SnapshotCumulativesInside(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TickBitmap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickSpacing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ticks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Token1(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IUniswapV3PoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Collect(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectProtocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeGrowthGlobal0X128(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeeGrowthGlobal1X128(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Flash(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseObservationCardinalityNext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxLiquidityPerTick(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Observations(element) => ::core::fmt::Display::fmt(element, f),
                Self::Observe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeeProtocol(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slot0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SnapshotCumulativesInside(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickBitmap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickSpacing(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ticks(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Token1(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BurnCall> for IUniswapV3PoolCalls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<CollectCall> for IUniswapV3PoolCalls {
        fn from(value: CollectCall) -> Self {
            Self::Collect(value)
        }
    }
    impl ::core::convert::From<CollectProtocolCall> for IUniswapV3PoolCalls {
        fn from(value: CollectProtocolCall) -> Self {
            Self::CollectProtocol(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for IUniswapV3PoolCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<FeeCall> for IUniswapV3PoolCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<FeeGrowthGlobal0X128Call> for IUniswapV3PoolCalls {
        fn from(value: FeeGrowthGlobal0X128Call) -> Self {
            Self::FeeGrowthGlobal0X128(value)
        }
    }
    impl ::core::convert::From<FeeGrowthGlobal1X128Call> for IUniswapV3PoolCalls {
        fn from(value: FeeGrowthGlobal1X128Call) -> Self {
            Self::FeeGrowthGlobal1X128(value)
        }
    }
    impl ::core::convert::From<FlashCall> for IUniswapV3PoolCalls {
        fn from(value: FlashCall) -> Self {
            Self::Flash(value)
        }
    }
    impl ::core::convert::From<IncreaseObservationCardinalityNextCall>
    for IUniswapV3PoolCalls {
        fn from(value: IncreaseObservationCardinalityNextCall) -> Self {
            Self::IncreaseObservationCardinalityNext(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IUniswapV3PoolCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LiquidityCall> for IUniswapV3PoolCalls {
        fn from(value: LiquidityCall) -> Self {
            Self::Liquidity(value)
        }
    }
    impl ::core::convert::From<MaxLiquidityPerTickCall> for IUniswapV3PoolCalls {
        fn from(value: MaxLiquidityPerTickCall) -> Self {
            Self::MaxLiquidityPerTick(value)
        }
    }
    impl ::core::convert::From<MintCall> for IUniswapV3PoolCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<ObservationsCall> for IUniswapV3PoolCalls {
        fn from(value: ObservationsCall) -> Self {
            Self::Observations(value)
        }
    }
    impl ::core::convert::From<ObserveCall> for IUniswapV3PoolCalls {
        fn from(value: ObserveCall) -> Self {
            Self::Observe(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for IUniswapV3PoolCalls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<ProtocolFeesCall> for IUniswapV3PoolCalls {
        fn from(value: ProtocolFeesCall) -> Self {
            Self::ProtocolFees(value)
        }
    }
    impl ::core::convert::From<SetFeeProtocolCall> for IUniswapV3PoolCalls {
        fn from(value: SetFeeProtocolCall) -> Self {
            Self::SetFeeProtocol(value)
        }
    }
    impl ::core::convert::From<Slot0Call> for IUniswapV3PoolCalls {
        fn from(value: Slot0Call) -> Self {
            Self::Slot0(value)
        }
    }
    impl ::core::convert::From<SnapshotCumulativesInsideCall> for IUniswapV3PoolCalls {
        fn from(value: SnapshotCumulativesInsideCall) -> Self {
            Self::SnapshotCumulativesInside(value)
        }
    }
    impl ::core::convert::From<SwapCall> for IUniswapV3PoolCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TickBitmapCall> for IUniswapV3PoolCalls {
        fn from(value: TickBitmapCall) -> Self {
            Self::TickBitmap(value)
        }
    }
    impl ::core::convert::From<TickSpacingCall> for IUniswapV3PoolCalls {
        fn from(value: TickSpacingCall) -> Self {
            Self::TickSpacing(value)
        }
    }
    impl ::core::convert::From<TicksCall> for IUniswapV3PoolCalls {
        fn from(value: TicksCall) -> Self {
            Self::Ticks(value)
        }
    }
    impl ::core::convert::From<Token0Call> for IUniswapV3PoolCalls {
        fn from(value: Token0Call) -> Self {
            Self::Token0(value)
        }
    }
    impl ::core::convert::From<Token1Call> for IUniswapV3PoolCalls {
        fn from(value: Token1Call) -> Self {
            Self::Token1(value)
        }
    }
    ///Container type for all return fields from the `burn` function with signature `burn(int24,int24,uint128)` and selector `0xa34123a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BurnReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `collect` function with signature `collect(address,int24,int24,uint128,uint128)` and selector `0x4f1eb3d8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CollectReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    ///Container type for all return fields from the `collectProtocol` function with signature `collectProtocol(address,uint128,uint128)` and selector `0x85b66729`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CollectProtocolReturn {
        pub amount_0: u128,
        pub amount_1: u128,
    }
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `fee` function with signature `fee()` and selector `0xddca3f43`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeReturn(pub u32);
    ///Container type for all return fields from the `feeGrowthGlobal0X128` function with signature `feeGrowthGlobal0X128()` and selector `0xf3058399`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeGrowthGlobal0X128Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feeGrowthGlobal1X128` function with signature `feeGrowthGlobal1X128()` and selector `0x46141319`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FeeGrowthGlobal1X128Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `liquidity` function with signature `liquidity()` and selector `0x1a686502`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidityReturn(pub u128);
    ///Container type for all return fields from the `maxLiquidityPerTick` function with signature `maxLiquidityPerTick()` and selector `0x70cf754a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MaxLiquidityPerTickReturn(pub u128);
    ///Container type for all return fields from the `mint` function with signature `mint(address,int24,int24,uint128,bytes)` and selector `0x3c8a7d8d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MintReturn {
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `observations` function with signature `observations(uint256)` and selector `0x252c09d7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ObservationsReturn {
        pub block_timestamp: u32,
        pub tick_cumulative: i64,
        pub seconds_per_liquidity_cumulative_x128: ::ethers::core::types::U256,
        pub initialized: bool,
    }
    ///Container type for all return fields from the `observe` function with signature `observe(uint32[])` and selector `0x883bdbfd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ObserveReturn {
        pub tick_cumulatives: ::std::vec::Vec<i64>,
        pub seconds_per_liquidity_cumulative_x12_8s: ::std::vec::Vec<
            ::ethers::core::types::U256,
        >,
    }
    ///Container type for all return fields from the `positions` function with signature `positions(bytes32)` and selector `0x514ea4bf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PositionsReturn {
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x128: ::ethers::core::types::U256,
        pub fee_growth_inside_1_last_x128: ::ethers::core::types::U256,
        pub tokens_owed_0: u128,
        pub tokens_owed_1: u128,
    }
    ///Container type for all return fields from the `protocolFees` function with signature `protocolFees()` and selector `0x1ad8b03b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProtocolFeesReturn {
        pub token_0: u128,
        pub token_1: u128,
    }
    ///Container type for all return fields from the `slot0` function with signature `slot0()` and selector `0x3850c7bd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Slot0Return {
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub tick: i32,
        pub observation_index: u16,
        pub observation_cardinality: u16,
        pub observation_cardinality_next: u16,
        pub fee_protocol: u8,
        pub unlocked: bool,
    }
    ///Container type for all return fields from the `snapshotCumulativesInside` function with signature `snapshotCumulativesInside(int24,int24)` and selector `0xa38807f2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SnapshotCumulativesInsideReturn {
        pub tick_cumulative_inside: i64,
        pub seconds_per_liquidity_inside_x128: ::ethers::core::types::U256,
        pub seconds_inside: u32,
    }
    ///Container type for all return fields from the `swap` function with signature `swap(address,bool,int256,uint160,bytes)` and selector `0x128acb08`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SwapReturn {
        pub amount_0: ::ethers::core::types::I256,
        pub amount_1: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `tickBitmap` function with signature `tickBitmap(int16)` and selector `0x5339c296`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TickBitmapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tickSpacing` function with signature `tickSpacing()` and selector `0xd0c93a7c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TickSpacingReturn(pub i32);
    ///Container type for all return fields from the `ticks` function with signature `ticks(int24)` and selector `0xf30dba93`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TicksReturn {
        pub liquidity_gross: u128,
        pub liquidity_net: i128,
        pub fee_growth_outside_0x128: ::ethers::core::types::U256,
        pub fee_growth_outside_1x128: ::ethers::core::types::U256,
        pub tick_cumulative_outside: i64,
        pub seconds_per_liquidity_outside_x128: ::ethers::core::types::U256,
        pub seconds_outside: u32,
        pub initialized: bool,
    }
    ///Container type for all return fields from the `token0` function with signature `token0()` and selector `0x0dfe1681`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Token0Return(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `token1` function with signature `token1()` and selector `0xd21220a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Token1Return(pub ::ethers::core::types::Address);
}
