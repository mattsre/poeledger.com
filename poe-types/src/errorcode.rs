use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ApiError {
    #[error("encountered an unknown error code from the API: `{0}`")]
    UnknownApiErrorCode(u16),
}

#[derive(Debug, PartialEq)]
pub enum ApiErrorCode {
    Accepted,
    ResourceNotFound,
    InvalidQuery,
    RateLimitExceeded,
    InternalError,
    UnexpectedContentType,
    Forbidden,
    TemporarilyUnavailable,
    Unauthorized,
    MethodNotAllowed,
    UnprocessableEntity,
}

impl TryFrom<u16> for ApiErrorCode {
    type Error = ApiError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ApiErrorCode::Accepted),
            1 => Ok(ApiErrorCode::ResourceNotFound),
            2 => Ok(ApiErrorCode::InvalidQuery),
            3 => Ok(ApiErrorCode::RateLimitExceeded),
            4 => Ok(ApiErrorCode::InternalError),
            5 => Ok(ApiErrorCode::UnexpectedContentType),
            6 => Ok(ApiErrorCode::Forbidden),
            7 => Ok(ApiErrorCode::TemporarilyUnavailable),
            8 => Ok(ApiErrorCode::Unauthorized),
            9 => Ok(ApiErrorCode::MethodNotAllowed),
            10 => Ok(ApiErrorCode::UnprocessableEntity),
            _ => Err(ApiError::UnknownApiErrorCode(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::errorcode::{ApiError, ApiErrorCode};

    #[test]
    fn test_api_error_codes() {
        assert_eq!(ApiErrorCode::Accepted, ApiErrorCode::try_from(0).unwrap());
        assert_eq!(
            ApiErrorCode::ResourceNotFound,
            ApiErrorCode::try_from(1).unwrap()
        );
        assert_eq!(
            ApiErrorCode::InvalidQuery,
            ApiErrorCode::try_from(2).unwrap()
        );
        assert_eq!(
            ApiErrorCode::RateLimitExceeded,
            ApiErrorCode::try_from(3).unwrap()
        );
        assert_eq!(
            ApiErrorCode::InternalError,
            ApiErrorCode::try_from(4).unwrap()
        );
        assert_eq!(
            ApiErrorCode::UnexpectedContentType,
            ApiErrorCode::try_from(5).unwrap()
        );
        assert_eq!(ApiErrorCode::Forbidden, ApiErrorCode::try_from(6).unwrap());
        assert_eq!(
            ApiErrorCode::TemporarilyUnavailable,
            ApiErrorCode::try_from(7).unwrap()
        );
        assert_eq!(
            ApiErrorCode::Unauthorized,
            ApiErrorCode::try_from(8).unwrap()
        );
        assert_eq!(
            ApiErrorCode::MethodNotAllowed,
            ApiErrorCode::try_from(9).unwrap()
        );
        assert_eq!(
            ApiErrorCode::UnprocessableEntity,
            ApiErrorCode::try_from(10).unwrap()
        );

        assert_eq!(true, ApiErrorCode::try_from(100).is_err());
        assert_eq!(
            ApiError::UnknownApiErrorCode(100),
            ApiErrorCode::try_from(100).unwrap_err()
        );
    }
}
