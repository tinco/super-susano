use std::env;
use std::path::PathBuf;

pub fn assets_path() -> PathBuf {
	return env::current_exe().unwrap().parent().unwrap().join("assets");
}

pub fn asset_path(path: &str) -> PathBuf {
	return assets_path().join(path);

}
