use sha2::{Digest, Sha256};

use super::BackupData;

use std::{
    fs::{self},
    path::Path,
};

pub fn get_latest_json(given_path: &str) -> Result<Option<String>, String> {
    let path = Path::new(given_path);

    if !path.is_dir() {
        return Ok(None);
    }

    let dir_items = match fs::read_dir(path) {
        Ok(val) => val,
        Err(_) => {
            let error_msg = format!("Could not traverse {}", given_path);
            return Err(error_msg);
        }
    };

    let latest_file_metadata = dir_items
        .map(|dir_result| {
            let item = match dir_result {
                Ok(res) => res,
                Err(_) => return None,
            };

            let file_path = match item.path().to_str() {
                Some(val) => {
                    if val.ends_with(".json") {
                        val.to_owned()
                    } else {
                        return None;
                    }
                }
                None => return None,
            };

            let meta_data = match item.metadata() {
                Ok(val) => val,
                Err(_) => return None,
            };

            let created_at = match meta_data.created() {
                Ok(val) => val,
                Err(_) => return None,
            };

            return Some((created_at, file_path));
        })
        .filter_map(std::convert::identity)
        .reduce(|curr, new| {
            if curr.0.max(new.0) == curr.0 {
                return curr;
            } else {
                return new;
            }
        });

    match latest_file_metadata {
        Some(val) => {
            let (_, latest_json_path) = val;

            match fs::read_to_string(&latest_json_path) {
                Ok(val) => Ok(Some(val)),
                Err(_) => {
                    let error_msg = format!("Could not read contents of file at {}", &latest_json_path);
                    return Err(error_msg);
                }
            }
        }
        None => Ok(None),
    }
}

pub fn write_data(destination_dir: &str, title: &str, data: &str) -> Result<(), String> {
    let path = Path::new(destination_dir);

    if !path.is_dir() && fs::create_dir_all(destination_dir).is_err() {
        return Err(format!("Could not create {}", destination_dir));
    }

    let file_path = format!("{}/{}", destination_dir, title);

    match fs::write(&file_path, &data) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not write to {}", file_path)),
    }
}

fn get_hash(data: &str) -> String {
    format!(
        "{:x}",
        Sha256::digest(data.chars().filter(|c| !c.is_whitespace()).collect::<String>().as_bytes())
    )
}

pub fn dump_data(destination_dir: &str, data: &BackupData) -> Result<String, String> {
    let json_data = match serde_json::to_string_pretty(&data) {
        Ok(val) => val,
        Err(_) => return Err("Could not serialize backup data.".to_owned()),
    };

    let new_hash = get_hash(&json_data);

    let existing_backup = match get_latest_json(destination_dir) {
        Ok(val) => val,
        Err(_) => None,
    };

    if existing_backup.is_some() && get_hash(&existing_backup.unwrap()) == new_hash {
        return Ok("Existing backup is up to date.".to_owned());
    }

    match write_data(destination_dir, &format!("{}.json", &new_hash[5..15]), &json_data) {
        Ok(_) => return Ok(format!("Successfully backed up data to {}", destination_dir)),
        Err(e) => return Err(format!("{}", e)),
    }
}
