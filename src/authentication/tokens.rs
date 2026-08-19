use reqwest::Client;

use serde::Serialize;
use serde::Deserialize;

use std::time::UNIX_EPOCH;
use std::time::SystemTime;

use crate::authentication::launch::CLIENT_ID;
use crate::authentication::launch::REDIRECT_URI;
use crate::authentication::launch::SCOPE;
use crate::error::Res;

const TEMPORARY_TOKEN_POST_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";
const GRANT_TYPE: &str = "authorization_code";

#[derive(Clone, Debug)]
pub struct TemporaryToken {
    pub code: String,
    pub state: String
}

#[derive(Clone, Debug)]
pub struct TemporaryTokenAndVerifier {
    pub code: String,
    pub verifier: String
}

#[derive(Clone, Debug)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: String,
    pub absolute_expiration: usize
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub access_token: String,
    pub expires_in: usize,
    pub refresh_token: String,
    scope: String,
    token_type: String
}

pub async fn post_temporary_token(temporary: TemporaryTokenAndVerifier) -> Res<TokenSet> {
    let params = [
        ("client_id", CLIENT_ID),
        ("grant_type", GRANT_TYPE),

        // the code provided on POST callback to localhost
        ("code", &temporary.code),

        // simply to verify its the same request, not used again
        ("redirect_uri", REDIRECT_URI),

        // The verifier is the raw PKCE string that was hashed when the user was redirected to the OAUTH2 page in the browser.
        // This code allows the request to be validated without the client secret.
        ("code_verifier", &temporary.verifier),
        ("scope", SCOPE)
    ];

    let client = Client::new();
    let res = client
        .post(TEMPORARY_TOKEN_POST_URL)
        .form(&params)
        .send()
        .await?;

    // if the POST request failed, avoid serde as it will panic
    if !res.status().is_success() {
        return Err(crate::error::Error::TemporaryTokenPostFailed);
    }

    // attempt to retrieve body of the response and parse with serde
    let text = res.text().await?;
    let response: Response = serde_json::from_str(&text)?;


    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        as usize
        + response.expires_in;

    // retrieve important part of the tokenset
    Ok(TokenSet {
        access_token: response.access_token,
        refresh_token: response.refresh_token,
        absolute_expiration: expiration
    })
}
