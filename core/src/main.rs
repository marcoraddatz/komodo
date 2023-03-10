// #![allow(unused)]

use ::helpers::get_socket_addr;
use auth::JwtClient;
use axum::{http::StatusCode, Router};
use state::State;
use tower_http::cors::{Any, CorsLayer};

mod actions;
mod api;
mod auth;
mod cloud;
mod config;
mod helpers;
mod monitoring;
mod state;
mod ws;

type ResponseResult<T> = Result<T, (StatusCode, String)>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (config, spa_router) = config::load();

    println!("starting monitor core on port {}...", config.port);

    let app = Router::new()
        .merge(spa_router)
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .nest("/ws", ws::router())
        .layer(JwtClient::extension(&config))
        .layer(State::extension(config.clone()).await)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    println!("started monitor core on port {}", config.port);

    axum::Server::bind(&get_socket_addr(config.port))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
