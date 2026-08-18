use std::net::IpAddr;
use std::net::Ipv4Addr;
use axum::routing::post;
use axum::http::StatusCode;

use axum::Json;
use axum::Router;

use crate::error::Res;

pub const CALLBACK_PORT: u16 = 3283;

pub async fn callback() -> Res<()> {
    let app = Router::new()
        .route("/", post(handle_post));

    let listener = tokio::net::TcpListener::bind((IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), CALLBACK_PORT))
        .await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub async fn handle_post(thing: String) -> (StatusCode, Json<()>) {
    println!("Returned: {thing}");
    (StatusCode::ACCEPTED, Json(()))
}
