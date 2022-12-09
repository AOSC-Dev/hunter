use anyhow::Result;
use tabled::object::Segment;
use tabled::{Alignment, Modify, Style, Table, Tabled};

#[derive(Debug, Tabled)]
pub struct Package {
    #[tabled(rename = "Package")]
    pub package: String,
    #[tabled(rename = "Version")]
    pub version: String,
    #[tabled(rename = "Description")]
    pub description: String,
}

// pub fn pretty_packages(packages: Vec<Vec<(&[u8], &[u8])>>) -> Result<Vec<Package>> {
//     let mut results = vec![];

//     for p in packages {
//         let (mut package, mut version, mut desc) = (None, None, None);
//         for i in p {
//             if i.0 == b"Package" {
//                 package = Some(i.1);
//             }

//             if i.0 == b"Version" {
//                 version = Some(i.1);
//             }
    
//             if i.0 == b"Description" {
//                 desc = Some(i.1);
//             }
//         }

//         // if package.and(version).and(desc).is_none() {
//         //     return Err(anyhow!("Can not parse package: {}", pkgname));
//         // }

//         results.push(Package {
//             package: std::str::from_utf8(package.unwrap())?.to_string(),
//             version: std::str::from_utf8(version.unwrap())?.to_string(),
//             description: std::str::from_utf8(desc.unwrap())?.to_string(),
//         });
//     }

//     Ok(results)
// }

pub fn to_tabled(list: Vec<Package>) -> Result<Table> {
    let mut table = Table::new(list);

    table
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(|s: &str| format!(" {s} ")))
        .with(Style::psql());

    Ok(table)
}

// fn dpkg_info(pkgname: &str, packages: Vec<Vec<(&[u8], &[u8])>>) -> Result<Package> {
//     let mut info = None;

//     for i in packages {
//         if i.iter().find(|(x, _)| x == b"Package").map(|x| x.1) == Some(pkgname.as_bytes()) {
//             info = Some(i);
//             break;
//         }
//     }

//     if info.is_none() {
//         return Err(anyhow!(
//             "Could not get package {} in /var/lib/dpkg/status",
//             pkgname
//         ));
//     }

//     let info = info.unwrap();

//     let (mut package, mut version, mut desc) = (None, None, None);

//     for i in info {
//         if i.0 == b"Package" {
//             package = Some(i.1);
//         }

//         if i.0 == b"Version" {
//             version = Some(i.1);
//         }

//         if i.0 == b"Description" {
//             desc = Some(i.1);
//         }
//     }

//     if package.and(version).and(desc).is_none() {
//         return Err(anyhow!("Can not parse package: {}", pkgname));
//     }

//     let package = Package {
//         package: std::str::from_utf8(package.unwrap())?.to_string(),
//         version: std::str::from_utf8(version.unwrap())?.to_string(),
//         description: std::str::from_utf8(desc.unwrap())?.to_string(),
//     };

//     Ok(package)
// }
