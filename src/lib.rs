//! Collection of api to get device info
//!

//! TODO:
//! [ ] get laptop brand
//! [ ] get ram information
//! [ ] get storage information
//! [ ] get partition, flags, mountpoint, unallocated, filesystem
//! [ ] get all installed os
//! [x] get battery percent
//! [ ] get internet connection
//! [ ] get bootloader
//! [ ] get disk config:  
//!   layout profiler
//!   devices, ex: /dev/sda, /dev/sdb, etc
//!   partition config, ex: fat: size, mountpoint, startsize & maxsize, flags
//! [ ] get hostname
//! [ ] get kernel version
//! [ ] get keyboard layout
//! [ ] get nic (network interface)
//! [ ] get ntp
//! [ ] get packages
//! [ ] get timezone
//! [ ] get swap
//!

//! source:
//! - `lspci`


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

