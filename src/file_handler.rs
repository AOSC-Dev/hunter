use std::{collections::HashMap, fs, io::Read};

use anyhow::{anyhow, Result};

use crate::{
    package_info::Package,
    parser::{all_packages, extract_all_names},
};

fn get_local_packages() -> Result<Vec<Package>> {
    let mut dpkg_status = std::fs::File::open("/var/lib/dpkg/status")?;
    let mut buf = Vec::new();
    dpkg_status.read_to_end(&mut buf)?;

    let packages = all_packages(&buf).map_err(|e| anyhow!("{}", e))?.1;

    let packages = packages
        .iter()
        .filter(|x| x.status == b"install ok installed");

    let mut results = vec![];

    for i in packages {
        results.push(Package {
            package: std::str::from_utf8(i.package)?.to_string(),
            version: std::str::from_utf8(i.version)?.to_string(),
            description: std::str::from_utf8(i.desc)?.to_string(),
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

        let packages = extract_all_names(&buf).map_err(|e| anyhow!("{}", e))?.1;

        for i in packages {
            result.insert(std::str::from_utf8(i)?.to_string(), 0);
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
