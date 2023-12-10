use mili::routes::router::init_router;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
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

    let domain = std::env::var("RAILWAY_PUBLIC_DOMAIN").unwrap();
    let domain = format!("https://{domain}");
    let port = std::env::var("PORT").unwrap().parse::<u16>().unwrap();

    let router = init_router(db, domain);

    let addr = SocketAddr::from(([0,0,0,0], port));

    let tcplistener = TcpListener::bind(addr).await.unwrap();
    axum::serve(tcplistener, router).await.unwrap();
}
