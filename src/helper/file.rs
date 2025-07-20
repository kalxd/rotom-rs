use crate::data::{
	AppState,
	error::{Error, Result},
	ty::FileExtension,
};

#[derive(Debug, drv::State, drv::Database)]
pub struct FileState(#[database] AppState);

impl FileState {
	pub async fn get_file_by_sha(&self, sha: &str) -> Result<Option<FileExtension>> {
		let ext = sqlx::query_scalar!(
			r#"
select 扩展名 as "extension!: FileExtension" from 文件
where 特征 = $1
"#,
			sha
		)
		.fetch_optional(self)
		.await?;

		Ok(ext)
	}

	pub async fn check_file_by_sha(&self, sha: &str) -> Result<FileExtension> {
		self.get_file_by_sha(sha)
			.await?
			.ok_or(Error::not_found("文件不存在！"))
	}
}
