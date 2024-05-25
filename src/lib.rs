//! Collection of api to get device info
//!

//! TODO:
//! - get laptop brand
//! - get ram information
//! - get storage information
//! - get partition, flags, mountpoint, unallocated, filesystem
//! - get all installed os
//! - get battery percent
//! - get internet connection
//! - get bootloader
//! - get disk config:  
//!   layout profiler
//!   devices, ex: /dev/sda, /dev/sdb, etc
//!   partition config, ex: fat: size, mountpoint, startsize & maxsize, flags
//! - get hostname
//! - get kernel version
//! - get keyboard layout
//! - get nic (network interface)
//! - get ntp
//! - get packages
//! - get timezone
//! - get swap
//!

//! source:
//! - `lspci`

pub mod disk;

