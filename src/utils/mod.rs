use std::{
    fs::File,
    io::{Error, Result},
    path::Path,
};

pub fn create_file(file_path: &str) -> Result<File> {
    let path = Path::new(file_path);
    match Path::new(path.parent().unwrap()).exists() {
        false => std::fs::create_dir_all(path.parent().unwrap())?,
        _ => (),
    };
    let file = File::create(path)?;
    Ok(file)
}
