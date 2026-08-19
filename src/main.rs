pub mod authentication;
pub mod error;

use crate::authentication::authenticate;
use crate::error::Res;

#[tokio::main]
async fn main() -> Res<()> {
    let temporary = authenticate::get_temporary_token().await?;
    let tokenset = authenticate::exchange_temporary(temporary).await?;
    
    Ok(())
}
