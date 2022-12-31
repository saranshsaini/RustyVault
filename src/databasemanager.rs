use super::passwordmanager::PasswordEntry;
use aes::cipher::generic_array::{typenum::U32, GenericArray};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use argon2::{self, Config};
use chrono::Local;
use directories::ProjectDirs;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::str;

pub struct DatabaseManager {
    pub key_file: String,
    pub db_file: String,
    pub pw_vec: Vec<PasswordEntry>,
}
#[derive(Serialize, Deserialize)]
struct KeyPair {
    login_key: String,
    nonce: [u8; 12],
}

impl DatabaseManager {
    pub fn new() -> DatabaseManager {
        let projects_path = ProjectDirs::from("com", "RustyVaultTeam", "RustyVault")
            .expect("Could not access home folder from OS");
        let mut dir_path = String::from(projects_path.config_dir().to_str().unwrap());
        let mut db_file = dir_path.clone();
        db_file.push_str("/data.json");
        dir_path.push_str("/keys.json");
        DatabaseManager {
            key_file: dir_path,
            db_file,
            pw_vec: Vec::new(),
        }
    }
    pub fn populate_data(&mut self, db_key: &String) {
        let raw_string = fs::read(&self.db_file).unwrap();
        if raw_string.is_empty() {
            self.pw_vec = Vec::new();
            return;
        }
        let raw_content = self.aes_decrypt(db_key, &raw_string);
        let decrypted_json = str::from_utf8(&raw_content).unwrap();
        let pw_vec: Vec<PasswordEntry> =
            serde_json::from_str(decrypted_json).expect("Couldnt read from str");
        self.pw_vec = pw_vec;
    }
    pub fn argon2_pw(&self, pw: &str, salt: &str) -> String {
        let config = Config::default();
        let encoded = argon2::hash_encoded(pw.as_bytes(), salt.as_bytes(), &config).unwrap();
        encoded
    }

    pub fn verify_login_pw(&self, raw_pw: &String) -> bool {
        let kp = self.get_keypair().expect("Login Info corrupted");
        argon2::verify_encoded(&kp.login_key, raw_pw.as_bytes()).unwrap()
    }

    pub fn save_login_pw(&self, raw_pw: &str) {
        let salt = "harcodedsalthatshouldprobablybechangedtosomethingelseatonepoint1";
        let en_pw_string = self.argon2_pw(raw_pw, salt);
        let mut k = KeyPair {
            login_key: en_pw_string,
            nonce: [0u8; 12],
        };
        if let Some(existing_kp) = self.get_keypair() {
            k.nonce = existing_kp.nonce;
        }
        self.save_keypair(k);
    }
    fn get_keypair(&self) -> Option<KeyPair> {
        let raw_keys_json = fs::read_to_string(&self.key_file).expect("json not there");
        if raw_keys_json.is_empty() {
            return None;
        }
        let v: KeyPair = serde_json::from_str(&raw_keys_json).expect("Could not read json");
        Some(v)
    }

    fn save_keypair(&self, kp: KeyPair) {
        let json = serde_json::to_string(&kp).expect("Couldn't serialize KeyPair");
        fs::write(&self.key_file, json).expect("Couldn't write KeyPair to file");
    }

    pub fn get_db_key(&self, pw: &String) -> String {
        let db_key_salt = "hardcodedsaltfordbkeythatcouldbechangedinthefuture123";
        self.argon2_pw(pw, db_key_salt)
    }

    fn aes_encrypt(&self, key: &String, json_plaintext: String, nonce: &[u8]) -> Vec<u8> {
        self.cipher(&key)
            .encrypt(Nonce::from_slice(nonce), json_plaintext.as_bytes())
            .unwrap()
    }

    fn aes_decrypt(&self, key: &String, enc_json: &Vec<u8>) -> Vec<u8> {
        let kp = self.get_keypair().expect("Corrupted key");
        self.cipher(&key)
            .decrypt(Nonce::from_slice(&kp.nonce), &enc_json[..])
            .unwrap()
    }

    fn cipher(&self, key: &String) -> Aes256Gcm {
        let key_bytes = &key.as_bytes()[..32];
        let key: GenericArray<_, U32> = GenericArray::clone_from_slice(key_bytes);
        Aes256Gcm::new_from_slice(&key).unwrap()
    }

    pub fn save_pw_vec(&self, db_key: &String) {
        let json = serde_json::to_string(&self.pw_vec).expect("Couldn't serialize pw vec");
        let nonce = rand::thread_rng().gen::<[u8; 12]>();
        let encrypted_json = self.aes_encrypt(db_key, json, &nonce);
        let mut kp = self.get_keypair().expect("Keys corrupted");
        kp.nonce = nonce;
        self.save_keypair(kp);
        fs::write(&self.db_file, encrypted_json)
            .expect("Couldn't write PasswordEntry json to file");
    }

    pub fn add_pw(&mut self, sitename: String, username: String, raw_pw: String, db_key: &String) {
        let new_pw = PasswordEntry::new(sitename, username, raw_pw);
        self.pw_vec.push(new_pw);
        self.save_pw_vec(db_key);
    }
    pub fn delete_site(&mut self, index: usize, db_key: &String) {
        self.pw_vec.remove(index);
        self.save_pw_vec(db_key);
    }
    pub fn update_password(&mut self, index: usize, new_pw: String, db_key: &String) {
        let mut entry = self.pw_vec.get_mut(index).unwrap();
        entry.password = new_pw;
        entry.last_updated = Local::now().date_naive();
        self.save_pw_vec(db_key);
    }
}
