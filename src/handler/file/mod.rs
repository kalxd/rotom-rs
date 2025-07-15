use futures::StreamExt;
use ntex::web::{DefaultError, Scope, post, scope, types::Json};
use ntex_multipart::Multipart;
use sha2::{Digest, Sha256};

use crate::data::{
	User,
	error::{self, Result},
};

#[post("/upload")]
async fn upload_file(_: User, mut body: Multipart) -> Result<Json<()>> {
	while let Some(item) = body.next().await {
		let mut field = item.map_err(error::Error::internal)?;

		let mut file_content: Vec<u8> = vec![]; // 保存到内存，计算完整的sha再确定文件名。
		let mut hasher = Sha256::new();

		dbg!(field.content_type());
		dbg!(field.content_type().type_());
		dbg!(field.content_type().subtype());

		while let Some(chunk) = field.next().await {
			let chunk = chunk.map_err(error::Error::internal)?;
			hasher.update(&chunk);
			file_content.extend(chunk);
		}

		let file_hash = hasher.finalize();
		let filename = format!("{:x}", file_hash);
		dbg!(filename);
	}
	Ok(Json(()))
}

pub fn api() -> Scope<DefaultError> {
	scope("/file").service(upload_file)
}
