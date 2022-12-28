// use crate::passwordmanager::PasswordManager;
mod passwordmanager;

fn main() {
    let mut pw = passwordmanager::PasswordManager::new();
    pw.show();
}
