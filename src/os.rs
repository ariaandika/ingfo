use std::path::Path;
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


