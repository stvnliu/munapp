use serde::Deserialize;
use std::fs::File;
use std::io::Read;
#[derive(Debug, Deserialize)]
#[serde()]
struct Country {
    name: String,
    ident: String,
}
#[derive(Debug, Deserialize)]
#[serde()]
struct CountryData {
    countries: Vec<Country>
}
fn read_data(path: &str) -> CountryData {
    let mut file = File::open("data.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let countries: CountryData = serde_json::from_str(&data)
        .expect("JSON was not well-formatted!");
    //println!("{:?}", countries);
    return countries;
}
fn main() {
    let data = read_data("./data.json");
    println!("{}",data.countries[0].name);
}
