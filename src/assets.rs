use std::env;
use std::path::PathBuf;

pub fn assets_path() -> PathBuf {
	return env::current_exe().unwrap().parent().unwrap().join("assets");
}

pub fn bitmaps_path() -> PathBuf {
	return assets_path().join("bitmaps");
}

pub fn shaders_path() -> PathBuf {
	return assets_path().join("shaders");
}