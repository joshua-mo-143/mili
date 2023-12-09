use mili::routes::router::init_router;
use tokio::net::TcpListener;
use shuttle_metadata::{Metadata, Environment};
use sqlx::PgPool;

struct CustomService {
    db: PgPool,
    metadata: Metadata,
} 

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_metadata::ShuttleMetadata] metadata: Metadata,
) -> Result<CustomService, shuttle_runtime::Error> {
    Ok(CustomService { db, metadata })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self,
                  addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
    sqlx::migrate!().run(&self.db).await.unwrap();

        let domain = match self.metadata.env {
            Environment::Local => "http://localhost:8000".to_string(),
            Environment::Deployment => format!("https://{}.shuttleapp.rs",self.metadata.project_name)
        };

    let router = init_router(self.db, domain);

    let tcplistener = TcpListener::bind(addr).await.unwrap();

    axum::serve(tcplistener, router).await.unwrap();

    Ok(())
    }
}
