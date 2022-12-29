use super::passwordmanager::PasswordEntry;
use std::fs;
pub struct DataManager {
    db_path: String,
    pub pw_vec: Vec<PasswordEntry>,
}

impl DataManager {
    pub fn new(db_path: &str) -> DataManager {
        let raw_content = fs::read_to_string(db_path).expect("db not there");
        let pw_vec: Vec<PasswordEntry> =
            serde_json::from_str(&raw_content).expect("Couldnt read from str");
        DataManager {
            db_path: String::from(db_path),
            pw_vec,
        }
    }
}
