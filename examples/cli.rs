

pub fn main() {
    println!("{:#?}",ingfo::os::os_release());
    println!("{:#?}",ingfo::os::OsRelease::from_etc());
}


