use super::passwordmanager::PasswordEntry;
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;
pub struct SecurityManager {
    key_file: String,
    // db_path: String,
    // pub pw_vec: Vec<PasswordEntry>,
}
#[derive(Serialize, Deserialize)]
struct KeyPair {
    login_key: String,
    data_key: String,
}

impl SecurityManager {
    pub fn new(mut path: String) -> SecurityManager {
        path.push_str("/keys.json");
        SecurityManager { key_file: path }
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
        // if let Err(e) = fs::read_to_string(&self.key_file) {
        //     panic!("FILE {}", self.key_file);
        // }
        // None
        let raw_keys_json = fs::read_to_string(&self.key_file).expect("json not there");

        if raw_keys_json.is_empty() {
            return None;
        }
        let v: KeyPair = serde_json::from_str(&raw_keys_json).expect("Could not read json");
        Some(v)
    }

    // pub encrypt_data_pw
}
