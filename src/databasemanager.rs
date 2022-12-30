use super::passwordmanager::PasswordEntry;
use argon2::{self, Config};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;
use uuid::Uuid;
pub struct DatabaseManager {
    key_file: String,
    db_file: String,
    pub pw_vec: Vec<PasswordEntry>,
}
#[derive(Serialize, Deserialize)]
struct KeyPair {
    login_key: String,
    data_key: String,
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let projects_path = ProjectDirs::from("com", "RustyBoxTeam", "RustyBox")
            .expect("Could not access home folder from OS");
        let mut dir_path = String::from(projects_path.config_dir().to_str().unwrap());
        // let mut dir_path = String::from(dir_path);
        let mut db_file = dir_path.clone();
        db_file.push_str("/data.json");

        let raw_content = match fs::read_to_string(&db_file) {
            Err(e) => String::new(),
            Ok(r) => r,
        };
        let pw_vec: Vec<PasswordEntry>;
        if raw_content.is_empty() {
            pw_vec = Vec::new();
        } else {
            pw_vec = serde_json::from_str(&raw_content).expect("Couldnt read from str");
        }
        dir_path.push_str("/keys.json");
        DatabaseManager {
            key_file: dir_path,
            db_file,
            pw_vec,
        }
    }
    pub fn encrypt_login_pw(&self, pw: &str) -> String {
        let salt = "harcodedsalthatshouldprobablybechangedtosomethingelseatonepoint1";
        let config = Config::default();
        let encoded = argon2::hash_encoded(pw.as_bytes(), salt.as_bytes(), &config).unwrap();
        encoded
    }

    pub fn verify_login_pw(&self, raw_pw: &String) -> bool {
        // let encoded = self.encrypt_login_pw(raw_pw);
        let kp = self.get_keypair().expect("Login Info corrupted");

        argon2::verify_encoded(&kp.login_key, raw_pw.as_bytes()).unwrap()
    }
    pub fn argon_with_salt(&self, uuid: Uuid, username: &String, raw_pw: String) -> String {
        let config = Config::default();
        let salt = format!("{}{}", uuid, username);
        argon2::hash_encoded(raw_pw.as_bytes(), salt.as_bytes(), &config).unwrap()
    }
    pub fn save_login_pw(&self, raw_pw: &str) {
        let en_pw_string = self.encrypt_login_pw(raw_pw);
        let mut k = KeyPair {
            login_key: en_pw_string,
            data_key: String::new(),
        };
        if let Some(existing_kp) = self.get_keypair() {
            k.data_key = existing_kp.data_key;
        }
        let json = serde_json::to_string(&k).expect("Couldn't serialize KeyPair");
        fs::write(&self.key_file, json).expect("Couldn't write KeyPair to file");
    }
    fn get_keypair(&self) -> Option<KeyPair> {
        let raw_keys_json = fs::read_to_string(&self.key_file).expect("json not there");
        if raw_keys_json.is_empty() {
            return None;
        }
        let v: KeyPair = serde_json::from_str(&raw_keys_json).expect("Could not read json");
        Some(v)
    }
    pub fn add_pw(&mut self, sitename: String, username: String, raw_pw: String) {
        let uuid = Uuid::new_v4();
        let enc_pw = self.argon_with_salt(uuid, &username, raw_pw);
        let pw = PasswordEntry::new(sitename, username, enc_pw, uuid);
        self.pw_vec.push(pw);
        let json = serde_json::to_string(&self.pw_vec).expect("Couldn't serialize pw vec");
        fs::write(&self.db_file, json).expect("Couldn't write PasswordEntrys to file");
    }

    // pub encrypt_data_pw
}
