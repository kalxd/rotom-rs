use futures::StreamExt;
use ntex::web::{DefaultError, Scope, post, scope, types::Json};
use ntex_multipart::Multipart;
use sha2::{Digest, Sha256};

use crate::data::{
	User,
	error::{Error, Result},
	file,
};

fn guard_file_type(ext: &str) -> Result<()> {
	match ext {
		"png" | "jpeg" | "jpg" => Ok(()),
		_ => Err(Error::illegal("无效的文件类型！")),
	}
}

#[post("/upload")]
async fn upload_file(_: User, mut body: Multipart) -> Result<Json<()>> {
	let mut field = body
		.next()
		.await
		.ok_or(Error::illegal("没有获取到上传文件！"))?
		.map_err(Error::internal)?;

	let file_type = field.content_type().subtype().to_string();
	guard_file_type(&file_type)?;

	let mut file_content: Vec<u8> = vec![]; // 保存到内存，计算完整的sha才能确定文件名。
	let mut hasher = Sha256::new();

	while let Some(chunk) = field.next().await {
		let chunk = chunk.map_err(Error::internal)?;
		hasher.update(&chunk);
		file_content.extend(chunk);
	}

	let file_hash = hasher.finalize();
	let filename = format!("{:x}", file_hash);
	let filepath = file::with_filename(&filename, &file_type);
	dbg!(filepath);

	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/file").service(upload_file)
}
