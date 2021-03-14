// Api Error type encapsulating HTTP status and the JSON error response
pub use chesscom_openapi::apis::default_api::ApiError;

// Error type that wraps any error the client can throw, e.g. api errors, http errors, etc...
pub type Error = chesscom_openapi::apis::Error<ApiError>;

// Represents the JSON error response from the API
pub type ErrorResponse = chesscom_openapi::models::error::Error;

// Convenience
pub type Result<T> = std::result::Result<T, Error>;
