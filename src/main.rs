pub mod authentication;
pub mod error;

use crate::error::Res;

use crate::authentication::launch::launch_oauth2;
use crate::authentication::callback::callback;
use crate::authentication::security::generate_csrf;
use crate::authentication::security::generate_pkce;

#[tokio::main]
async fn main() -> Res<()> {
   let (verifier, challenge) = generate_pkce(); 
   let state = generate_csrf();

   launch_oauth2(state.clone(), challenge)?;
   let res = callback().await?;
   println!("{res:?}");
   println!("{state}");
   Ok(())
}
