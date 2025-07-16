use ntex::{
	http::{Response, StatusCode},
	web::{self, WebResponseError},
};
use serde::Serialize;

#[derive(Debug)]
pub enum Error {
	/// 未登录，或者权限不足。
	NotAuth(String),
	/// 前端提供参数不合法。
	Illegal(String),
	/// 资源未找到。
	NotFound(String),
	/// 内部错误。
	Internal(String),
}

macro_rules! error_fn {
	($fn_name:tt, $field:tt) => {
		pub fn $fn_name<S: ToString>(msg: S) -> Self {
			Self::$field(msg.to_string())
		}
	};
}

impl Error {
	error_fn!(not_auth, NotAuth);
	error_fn!(illegal, Illegal);
	error_fn!(not_found, NotFound);
	error_fn!(internal, Internal);
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NotAuth(s) => write!(f, "验证失败：{s}"),
			Self::Illegal(s) => write!(f, "参数错误：{s}"),
			Self::NotFound(s) => write!(f, "资源不存在：{s}"),
			Self::Internal(s) => write!(f, "内部错误：{s}"),
		}
	}
}

impl From<sqlx::Error> for Error {
	fn from(value: sqlx::Error) -> Self {
		Self::internal(value)
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::internal(value)
	}
}

impl From<ntex_multipart::MultipartError> for Error {
	fn from(value: ntex_multipart::MultipartError) -> Self {
		Self::illegal(value)
	}
}

impl WebResponseError for Error {
	fn status_code(&self) -> StatusCode {
		match self {
			Self::NotAuth(_) => StatusCode::FORBIDDEN,
			Self::Illegal(_) => StatusCode::NOT_ACCEPTABLE,
			Self::NotFound(_) => StatusCode::NOT_FOUND,
			Self::Internal(_) => StatusCode::BAD_REQUEST,
		}
	}

	fn error_response(&self, _: &web::HttpRequest) -> Response {
		#[derive(Serialize)]
		struct Body {
			msg: String,
		}

		let status_code = self.status_code();
		let body = Body {
			msg: self.to_string(),
		};
		Response::build(status_code).json(&body)
	}
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
