#[derive(Debug)]
pub enum Error {
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
	error_fn!(internal, Internal);
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
