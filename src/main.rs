#[tokio::main]
async fn main() {
    let wallet_app = axum::Router::new()
        .route(
            "/foo/bar",
            axum::routing::get(|| async { "hello from bar" }),
        )
        .route(
            "/foo/baz",
            axum::routing::get(|| async { "hello from baz" }),
        )
        .with_state(());
    let app = axum::Router::new().route_service("/foo", wallet_app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
