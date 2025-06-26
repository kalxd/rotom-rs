#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct SaltPassword(String);

impl SaltPassword {
	pub fn new(password: &str, salt: &str) -> Self {
		Self(String::from(format!("{salt}-{password}-{salt}")))
	}
}
