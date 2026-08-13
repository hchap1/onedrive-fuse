// oauth2
const URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?";

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

pub async fn launch_oauth2(csrf: String, pkce: String, state: String)
