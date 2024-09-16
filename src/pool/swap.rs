use crate::{types::amount::IntoTokenAmount, Token, TokenAmount, V3Pool};

#[async_trait::async_trait]
trait SwapExt: V3Pool {
    async fn amount_in<'a, T>(&'a self, amount_out: T, token_idx: Token) -> Result<TokenAmount<'a, Self>, Self::BackendError>
    where
        T: IntoTokenAmount<'a, Self> + Send
    {
        todo!()
    }

    async fn amount_out<'a, T>(&'a self, amount_in: T, token_idx: Token) -> Result<TokenAmount<'a, Self>, Self::BackendError>
    where
        T: IntoTokenAmount<'a, Self> + Send 
    {
        todo!()
    }
}
