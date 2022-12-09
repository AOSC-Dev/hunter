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

pub fn to_tabled(list: Vec<Package>) -> Result<Table> {
    let mut table = Table::new(list);

    table
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(|s: &str| format!(" {s} ")))
        .with(Style::psql());

    Ok(table)
}
