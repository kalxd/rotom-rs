use std::{
	fs,
	path::{Path, PathBuf},
};

use super::error::{Error, Result};

const BASE_DIR: &str = "static";

pub fn ensure_base_dir() -> Result<()> {
	let path = Path::new(BASE_DIR);
	fs::create_dir_all(path).map_err(Error::internal)
}

pub fn with_filename(filename: &str, ext: &str) -> PathBuf {
	let mut path = PathBuf::from(BASE_DIR);

	path.push(filename);
	path.set_extension(ext);
	path
}
