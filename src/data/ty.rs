use serde::{Deserialize, Serialize, de::Error};

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Uuid(uuid::Uuid);

impl TryFrom<&str> for Uuid {
	type Error = uuid::Error;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		uuid::Uuid::try_from(value).map(Self)
	}
}

impl Serialize for Uuid {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut buf = uuid::Uuid::encode_buffer();
		let s = self.0.hyphenated().encode_lower(&mut buf);
		serializer.serialize_str(s)
	}
}

impl<'de> Deserialize<'de> for Uuid {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Self::try_from(s.as_str())
			.map_err(|e| D::Error::custom(format!("parsing UUID failed: {e}")))
	}
}

#[derive(Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct SaltPassword(String);

impl SaltPassword {
	pub fn new(password: &str, salt: &str) -> Self {
		Self(String::from(format!("{salt}-{password}-{salt}")))
	}
}

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "file_type")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum FileExtension {
	Jpg,
	Png,
	Webp,
}

impl std::fmt::Display for FileExtension {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Jpg => write!(f, "jpg"),
			Self::Png => write!(f, "png"),
			Self::Webp => write!(f, "webp"),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct UpdateBody<T> {
	pub id: i32,
	pub data: T,
}

#[derive(Debug, Serialize)]
pub struct Pager<T> {
	pub count: i64,
	pub hits: Vec<T>,
}

impl<T, TS> From<(i64, TS)> for Pager<T>
where
	TS: IntoIterator<Item = T>,
{
	fn from((count, hits): (i64, TS)) -> Self {
		Self {
			count,
			hits: Vec::from_iter(hits),
		}
	}
}
