use crate::{types::amount::IntoTokenAmount, TokenAmount, V3Pool};

#[async_trait::async_trait]
pub trait SwapExt: V3Pool {
    async fn amount_in<'a, T>(&'a self, amount_out: T) -> Result<TokenAmount<'a, Self>, Self::BackendError>
    where
        T: IntoTokenAmount<'a, Self> + Send
    {
        todo!()
    }

    async fn amount_out<'a, T>(&'a self, amount_in: T) -> Result<TokenAmount<'a, Self>, Self::BackendError>
    where
        T: IntoTokenAmount<'a, Self> + Send 
    {
        todo!()
    }
}
