use serde::Serialize;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Cat {
	pub id: i32,
	pub name: String,
}
