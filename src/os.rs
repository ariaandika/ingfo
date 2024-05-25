#[derive(Debug)]
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

pub fn os_release() -> dotenvy::Result<std::collections::HashMap<String,String>> {
    let foo = dotenvy::from_path_iter(std::path::Path::new("/etc/os-release"))?;

    let mut hash = std::collections::HashMap::new();

    for f in foo {
        let (key,val) = f?;
        hash.insert(key,val);
    }

    Ok(hash)
}

impl OsRelease {
    pub fn from_etc() -> dotenvy::Result<Self> {
        use dotenvy::Error::EnvVar;
        use std::env::VarError::NotPresent;
        let foo = os_release()?;
        let slef = Self {
            name: foo.get("NAME".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            pretty_name: foo.get("PRETTY_NAME".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            id: foo.get("ID".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            build_id: foo.get("BUILD_ID".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            ansi_color: foo.get("ANSI_COLOR".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            home_url: foo.get("HOME_URL".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            documentation_url: foo.get("DOCUMENTATION_URL".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            support_url: foo.get("SUPPORT_URL".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            bug_report_url: foo.get("BUG_REPORT_URL".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            privacy_policy_url: foo.get("PRIVACY_POLICY_URL".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
            logo: foo.get("LOGO".into()).take().ok_or(EnvVar(NotPresent))?.to_owned(),
        };
        Ok(slef)
    }
}


