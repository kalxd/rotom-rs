use ralts::error::{QuickThrow, Result};

use crate::data::{AppState, ty::FileExtension};

async fn get_file_by_sha(sha: &str, state: &AppState) -> Result<Option<FileExtension>> {
	let ext = sqlx::query_scalar!(
		r#"
select 扩展名 as "extension!: FileExtension" from 文件
where 特征 = $1
"#,
		sha
	)
	.fetch_optional(&state.db)
	.await?;

	Ok(ext)
}

pub async fn check_file_by_sha(sha: &str, state: &AppState) -> Result<FileExtension> {
	get_file_by_sha(sha, state).await?.not_found("文件不存在！")
}
