use axum::{async_trait, http::StatusCode, response::IntoResponse, RequestPartsExt, extract::{rejection::FormRejection, FromRequest, MatchedPath, Request}};

// We define our own `Json` extractor that customizes the error from `axum::Json`
pub struct Form<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Form<T>
where
    axum::Form<T>: FromRequest<S, Rejection = FormRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match axum::Form::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                Err((rejection.status(), "Deserialization failed. Are you sure this is a URL?".to_string()))
            }
        }
    }
}
