use std::{clone, path::Path};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct OsRelease {
    pub name: String,
    pub pretty_name: String,
    pub id: String,
    pub build_id: String,
    pub ansi_color: String,
    pub home_url: String,
    pub documentation_url: String,
    pub support_url: String,
    pub bug_report_url: String,
    pub privacy_policy_url: String,
    pub logo: String,
}

pub fn os_release() -> dotenvy::Result<Map<String,Value>> {
    let foo = dotenvy::from_path_iter(Path::new("/etc/os-release"))?;

    let mut map = Map::new();

    for f in foo {
        let (key,val) = f?;
        map.insert(key.to_lowercase(), Value::String(val));
    }

    Ok(map)
}

impl OsRelease {
    pub fn from_etc() -> dotenvy::Result<Self> {
        let map = os_release()?;
        serde_json::from_value(Value::Object(map))
            .map_err(|e|dotenvy::Error::LineParse("".into(), e.line()))
    }
}

struct Dmi {
    handle: String,
    dmi_type: i32, // a number that have specification of what correspond to
    size: String,
}

fn dmidecode() -> std::io::Result<()> {
    use std::{io::{prelude::*, BufReader}, process::{Command, Stdio}};
    let mut cmd = Command::new("dmidecode")
        .stdout(Stdio::piped())
        .spawn()?;

    let Some(mut read) = cmd.stdout.take() else { unreachable!() };
    let mut reader = BufReader::new(&mut read);

    let mut buffer = String::new();
    let mut maps = Map::new();

    // Header
    loop {
        let n = reader.read_line(&mut buffer)?;
        if n <= 1 { break }
    }

    buffer.clear();

    loop {
        let n = reader.read_line(&mut buffer)?;
        if n == 0 { break }
        if n == 1 {
            buffer.clear();
            continue;
        }

        // new blocks
        let mut cols = buffer.split(", ");

        let dmi = Dmi {
            handle: cols.next().unwrap_or_default().replace("Handle ", ""),
            dmi_type: cols.next().unwrap_or_default().replace("DMI type ", "").parse::<i32>().unwrap_or(-1),
            size: cols.next().unwrap_or_default().into(),
        };

        todo!()
    }


    Ok(())
}

