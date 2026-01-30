use axum::Json;
use axum::http::{ StatusCode, Error as AxumError };
use serde_json::json;

#[derive(Debug)]
pub enum CalcError
{
    InternalServer(String),
}

impl axum::response::IntoResponse for CalcError
{
    fn into_response(self) -> axum::response::Response
    {
        let(status, error_message) = match self
        {
            Self::InternalServer(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!(
        {
            "error": error_message,
        }
        ));

        (status, body).into_response()
    }
}

impl From<AxumError> for CalcError 
{
    fn from(err: AxumError) -> Self 
    {
        Self::InternalServer(err.to_string())
    }
}

impl From<std::io::Error> for CalcError 
{
    fn from(err: std::io::Error) -> Self 
    {
        Self::InternalServer(err.to_string())
    }
}
