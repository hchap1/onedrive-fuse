use crate::error::Res;
use crate::authentication::callback::CALLBACK_PORT;
use url::Url;

// oauth2
const URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";

// Public-facing Client ID
pub const CLIENT_ID: &str = "893e8a4b-f159-46f0-abb8-1a1bb74108ac";
const RESPONSE_TYPE: &str = "code";

// Redirect-URI is localhost, port specified in callback
pub const REDIRECT_URI: &str = "http://localhost";

// Tell microsoft to POST the auth token to the redirect_uri
const RESPONSE_MODE: &str = "form_post";

// Access to basic user information and complete filesystem access
pub const SCOPE: &str = "User.Read offline_access Files.ReadWrite";

// Inform microsoft that the PKCE challenge is SHA-256 hashed with BASE64 encoding.
const CODE_CHALLENGE_METHOD: &str = "S256";

/// Non-blocking call opening oauth2 permission page in default browser
pub fn launch_oauth2(csrf: &String, pkce: &String) -> Res<()> {
    let redirect_uri = format!("{REDIRECT_URI}:{}", CALLBACK_PORT);

    let mut auth_url = Url::parse(URL)?;
    auth_url
        .query_pairs_mut()
        .append_pair("client_id", CLIENT_ID)
        .append_pair("response_type", RESPONSE_TYPE)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("response_mode", RESPONSE_MODE)
        .append_pair("scope", SCOPE)
        .append_pair("state", &csrf)
        .append_pair("code_challenge", &pkce)
        .append_pair("code_challenge_method", CODE_CHALLENGE_METHOD);

    open::that(auth_url.as_str())?;
    Ok(())
}
