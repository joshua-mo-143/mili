use axum::{
    extract::{Multipart, Path, State, Form},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use askama::Template;
use askama_axum::IntoResponse as AskamaResponse;
use image::{imageops, io::Reader, ImageFormat};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::io::Cursor;

use crate::qrcode::make_qrcode;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Deserialize)]
pub struct LinkSubmission {
    uri: url::Url,
    shortlink: Option<String>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Link {
    uri: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ByteData {
    bytedata: Option<Vec<u8>>,
}

#[derive(Template)]
#[template(path = "shorten_success.html")]
struct ShortenSuccessTemplate {
    shortlink_id: String
} 

pub async fn shorten(
    State(state): State<AppState>,
    Form(query): Form<LinkSubmission>,
) -> impl AskamaResponse {
    let shortlink_id = match query.shortlink {
        Some(res) =>  {
            if res != String::new() { res } else { nanoid!(6) }
            },
        None => nanoid!(6),
    };

    sqlx::query(
        "
        INSERT INTO LINKS
        (uri, shortlink_id)
        VALUES
        ($1, $2)
    ",
    )
    .bind::<String>(query.uri.into())
    .bind(shortlink_id.to_owned())
    .execute(&state.db)
    .await
    .unwrap();

    ShortenSuccessTemplate {
        shortlink_id
    }
}

pub async fn delete_link(
    State(state): State<AppState>,
    Path(shortlink_id): Path<String>,
) -> impl IntoResponse {
    sqlx::query(
        "DELETE FROM links WHERE shortlink_id = $1",
    )
    .bind(shortlink_id)
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::OK
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(shortlink_id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Link>(
        "
        SELECT uri FROM LINKS
        WHERE SHORTLINK_ID = $1
        LIMIT 1
    ",
    )
    .bind(shortlink_id)
    .fetch_one(&state.db)
    .await {
    Ok(res) => Ok(Redirect::to(&res.uri)),
    Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

pub async fn get_qrcode(
    State(state): State<AppState>,
    Path(shortlink_id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = sqlx::query_as::<_, Link>(
        "
        SELECT uri FROM LINKS
        WHERE SHORTLINK_ID = $1
        LIMIT 1
    ",
    )
    .bind(shortlink_id)
    .fetch_one(&state.db)
    .await
    .unwrap();
    
    let query = match sqlx::query_as::<_, ByteData>("SELECT bytedata FROM images
    WHERE is_default is true LIMIT 1")
        .fetch_one(&state.db)
        .await {
            Ok(res) => res,
            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        };

    let res = match query.bytedata {
        Some(logo) => {
            make_qrcode(&result.uri, Some(logo))
        }
        None => make_qrcode(&result.uri, None),
    };

    Ok(Response::builder()
        .header("Content-Type", "image/png")
        .header("Content-Disposition", r#"attachment; filename="image.png""#)
        .status(200)
        .body(axum::body::Full::from(res))
        .unwrap())
}

pub async fn upload_logo(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let _name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let img2 = Reader::new(Cursor::new(data))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .resize(300, 300, imageops::FilterType::Lanczos3);

        let mut bytes: Vec<u8> = Vec::new();
        img2.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();

        sqlx::query("UPDATE IMAGES SET is_default = null where is_default = true").execute(&state.db).await.unwrap();

        sqlx::query("INSERT INTO images (
        alias,
        bytedata,
        is_default
        ) 
        VALUES
        (
        $1, $2, $3
        )"
        )
            .bind("meme".to_string())
            .bind(bytes)
            .bind(Some(true))
            .execute(&state.db)
            .await
            .unwrap();

    }
    (StatusCode::OK, "Logo uploaded!".to_string())
}
