use std::io::Read;

use anyhow::{Context, Result};
use apt_sources_lists::{SourceEntry, SourceLine, SourcesLists};
use file_handler::hunter;
use package_info::Csv;
use xz2::bufread::XzDecoder;

use crate::package_info::to_tabled;

mod file_handler;
mod package_info;

fn main() {
    let list = hunter().unwrap();
    let csv = csv().unwrap();
    let mut res = vec![];

    if !list.is_empty() {
        for i in &csv {
            let index = list
                .iter()
                .position(|x| x.package == i.name && x.version == i.version);

            if let Some(index) = index {
                res.push(list[index].clone());
            }
        }
        println!(
            "Hunter v{} Detects retired and user-installed packages\n",
            env!("CARGO_PKG_VERSION")
        );

        if !res.is_empty() {
            println!("The following packages installed on your system has been retired from our source\ntree and are no longer supported by AOSC OS maintainers. These packages will not\nreceive any updates, enhancements, or fixes.\n\nWe strongly recommend that you remove these packages to prevent potential stability issues\nor security vulnerabilities.");
            let table1 = to_tabled(res).unwrap();
            println!("{}", table1);
        }

        println!(
            r#"The following packages installed on your system may be installed from a third-party
source. You may have installed them at some point manually.
        
This is a friendly reminder, for your reference.
"#
        );

        if !list.is_empty() {
            let table2 = to_tabled(list).unwrap();
            println!("{}", table2);
        }
    } else {
        println!(
            r#"Hunter v        Detects retired and user-installed packages

No package requires your attention at this moment.
        "#
        );
    }
}

fn csv() -> Result<Vec<Csv>> {
    let mut res = vec![];
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent("hunter")
        .build()?;

    let sources = get_sources()?;
    let source = sources
        .iter()
        .filter(|x| x.suite == "stable")
        .collect::<Vec<_>>();
    let source = source.first().take().context("mirror source is emoty!")?;
    let url = source.url();
    let url = format!("{url}/manifest/removed.csv.xz");

    let v = client
        .get(url)
        .send()?
        .error_for_status()?
        .bytes()?
        .to_vec();

    let mut decompressor = XzDecoder::new(&*v);
    let mut buf = String::new();
    decompressor.read_to_string(&mut buf)?;

    for i in buf.lines() {
        let mut i = i.split(',');
        let name = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let hash = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let size = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let arch = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let filename = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let version = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let repo = i.nth(0).take().context("Csv file is broken!")?.to_string();
        let retire_date = i.nth(0).take().context("Csv file is broken!")?.to_string();

        res.push(Csv {
            name,
            hash,
            size,
            arch,
            filename,
            version,
            repo,
            retire_date,
        })
    }

    Ok(res)
}

fn get_sources() -> Result<Vec<SourceEntry>> {
    let mut res = Vec::new();
    let list = SourcesLists::scan()?;

    for file in list.iter() {
        for i in &file.lines {
            if let SourceLine::Entry(entry) = i {
                res.push(entry.to_owned());
            }
        }
    }

    Ok(res)
}
