use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum Error {
	/// 401
	#[error("Unauthorized")]
	Unauthorized,

	/// 403
	#[error("Forbidden")]
	Forbidden,

	/// 404
	#[error("Not Found")]
	NotFound,

	/// 500
	#[error("Internal Server Error")]
	InternalServerError,

	#[error("Deserialize Error")]
	DeserializeError,

	#[error("Http Request Error")]
	RequestError,
}
