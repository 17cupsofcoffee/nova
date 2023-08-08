use std::path::PathBuf;
use std::sync::OnceLock;

pub fn base_path() -> &'static PathBuf {
    static BASE_PATH: OnceLock<PathBuf> = OnceLock::new();

    // TODO: Make this use SDL_GetBaseDir when packaging for release
    BASE_PATH.get_or_init(|| std::env::current_dir().unwrap())
}

pub fn asset_path(path: &str) -> PathBuf {
    base_path().join(path)
}

pub fn read(path: &str) -> Vec<u8> {
    let full_path = asset_path(path);

    std::fs::read(full_path).unwrap()
}

pub fn read_to_string(path: &str) -> String {
    let full_path = asset_path(path);

    std::fs::read_to_string(full_path).unwrap()
}
