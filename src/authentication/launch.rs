use crate::error::Res;

// oauth2
const URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize";

// Public-facing Client ID
const CLIENT_ID: &str = "893e8a4b-f159-46f0-abb8-1a1bb74108ac";
const RESPONSE_TYPE: &str = "code";

// Redirect-URI is localhost, port specified in callback
const REDIRECT_URI: &str = "http://localhost";

// Tell microsoft to POST the auth token to the redirect_uri
const RESPONSE_MODE: &str = "form_post";

// Access to basic user information and complete filesystem access
const SCOPE: &str = "User.Read offline_access Files.ReadWrite";

// Inform microsoft that the PKCE challenge is SHA-256 hashed with BASE64 encoding.
const CODE_CHALLENGE_METHOD: &str = "S256";

/// Non-blocking call opening oauth2 permission page in default browser
pub fn launch_oauth2(csrf: String, pkce: String) -> Res<()> {
    open::that(
        format!(
            "{URL}?\
            client_id={CLIENT_ID}&\
            response_type={RESPONSE_TYPE}&\
            redirect_uri={REDIRECT_URI}&\
            response_mode={RESPONSE_MODE}&\
            scope={SCOPE}&\
            state={csrf}&\
            code_challenge={pkce}&\
            code_challenege_method={CODE_CHALLENGE_METHOD}"
        )
    )?;
    Ok(())
}
