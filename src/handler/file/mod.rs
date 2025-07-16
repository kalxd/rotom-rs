use std::{fs, io::Write};

use futures::StreamExt;
use ntex::web::{
	DefaultError, Scope, post, scope,
	types::{Json, State},
};
use ntex_multipart::Multipart;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::data::{
	AppState, User,
	error::{Error, Result},
	file,
	ty::FileExtension,
};

fn guard_file_type(ext: &str) -> Result<FileExtension> {
	match ext {
		"png" => Ok(FileExtension::Png),
		"jpeg" | "jpg" => Ok(FileExtension::Jpg),
		"webp" => Ok(FileExtension::Webp),
		_ => Err(Error::illegal("无效的文件类型！")),
	}
}

struct SaveFile {
	sha: String,
	ext: FileExtension,
}

async fn save_file(mut body: Multipart) -> Result<SaveFile> {
	let mut field = body
		.next()
		.await
		.ok_or(Error::illegal("没有获取到上传文件！"))??;

	let file_type = field.content_type().subtype().to_string();
	let file_type = guard_file_type(&file_type)?;

	let mut file_content: Vec<u8> = vec![]; // 保存到内存，计算完整的sha才能确定文件名。
	let mut hasher = Sha256::new();

	while let Some(chunk) = field.next().await {
		let chunk = chunk?;
		hasher.update(&chunk);
		file_content.extend(chunk);
	}

	let file_hash = hasher.finalize();
	let filename = format!("{:x}", file_hash);
	let filepath = file::with_filename(&filename, &file_type);

	let mut f = fs::File::create(filepath)?;
	f.write_all(&file_content)?;
	drop(f);

	Ok(SaveFile {
		sha: filename,
		ext: file_type,
	})
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct File {
	id: i32,
	sha: String,
	extension: FileExtension,
}

#[post("/upload")]
async fn upload_file(_: User, body: Multipart, state: State<AppState>) -> Result<Json<File>> {
	let local_file = save_file(body).await?;

	let file = sqlx::query_as!(
		File,
		r#"
insert into 文件
(特征, 扩展名)
values ($1, $2)
on conflict (特征) do update
set 创建日期 = now()
returning 编号 as id, 特征 as sha, 扩展名 as "extension: FileExtension";
"#,
		local_file.sha,
		local_file.ext as FileExtension
	)
	.fetch_one(&state.db)
	.await?;

	Ok(Json(file))
}

pub fn api() -> Scope<DefaultError> {
	scope("/file").service(upload_file)
}
