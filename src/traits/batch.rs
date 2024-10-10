use alloy::contract::SolCallBuilder;
use alloy::network::Network;
use alloy::providers::Provider;
use alloy::sol_types::SolCall;
use alloy::transports::Transport;

pub trait Batch: Sized {
    fn batch(self) -> BatchCall<Self>;
}

impl<'a, T, P, N, SC, I> Batch for I
where
    P: Provider<T, N> + 'a,
    I: IntoIterator<Item = SolCallBuilder<T, &'a P, SC, N>>,
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

impl<'a, T, P, N, SC, I> BatchCall<I>
where
    P: Provider<T, N> + 'a,
    I: IntoIterator<Item = SolCallBuilder<T, &'a P, SC, N>>,
    SC: SolCall,
    T: Transport + Clone,
    N: Network
{
    pub async fn call(self) -> Result<Vec<SC::Return>, alloy::contract::Error> {
        let mut iter = self.calls.into_iter();

        let Some(call) = iter.next() else {
            return Ok(vec![]);
        };

        let (provider, call) = (call.provider, call.into_transaction_request());

        #[cfg(feature = "trace_callMany")]
        {
            use alloy::providers::ext::TraceApi;
            use alloy::rpc::types::trace::parity::TraceType;

            let tt = vec![TraceType::Trace];
            return provider
                .trace_call_many(
                    std::iter::once(call)
                        .chain(iter.map(|call| call.into_transaction_request()))
                        .map(|call| (call, tt.as_slice()))
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .await
                .map_err(alloy::contract::Error::from)?
                .into_iter()
                .map(|r| SC::abi_decode_returns(&r.output, true).map_err(alloy::contract::Error::from))
                .collect();
        }

        // #[cfg(not(feature = "trace_callMany"))]
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
}
