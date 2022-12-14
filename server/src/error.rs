// use actix_web::ResponseError;
// use actix_web::{http::StatusCode, HttpResponse};
// use bcrypt::BcryptError;
// use diesel::r2d2::{Error as R2D2Error, PoolError};
// use diesel::result::{DatabaseErrorKind, Error as DieselError};
// use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
// use serde_json::json;
// use serde_json::Value as JsonValue;
// use std::convert::From;
// use thiserror::Error;
// use uuid::Error as UuidError;
// use validator::ValidationErrors;

// #[derive(Error, Debug)]
// pub enum AppError {
//     // 401
//     #[error("Unauthorized: {}", _0)]
//     Unauthorized(JsonValue),

//     // 403
//     #[error("Forbidden: {}", _0)]
//     Forbidden(JsonValue),

//     // 404
//     #[error("Not Found: {}", _0)]
//     NotFound(JsonValue),

//     // 422
//     #[error("Unprocessable Entity: {}", _0)]
//     UnprocessableEntity(JsonValue),

//     // 500
//     #[error("Internal Server Error")]
//     InternalServerError,
//     // #[error("Validation Error: {:?}", _0)]
//     // ValidationError(Vec<String>),
// }

// impl ResponseError for AppError {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(msg),
//             AppError::Forbidden(ref msg) => HttpResponse::Forbidden().json(msg),
//             AppError::NotFound(ref msg) => HttpResponse::NotFound().json(msg),
//             AppError::UnprocessableEntity(ref msg) => HttpResponse::UnprocessableEntity().json(msg),
//             // AppError::ValidationError(ref msg) => todo!(),
//             AppError::InternalServerError => {
//                 HttpResponse::InternalServerError().json("Internal Server Error")
//             }
//         }
//     }
//     fn status_code(&self) -> StatusCode {
//         match *self {
//             AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
//             AppError::Forbidden(_) => StatusCode::FORBIDDEN,
//             AppError::NotFound(_) => StatusCode::NOT_FOUND,
//             AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
//             AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }

// impl From<PoolError> for AppError {
//     fn from(_err: PoolError) -> Self {
//         AppError::InternalServerError
//     }
// }

// impl From<BcryptError> for AppError {
//     fn from(err: BcryptError) -> Self {
//         match err {
//             BcryptError::InvalidHash(_) => AppError::Unauthorized(json!({
//                 "error": "PW is invalid"
//             })),
//             _ => AppError::InternalServerError,
//         }
//     }
// }

// impl From<JwtError> for AppError {
//     fn from(err: JwtError) -> Self {
//         match err.kind() {
//             JwtErrorKind::InvalidToken => AppError::Unauthorized(json!({
//                 "error": "Token is invalid"
//             })),
//             JwtErrorKind::InvalidIssuer => AppError::Unauthorized(json!({
//                 "error": "Issuer is invalid",
//             })),
//             _ => AppError::Unauthorized(json!({
//                 "error": "An issue was found with the token provided",
//             })),
//         }
//     }
// }

// impl From<R2D2Error> for AppError {
//     fn from(_err: R2D2Error) -> Self {
//         AppError::InternalServerError
//     }
// }

// impl From<DieselError> for AppError {
//     fn from(err: DieselError) -> Self {
//         match err {
//             DieselError::DatabaseError(kind, info) => {
//                 if let DatabaseErrorKind::UniqueViolation = kind {
//                     let message = info.details().unwrap_or_else(|| info.message()).to_string();
//                     AppError::UnprocessableEntity(json!({ "error": message }))
//                 } else {
//                     AppError::InternalServerError
//                 }
//             }
//             DieselError::NotFound => {
//                 AppError::NotFound(json!({ "error": "requested record was not found" }))
//             }
//             _ => AppError::InternalServerError,
//         }
//     }
// }

// impl From<UuidError> for AppError {
//     fn from(_err: UuidError) -> Self {
//         AppError::NotFound(json!({"error":"Uuid is invalid."}))
//     }
// }

// impl From<ValidationErrors> for AppError {
//     fn from(errors: ValidationErrors) -> Self {
//         let error_messages: Vec<String> = errors
//             .field_errors()
//             .into_iter()
//             .map(|err| {
//                 let default = format!("{} fails validation", err.0);
//                 err.1[0]
//                     .message
//                     .as_ref()
//                     .unwrap_or(&std::borrow::Cow::Owned(default))
//                     .to_string()
//             })
//             .collect();

//         AppError::UnprocessableEntity(error_messages.into())
//     }
// }
