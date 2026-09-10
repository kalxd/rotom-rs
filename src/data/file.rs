use ralts::error::{AppError, Result};
use std::{
	fs,
	path::{Path, PathBuf},
};

use super::ty::FileExtension;

const BASE_DIR: &str = "static";

pub fn ensure_base_dir() -> Result<()> {
	let path = Path::new(BASE_DIR);
	fs::create_dir_all(path).map_err(AppError::internal)
}

pub fn with_filename(filename: &str, ext: &FileExtension) -> PathBuf {
	let mut path = PathBuf::from(BASE_DIR);

	path.push(filename);
	path.set_extension(ext.to_string());
	path
}
