use axum::{
    routing::{get, post},
    Router
};
use sqlx::PgPool;

use crate::routes::{frontend, links};

pub fn init_router(db: PgPool, domain: String) -> Router {
    let state = links::AppState { db, domain };

    Router::new()
        .route("/", get(frontend::homepage))
        .route("/styles.css", get(frontend::styles))
        .route("/links", get(frontend::all_links))
        .route("/options", get(frontend::options))
        .route("/shorten", post(links::shorten))
        .route("/shortlink", get(frontend::shortlink))
        .route("/logo", post(links::upload_logo))
        .route("/:id", get(links::redirect).delete(links::delete_link))
        .route("/qr/:id", get(links::get_qrcode))
        .with_state(state)
}
