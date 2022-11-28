use anyhow::{anyhow, Result};
use std::process::Command;

#[derive(Debug)]
pub struct Package {
    pub package: String,
    pub version: String,
    pub description: String,
}

pub fn parse_dpkg_info(s: &str) -> Package {
    let package_vec = s.split('\n').map(|x| x.to_string()).collect::<Vec<_>>();

    let package = get_value(&package_vec, "Package");

    let version = get_value(&package_vec, "Version");
    let desc = get_value(&package_vec, "Description");

    Package {
        package: package.to_string(),
        version: version.to_string(),
        description: desc.to_string(),
    }
}

fn get_value<'a>(package_vec: &'a [String], value: &'a str) -> &'a str {
    let index = package_vec
        .iter()
        .position(|x| x.contains(&format!("{}: ", value)))
        .unwrap();
    let result = package_vec[index]
        .strip_prefix(&format!("{}: ", value))
        .unwrap();

    result
}

pub fn dpkg_info(pkgname: &str) -> Result<Package> {
    let cmd = Command::new("dpkg").arg("-s").arg(pkgname).output()?;

    if !cmd.status.success() {
        return Err(anyhow!(
            "Can not run dpkg -s {}\n\n Error:\n\n {}",
            pkgname,
            std::str::from_utf8(&cmd.stderr)?
        ));
    }

    let package = parse_dpkg_info(std::str::from_utf8(&cmd.stdout)?);

    Ok(package)
}
