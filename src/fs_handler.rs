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

            let file_name = match item.file_name().to_str() {
                Some(val) => val.to_string(),
                None => return None,
            };

            if !file_name.ends_with(".json") {
                return None;
            }

            let meta_data = match item.metadata() {
                Ok(val) => val,
                Err(_) => return None,
            };

            let created_at = match meta_data.created() {
                Ok(val) => val,
                Err(_) => return None,
            };

            return Some((created_at, file_name));
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
            let latest_json_path = format!("{}{}", given_path, val.1);

            match fs::read_to_string(&latest_json_path) {
                Ok(val) => Ok(Some(val)),
                Err(_) => {
                    let error_msg =
                        format!("Could not read contents of file at {}", &latest_json_path);
                    return Err(error_msg);
                }
            }
        }
        None => Ok(None),
    }
}

pub fn dump_backup(destination_dir: &str, title: &str, data: &str) -> Result<(), String> {
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
