use std::{collections::HashMap, io::Read, process::Command, fs};

use anyhow::{anyhow, Result};

fn get_local_packages() -> Result<Vec<String>> {
    let cmd = Command::new("apt-mark").arg("showmanual").output()?;

    if !cmd.status.success() {
        return Err(anyhow!("Can not run apt-mark showmanual!"));
    }

    let stdout_manual = std::str::from_utf8(&cmd.stdout)?
        .split('\n')
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    let cmd = Command::new("apt-mark").arg("showauto").output()?;

    if !cmd.status.success() {
        return Err(anyhow!("Can not run apt-mark showauto!"));
    }

    let stdout_auto = std::str::from_utf8(&cmd.stdout)?
        .split('\n')
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();

    Ok([stdout_manual, stdout_auto].concat())
}

fn get_apt_mirror_packages() -> Result<HashMap<String, u8>> {
    let dir = fs::read_dir("/var/lib/apt/lists")?;
    let mut result = HashMap::new();

    for i in dir.flatten() {
        if !i.file_name().to_str().ok_or_else(|| anyhow!("Can not get filename str!"))?.ends_with("_Packages") {
            continue;
        }

        let mut f = std::fs::File::open(i.path())?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
    
        let packages = s
            .split('\n')
            .filter(|x| x.starts_with("Package: "))
            .map(|x| x.replace("Package: ", ""));

        for i in packages {
            result.insert(i, 0);
        }
    }
    

    Ok(result)
}

pub fn hunter() -> Result<Vec<String>> {
    let mut result = vec![];
    let local_packages = get_local_packages()?;
    let installed_from_mirror = get_apt_mirror_packages()?;

    for i in local_packages {
        if installed_from_mirror.get(&i).is_none() {
            result.push(i);
        }
    }

    Ok(result)
}
