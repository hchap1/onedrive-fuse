use crate::{authentication::{callback, launch::launch_oauth2, security::{generate_csrf, generate_pkce}, tokens::{self, TemporaryTokenAndVerifier, TokenSet}}, error::Res};

pub async fn get_temporary_token() -> Res<TemporaryTokenAndVerifier> {
    let (verifier, challenge) = generate_pkce();
    let state = generate_csrf();

    launch_oauth2(&state, &challenge)?;
    let temporary_token = callback::callback().await?;

    // assert that the state matches to prevent csrf attack
    if temporary_token.state != state {
        return Err(crate::error::Error::StateMismatchViolation)
    }

    Ok(TemporaryTokenAndVerifier {
        code: temporary_token.code,
        verifier,
    })
}

/// post the temporary token to get a longer lived tokenset
pub async fn exchange_temporary(temporary: TemporaryTokenAndVerifier) -> Res<TokenSet> {
    tokens::post_temporary_token(temporary).await
}
