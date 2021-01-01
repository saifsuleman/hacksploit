use crate::HacksploitModule;

pub struct FirefoxModule {}

impl FirefoxModule {
    pub fn new() -> Self {
        todo!()
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
