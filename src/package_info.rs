use anyhow::{anyhow, Result};
use std::process::Command;
use tabled::object::Segment;
use tabled::{Alignment, Modify, Style, Table, Tabled};

use crate::parser::single_package;

#[derive(Debug, Tabled)]
pub struct Package {
    #[tabled(rename = "Package")]
    pub package: String,
    #[tabled(rename = "Version")]
    pub version: String,
    #[tabled(rename = "Description")]
    pub description: String,
}

fn parse_dpkg_info(buf: &[u8]) -> Result<Package> {
    let pk_infos = single_package(buf).map_err(|e| anyhow!("{}", e))?.1;

    let pk = get_value(&pk_infos, "Package")?;
    let ver = get_value(&pk_infos, "Version")?;
    let desc = get_value(&pk_infos, "Description")?;

    Ok(Package {
        package: pk,
        version: ver,
        description: desc,
    })
}

fn get_value<'a>(pk_infos: &[(&[u8], &[u8])], value: &str) -> Result<String> {
    let v = pk_infos
        .iter()
        .find(|(x, _)| x == &value.as_bytes())
        .map(|(_, y)| y)
        .ok_or_else(|| anyhow!("Can not get {:?} value {}", pk_infos, value))?;

    let v = std::str::from_utf8(v)?.to_string();

    Ok(v)
}

pub fn to_tabled(list: &[String]) -> Result<Table> {
    let mut table = vec![];
    for i in list {
        table.push(dpkg_info(i)?);
    }

    let mut table = Table::new(table);

    table
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(|s: &str| format!(" {s} ")))
        .with(Style::psql());

    Ok(table)
}

fn dpkg_info(pkgname: &str) -> Result<Package> {
    let cmd = Command::new("dpkg").arg("-s").arg(pkgname).output()?;

    if !cmd.status.success() {
        return Err(anyhow!(
            "Can not run dpkg -s {}\n\n Error:\n\n {}",
            pkgname,
            std::str::from_utf8(&cmd.stderr)?
        ));
    }

    let package = parse_dpkg_info(&cmd.stdout)?;

    Ok(package)
}
