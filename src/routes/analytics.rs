use axum::{
    extract::State,
    response::{Response, IntoResponse as AxumIntoResponse}
};
use serde::Serialize;
use askama_axum::IntoResponse;
use askama::Template;
use crate::routes::links;
use chrono::NaiveDate;

use crate::routes::links::AppState;

#[derive(sqlx::FromRow)]
pub struct Counter {
    date: NaiveDate,
    count: i64
}

#[derive(Template)]
#[template(path = "analytics.html")]
struct AnalyticsTemplate {
    data: Vec<Counter>
}

pub async fn get_analytics(
    State(state): State<AppState>,
    ) -> impl IntoResponse {
    let data = sqlx::query_as::<_, Counter>("SELECT DATE(visited_at), COUNT(*) FROM STATS GROUP BY DATE ORDER BY DATE DESC LIMIT 7")
        .fetch_all(&state.db)
        .await
        .unwrap();

    AnalyticsTemplate { data }
} 
