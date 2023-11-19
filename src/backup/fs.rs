use anyhow::Ok;
use serde::Serialize;
use std::{fs, path::Path};

pub fn write_data<T: Serialize>(
    data: &T,
    destination_dir: &str,
    title: &str,
) -> Result<(), anyhow::Error> {
    if !Path::new(destination_dir).is_dir() {
        fs::create_dir_all(destination_dir)?
    }

    let file_path = format!("{}/{}", destination_dir, title);
    let json_data = serde_json::to_string_pretty(&data)?;

    fs::write(&file_path, json_data)?;
    Ok(())
}
