use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponseWith<TData> {
    message: String,
    data: TData,
}

pub struct ApiResponseError<TError> {
    error_code: ErrorCode,
    error: TError,
}

#[derive(Debug)]
pub enum ErrorCode {
    UnAuthorized = 401,
    ForBidden = 403,
    NotFound = 404,
    ValidationError = 1000,
    ConnectionError = 1001,
}

impl<TError> ApiResponseError<TError> {
    fn get_status_code(self) -> StatusCode {
        match self.error_code {
            ErrorCode::UnAuthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::ForBidden => StatusCode::FORBIDDEN,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::presentation_models::api_response::*;

    #[test]
    fn test_case_one() {
        let response_message = ApiResponseError::<String> {
            error_code: ErrorCode::UnAuthorized,
            error: "User is unauthorized".to_string(),
        };

        let response_status_code = response_message.get_status_code();
        assert_eq!(StatusCode::UNAUTHORIZED, response_status_code);
    }
}