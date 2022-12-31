mod databasemanager;
mod passwordmanager;
fn main() {
    let mut pw = passwordmanager::PasswordManager::new();
    if let Err(e) = pw.show() {
        println!("error: {}", e);
    }
}
