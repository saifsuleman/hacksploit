use crate::HacksploitModule;

const USERNAME_TEST: &str = "MEoEEPgAAAAAAAAAAAAAAAAAAAEwFAYIKoZIhvcNAwcECEABtbofCyjGBCCv0uPuFW7yZFeXZMMttC6b18G1P6o97nVe+VMycAieRQ==";
const PASSWORD_TEST: &str =
    "MDoEEPgAAAAAAAAAAAAAAAAAAAEwFAYIKoZIhvcNAwcECF7DKv38R6nvBBB/pKlHhoMNo4AI/0Uoxmvg";

pub struct FirefoxModule {}

impl FirefoxModule {
    pub fn new() -> Self {
        FirefoxModule {}
    }
}

impl HacksploitModule for FirefoxModule {
    fn on_command(&self, args: Vec<&str>) -> String {
        format!("Not implemented yet.")
    }

    fn get_name(&self) -> String {
        format!("firefox")
    }

    fn get_description(&self) -> String {
        format!("Decrypts and prints firefox passwords")
    }
}

fn decode_login_data(vec: Vec<u8>) -> Vec<u8> {
    // Base 64 decode it first
    // ASN1DER decode afterwards
    let data: Vec<u8> = match base64::decode(vec.clone()) {
        Ok(data) => data,
        Err(_) => return vec.clone(),
    };
    vec
}
