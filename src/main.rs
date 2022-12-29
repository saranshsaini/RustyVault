// use crate::passwordmanager::PasswordManager;
mod db;
mod passwordmanager;
// use rmp_serde as rmps;
// use rmps::{Deserializer, Serializer};
fn main() {
    let mut pw = passwordmanager::PasswordManager::new();
    if let Err(e) = pw.show() {
        println!("error: {}", e);
    }
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
