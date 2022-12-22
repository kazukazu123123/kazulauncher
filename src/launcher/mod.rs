use std::io;
use std::process::{Child, Command};

pub fn launch(cmd: &str) -> io::Result<Child> {
    Command::new(cmd).spawn()
}
