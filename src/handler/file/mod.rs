use std::path::PathBuf;

use futures::StreamExt;
use ntex::web::{DefaultError, Scope, post, scope, types::Json};
use ntex_multipart::Multipart;
use sha2::{Digest, Sha256};

const BASE_DIR: &str = "static";

use crate::data::{
	User,
	error::{Error, Result},
};

fn concat_file<Ext: AsRef<str>>(filename: String, ext: Ext) -> PathBuf {
	let mut path = PathBuf::from(BASE_DIR);

	path.push(filename);
	path.set_extension(ext.as_ref());

	path
}

#[post("/upload")]
async fn upload_file(_: User, mut body: Multipart) -> Result<Json<()>> {
	let mut field = body
		.next()
		.await
		.ok_or(Error::illegal("没有获取到上传文件！"))?
		.map_err(Error::internal)?;

	let mut file_content: Vec<u8> = vec![]; // 保存到内存，计算完整的sha再确定文件名。
	let mut hasher = Sha256::new();

	while let Some(chunk) = field.next().await {
		let chunk = chunk.map_err(Error::internal)?;
		hasher.update(&chunk);
		file_content.extend(chunk);
	}

	let file_hash = hasher.finalize();
	let filename = format!("{:x}", file_hash);
	let filepath = concat_file(filename, field.content_type().subtype());
	dbg!(filepath);

	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/file").service(upload_file)
}
