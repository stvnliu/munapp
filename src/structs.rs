use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde()]
pub struct Country {
    pub name: String,
    pub ident: String,
}
#[derive(Debug, Deserialize)]
#[serde()]
pub struct CountryData {
    pub countries: Vec<Country>
}
