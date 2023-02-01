use std::{collections::HashMap, fs, io::Read};

use anyhow::{anyhow, Context, Result};

use crate::package_info::Package;

fn get_local_packages() -> Result<Vec<Package>> {
    let mut dpkg_status = std::fs::File::open("/var/lib/dpkg/status")?;
    let mut buf = String::new();
    dpkg_status.read_to_string(&mut buf)?;

    let packages = debcontrol::parse_str(&buf).map_err(|e| anyhow!("{}", e))?;

    let packages = packages.into_iter().map(|x| x.fields).filter(|x| {
        let res = x.iter().find(|x| x.name == "Status");
        if let Some(res) = res {
            res.value == "install ok installed"
        } else {
            false
        }
    });

    let mut results = vec![];

    for i in packages {
        let mut i_iter = i.into_iter();
        let package = i_iter
            .find(|x| x.name == "Package")
            .take()
            .context("hould have Package field")?
            .value;
        let version = i_iter
            .find(|x| x.name == "Version")
            .take()
            .context("Should have Version field")?
            .value;
        let desc = i_iter
            .find(|x| x.name == "Description")
            .take()
            .context("Should have Version field")?
            .value;

        results.push(Package {
            package,
            version,
            description: desc,
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
        let packages = debcontrol::parse_str(s).map_err(|x| anyhow!("{}", x))?;
        let packages = packages.into_iter().map(|x| x.fields);

        for i in packages {
            if let Some(f) = i.iter().find(|x| x.name == "Package") {
                result.insert(f.value.to_owned(), 0);
            }
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
