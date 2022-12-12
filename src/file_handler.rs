use std::{collections::HashMap, fs, io::Read};

use anyhow::{anyhow, Result};
use eight_deep_parser::{Item, parse_multi};

use crate::package_info::Package;

fn get_local_packages() -> Result<Vec<Package>> {
    let mut dpkg_status = std::fs::File::open("/var/lib/dpkg/status")?;
    let mut buf = Vec::new();
    dpkg_status.read_to_end(&mut buf)?;

    let packages = eight_deep_parser::parse_multi(std::str::from_utf8(&buf)?)?;

    let packages = packages
        .iter()
        .filter(|x| x.get("Status") == Some(&Item::OneLine("install ok installed".to_string())));

    let mut results = vec![];

    for i in packages {
        let package = if let Item::OneLine(package) = i.get("Package").expect("Should have Package field") {
            package
        } else {
            return Err(anyhow!(""))
        };

        let version = if let Item::OneLine(version) = i.get("Version").expect("Should have Version field") {
            version
        } else {
            return Err(anyhow!(""))
        };

        let desc = if let Item::OneLine(desc) = i.get("Description").expect("Should have Desc field") {
            desc
        } else {
            return Err(anyhow!(""))
        };

        results.push(Package {
            package: package.to_owned(),
            version: version.to_owned(),
            description: desc.to_string(),
        })
    }

    Ok(results)
}

fn get_apt_mirror_packages() -> Result<HashMap<String, u8>> {
    let dir = fs::read_dir("/var/lib/apt/lists")?;
    let mut result = HashMap::new();

    for i in dir.flatten() {
        if !i
            .file_name()
            .to_str()
            .ok_or_else(|| anyhow!("Can not get filename str!"))?
            .ends_with("_Packages")
        {
            continue;
        }

        let mut f = std::fs::File::open(i.path())?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        let s = std::str::from_utf8(&buf)?;
        let packages = parse_multi(s)?;
        let packages = packages.iter().map(|x| x.get("Package"));

        for i in packages.flatten() {
            let package = if let Item::OneLine(package) = i {
                package
            } else {
                return Err(anyhow!(""))
            };

            result.insert(package.to_owned(), 0);
        }
    }

    Ok(result)
}

pub fn hunter() -> Result<Vec<Package>> {
    let mut result = vec![];
    let local_packages = get_local_packages()?;
    let installed_from_mirror = get_apt_mirror_packages()?;

    for i in local_packages {
        if installed_from_mirror.get(&i.package).is_none() {
            result.push(i);
        }
    }

    Ok(result)
}
