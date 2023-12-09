use mili::routes::router::init_router;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let conn_string = std::env::var("DATABASE_URL").unwrap();

    let db = PgPoolOptions::new()
        .min_connections(5)
        .connect(&conn_string)
        .await
        .unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    let router = init_router(db);

    let addr = SocketAddr::from(([0,0,0,0],8000));

    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}
