use std::{fs, io::Write};

use futures::StreamExt;
use ntex::web::{
	DefaultError, Scope, get, post, scope,
	types::{Json, Path, State},
};
use ntex_files::NamedFile;
use ntex_multipart::Multipart;
use ralts::error::{AppError, QuickThrow, Result};
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::data::{AppState, User, file as filedata, ty::FileExtension};
use crate::helper;

fn guard_file_type(ext: Option<&str>) -> Result<FileExtension> {
	match ext {
		Some("png") => Ok(FileExtension::Png),
		Some("jpeg") | Some("jpg") => Ok(FileExtension::Jpg),
		Some("webp") => Ok(FileExtension::Webp),
		_ => Err(AppError::internal("无效的文件类型！")),
	}
}

struct SaveFile {
	sha: String,
	ext: FileExtension,
}

async fn save_file(mut body: Multipart) -> Result<SaveFile> {
	let mut field = body.next().await.internal("没有获取到上传文件！")??;

	let file_type = field
		.content_type()
		.filter(|t| t.type_().as_ref() == "image")
		.map(|t| t.subtype().as_str());
	let file_type = guard_file_type(file_type)?;

	let mut file_content: Vec<u8> = vec![]; // 保存到内存，计算完整的sha才能确定文件名。
	let mut hasher = Sha256::new();

	while let Some(chunk) = field.next().await {
		let chunk = chunk?;
		hasher.update(&chunk);
		file_content.extend(chunk);
	}

	let file_hash = hasher.finalize();
	let filename = format!("{:x}", file_hash);
	let filepath = filedata::with_filename(&filename, &file_type);

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
set 更新日期 = now()
returning 特征 as sha, 扩展名 as "extension: FileExtension";
"#,
		local_file.sha,
		local_file.ext as FileExtension
	)
	.fetch_one(&state.db)
	.await?;

	Ok(Json(file))
}

#[get("/view/{id}")]
async fn view_file(id: Path<String>, state: State<AppState>) -> Result<NamedFile> {
	let ext = helper::file::check_file_by_sha(&id, &state).await?;
	let filepath = filedata::with_filename(&id, &ext);
	Ok(NamedFile::open(filepath)?)
}

pub fn api() -> Scope<DefaultError> {
	scope("/file").service(upload_file).service(view_file)
}
