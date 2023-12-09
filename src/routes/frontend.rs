use axum::{
    extract::State,
    response::{Response, IntoResponse as AxumIntoResponse}
};
use serde::Serialize;
use askama_axum::IntoResponse;
use askama::Template;
use crate::routes::links;

pub async fn styles() -> impl AxumIntoResponse {
    Response::builder()
        .header("Content-Type", "text/css")
        .body(include_str!("../../templates/styles.css").to_owned())
        .unwrap()
} 

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn homepage() -> impl IntoResponse {
    IndexTemplate
} 

#[derive(Template)]
#[template(path = "shortlink.html")]
struct ShortlinkTemplate;

#[derive(Template)]
#[template(path = "options.html")]
struct OptionsTemplate;

pub async fn options() -> impl IntoResponse {
    OptionsTemplate
} 

pub async fn shortlink() -> impl IntoResponse {
    ShortlinkTemplate
} 

#[derive(Serialize, sqlx::FromRow)]
struct Link {
    uri: String,
    shortlink_id: String,
}

#[derive(Template)]
#[template(path = "links.html")]
struct LinksTemplate {
    links: Vec<Link>
}

pub async fn all_links(
    State(state): State<links::AppState>
    ) -> impl IntoResponse {

    let links = sqlx::query_as::<_, Link>("
        SELECT uri, shortlink_id FROM links
        ")
        .fetch_all(&state.db)
        .await
        .unwrap();

    LinksTemplate { links }
}
