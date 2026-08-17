use crate::{authentication::{launch::launch_oauth2, security::{generate_csrf, generate_pkce}}, error::Res};

pub struct TemporaryToken {
    token: String
}

pub async fn get_temporary_token() -> Res<String> {
    let (verifier, challenge) = generate_pkce();
    let state = generate_csrf();

    launch_oauth2(challenge, state);

    Ok(String::new())
}
