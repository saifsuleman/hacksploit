use std::io::Error;
use std::process::Command;

use crate::HacksploitModule;

pub struct SpawnInstanceModule {}

impl SpawnInstanceModule {
    pub fn new() -> Self {
        SpawnInstanceModule {}
    }
}

impl HacksploitModule for SpawnInstanceModule {
    fn on_command(&self, args: Vec<&str>) -> String {
        let addr = args.get(0);
        match addr {
            Some(addr) => match spawn(addr) {
                Ok(out) => out,
                Err(_) => format!("Usage - spawninstance <addr>"),
            },
            None => format!("Usage - spawninstance <addr>"),
        }
    }

    fn get_name(&self) -> String {
        format!("spawninstance")
    }

    fn get_description(&self) -> String {
        format!("Spawns another HackSploit instance at the address [Usage: spawninstance <addr>]")
    }
}

fn spawn(addr: &str) -> Result<String, Error> {
    let output: String = match std::env::current_exe()?.into_os_string().into_string() {
        Ok(exe) => {
            Command::new(exe).arg(addr).spawn()?;
            format!("Spawned successfully...\n")
        }
        Err(_) => format!("Error getting current executable..\n"),
    };
    Ok(output)
}
