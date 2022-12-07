use std::{collections::HashMap, fs, io::Read, process::Command};

use anyhow::{anyhow, Result};

use crate::parser::extract_all_names;

fn get_local_packages() -> Result<Vec<String>> {
    let cmd = Command::new("dpkg-query")
        .arg("-W")
        .arg("-f=${Package}\n")
        .output()?;

    if !cmd.status.success() {
        return Err(anyhow!(
            "Run dpkg-query failed!\n\nError:\n\n{}",
            String::from_utf8_lossy(&cmd.stderr)
        ));
    }

    let results = std::str::from_utf8(&cmd.stdout)?
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();

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
