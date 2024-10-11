use alloy::contract::{CallBuilder, SolCallBuilder};
use alloy::network::{Network, TransactionBuilder};
use alloy::providers::Provider;
use alloy::sol;
use alloy::sol_types::SolCall;
use alloy::transports::Transport;

pub trait Batch: Sized {
    /// Batch a collection of calls using one of the available batching strategies.
    fn batch(self) -> BatchCall<Self>;
}

impl<T, P, N, SC, I> Batch for I
where
    P: Provider<T, N>,
    I: IntoIterator<Item = SolCallBuilder<T, P, SC, N>>,
    SC: SolCall,
    T: Transport + Clone,
    N: Network,
{
    fn batch(self) -> BatchCall<I> {
        BatchCall { calls: self }
    }
}

pub struct BatchCall<I> {
    calls: I,
}

impl<T, P, N, SC, I> BatchCall<I>
where
    P: Provider<T, N>,
    I: IntoIterator<Item = SolCallBuilder<T, P, SC, N>>,
    SC: SolCall,
    T: Transport + Clone,
    N: Network,
{
    /// Note: https://github.com/rust-lang/rust/issues/110338
    /// 
    /// You may need to collect your iterator before calling this function
    pub async fn call(self) -> Result<Vec<SC::Return>, alloy::contract::Error> {
        let mut iter = self.calls.into_iter();

        let Some(call) = iter.next() else {
            return Ok(vec![]);
        };

        // todo: Clone because upstream stops us from taking both the provideer and the tx_req
        let tx_req = call.as_ref().clone();

        let CallBuilder {
            provider,
            ..
        } = call;

        #[cfg(not(any(feature = "trace_callMany", feature = "eth_callMany")))]
        {
            use Multicall::{Call, MulticallInstance};

            #[cfg(debug_assertions)]
            {
                let chain_id = provider.get_chain_id().await?;

                if !crate::constants::NETWORKS
                    .get(&chain_id)
                    .map(|c| c.supports_multicall)
                    .unwrap_or(false)
                {
                    tracing::error!("Chain does not support multicall, this call will fail");
                }
            }

            let calls = std::iter::once(tx_req)
                .chain(iter.map(|c| c.into_transaction_request()))
                .map(|call| Call {
                    target: call.to().unwrap_or_default(),
                    callData: call.input().cloned().unwrap_or_default(),
                })
                .collect::<Vec<_>>();

            let multicall = MulticallInstance::new(crate::constants::MULTICALL3, provider);

            let data = multicall.aggregate(calls).call().await?.returnData;

            return data
                .into_iter()
                .map(|d| SC::abi_decode_returns(&d, true).map_err(alloy::contract::Error::from))
                .collect();
        }

        #[cfg(feature = "trace_callMany")]
        {
            use alloy::providers::ext::TraceApi;
            use alloy::rpc::types::trace::parity::TraceType;

            let tt = vec![TraceType::Trace];
            return provider
                .trace_call_many(
                    std::iter::once(tx_req)
                        .chain(iter.map(|call| call.into_transaction_request()))
                        .map(|call| (call, tt.as_slice()))
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .await
                .map_err(alloy::contract::Error::from)?
                .into_iter()
                .filter_map(|r| match SC::abi_decode_returns(&r.output, true).ok() {
                    Some(o) => Some(Ok(o)),
                    None => {
                        tracing::error!("Failed to decode return value: {:?}", r.output);
                        None
                    }
                })
                .collect();
        }

        // todo: bundle is concrete over Eth::TxRequest
        // #[cfg(feature = "eth_callMany")]
        // {
        //     use alloy::rpc::types::Bundle;
        //     let bundle = Bundle {
        //         transactions: std::iter::once(call)
        //             .chain(iter.map(|call| call.into_transaction_request()))
        //             .map(|call| call.into())
        //             .collect::<Vec<_>>(),
        //         block_override: None,
        //     };

        //     return provider
        //         .raw_request(
        //             "eth_callMany".into(),
        //             bundle
        //         )
        //         .await
        //         .map_err(alloy::contract::Error::from)?
        //         .into_iter()
        //         .map(|r| {
        //             SC::abi_decode_returns(&r.output, true).map_err(alloy::contract::Error::from)
        //         })
        //         .collect();
        // }
    }

    pub fn map<F, R>(self, f: F) -> MapCall<I, F>
    where
        F: FnMut(SC::Return) -> R,
    {
        MapCall { batch: self, f }
    }
}

pub struct MapCall<I, F> {
    batch: BatchCall<I>,
    f: F,
}

impl<T, P, N, SC, I, F, R> MapCall<I, F>
where
    P: Provider<T, N>,
    SC: SolCall,
    T: Transport + Clone,
    N: Network,
    I: IntoIterator<Item = SolCallBuilder<T, P, SC, N>>,
    F: FnMut(SC::Return) -> R,
{
    pub async fn call(mut self) -> Result<Vec<R>, alloy::contract::Error> {
        Ok(self
            .batch
            .call()
            .await?
            .into_iter()
            .map(|r| (self.f)(r))
            .collect())
    }
}

sol! {
    #[sol(rpc)]
    interface Multicall {
        struct Call {
            address target;
            bytes callData;
        }

        function aggregate(Call[] memory calls) external returns (uint256 blockNumber, bytes[] returnData);
    }
}
