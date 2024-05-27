//! Collection of api to get device info
//!

pub mod disk;
pub mod os;

pub fn battery() -> std::io::Result<i32> {
    use std::io::{Error, ErrorKind, Read};
    use std::process::{Command, Stdio};

    let cmd = Command::new("cat")
        .arg("/sys/class/power_supply/BAT0/capacity")
        .stdout(Stdio::piped())
        .spawn()?;

    let Some(mut out) = cmd.stdout else { unreachable!() };
    let mut bat = String::new();

    let _ = out.read_to_string(&mut bat)?;

    bat.trim().parse::<i32>().map_err(|e|Error::new(ErrorKind::InvalidData, e))
}

