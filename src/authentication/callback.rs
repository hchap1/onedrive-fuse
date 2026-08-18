use std::net::SocketAddr;

use async_channel::Receiver;
use async_channel::Sender;
use axum::Form;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::post;
use axum::routing::get;
use axum::Router;
use axum::extract::Query;
use axum::response::Html;
use axum_server::Handle;

use serde::Deserialize;

use crate::authentication::tokens;
use crate::error::Res;

pub const CALLBACK_PORT: u16 = 3283;

#[derive(Debug, Deserialize)]
struct AuthCallback {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>
}

#[derive(Clone)]
struct CallbackAgent {
    sender: Sender<Res<tokens::TemporaryToken>>
}

#[derive(Debug, Deserialize)]
struct ErrorQuery {
    error: Option<String>,
}

// echo the authentication result out via State
impl CallbackAgent {
    async fn handle_post(
        State(callback_agent): State<CallbackAgent>,
        Form(payload): Form<AuthCallback>
    ) -> impl IntoResponse {
        let res = if let Some(error) = payload.error {
            Err(crate::error::Error::PostCallbackError(format!("{error} {:?}", payload.error_description)))
        } else {
            let code_exists = payload.code.is_some();
            let state_exists = payload.state.is_some();

            if code_exists && state_exists {
                Ok(
                    tokens::TemporaryToken {
                        code: payload.code.unwrap(),
                        state: payload.state.unwrap(),
                    }
                )
            } else if code_exists {
                Err(crate::error::Error::PostCallbackMissingState)
            } else {
                Err(crate::error::Error::PostCallbackMissingCode)
            }
        };

        let error_snapshot: Option<String> = res.as_ref().err().map(|e| format!("{e:?}"));

        if let Err(send_err) = callback_agent.sender.send(res).await {
            eprintln!("failed to forward callback result: {send_err}");
        }

        match error_snapshot {
            Some(err_msg) => {
                let auth_url = format!("/error?error={}", urlencoding::encode(&err_msg));
                Redirect::to(&auth_url).into_response()
            }
            None => Redirect::to("/success").into_response(),
        }
    }
}

pub async fn callback() -> Res<tokens::TemporaryToken> {

    let (sender, receiver) = async_channel::unbounded();
    let callback_agent = CallbackAgent {
        sender
    };

    let app = Router::new()
        .route("/", post(CallbackAgent::handle_post))
        .route("/success", get(success_page))
        .route("/error", get(error_page))
        .with_state(callback_agent);

    // axum-server handle to shutdown once a code is parsed
    let handle = Handle::new();
    let callback_handle = tokio::spawn(await_callback(receiver, handle.clone()));

    // bind to the specified port on all available interfaces
    let addr = SocketAddr::from(([0, 0, 0, 0], CALLBACK_PORT));
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await?;

    callback_handle.await?
}

async fn await_callback<A: axum_server::Address>(
    receiver: Receiver<Res<tokens::TemporaryToken>>, handle: Handle<A>
) -> Res<tokens::TemporaryToken> {

    // await the receiver to do something
    let res = receiver.recv().await;
    
    // regardless of result, always shut down webserver, 2 sec deadline for remaining conns
    handle.graceful_shutdown(Some(std::time::Duration::from_secs(2)));
    res?
}

async fn success_page() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>Success</title></head>
        <body>
            <h1>You're signed in!</h1>
            <p>Authentication completed successfully. You can close this window.</p>
        </body>
        </html>
        "#,
    )
}

async fn error_page(Query(params): Query<ErrorQuery>) -> impl IntoResponse {
    let message = params.error.unwrap_or_else(|| "unknown".to_string());
    let escaped = escape_html(&message);

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head><title>Sign-in Error</title></head>
        <body>
            <h1>Something went wrong</h1>
            <p class="error">{escaped}</p>
            <a href="/login">Try again</a>
        </body>
        </html>
        "#
    ))
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
