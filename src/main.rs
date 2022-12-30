// use crate::passwordmanager::PasswordManager;

mod databasemanager;
mod passwordmanager;
use argon2::{self, Config};
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::path::Path;
// use rmp_serde as rmps;
// use rmps::{Deserializer, Serializer};
fn main() {
    // let pw = "mypass123";
    // let salt = "harcodedsalthatshouldprobablybechangedtosomethingelseatonepoint";
    // let config = Config::default();
    // let encoded = argon2::hash_encoded(pw.as_bytes(), salt.as_bytes(), &config).unwrap();
    // let ans = argon2::verify_encoded(&encoded[..], pw.as_bytes()).unwrap();
    // println!("ans: {}", ans);
    let mut pw = passwordmanager::PasswordManager::new();
    if let Err(e) = pw.show() {
        println!("error: {}", e);
    }
    // if let Some(proj_dirs) = ProjectDirs::from("com", "ssaini", "RustyBox") {
    //     let path = proj_dirs.config_dir();
    //     println!("path: {:?}", path);
    //     if path.exists() {
    //         println!("path exists: {:?}", path);
    //     }
    // }
    // let pw1 = Password {
    //     username: String::from("billy"),
    //     password: String::from("123"),
    //     site: String::from("elgoog.com"),
    //     created: Local::now(),
    //     last_updated: Local::now(),
    // };
    // let pw2 = Password {
    //     username: String::from("john"),
    //     password: String::from("456"),
    //     site: String::from("seafrs.com"),
    //     created: Local::now(),
    //     last_updated: Local::now(),
    // };
    // let v = vec![pw1, pw2];
    // let j = serde_json::to_string(&v).unwrap();

    // println!("{}", j);
    // let mut buf = Vec::new();
    // pw1.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // println!("Serialized: {:?}", buf);
    // let uns = rmp_serde::from_read(buf).unwrap();
    // let mut de = Deserializer::new(&buf[..]);
    // Deserialize::deserialize(&mut de).unwrap();
    // println!("Deserialized: {:?}", de);
}
