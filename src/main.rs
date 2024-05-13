mod structs;

use std::fs::File;
use std::io::Read;
use cursive::CursiveRunnable;
use cursive::view::*;
use cursive::views::*;
use structs::CountryData;
fn show_clist(s: &mut CursiveRunnable, cdata: CountryData) {
    let countries = cdata.countries;
    let mut countries_view = ListView::new();
    for i in 0..countries.len() {
        countries_view.add_child(&countries[i].ident, TextView::new(&countries[i].name));
    }
    countries_view.add_child("", EditView::new().with_name("name").fixed_width(4));
    s.add_layer(OnEventView::new(Dialog::around(countries_view).title("Countries List")));
}
fn show_menu(s: &mut CursiveRunnable) {
    s.add_layer(Menubar::new());
}
fn tui(cdata: CountryData) {
    // Creates the cursive root - required for every application.
    let mut s = cursive::default();
    show_menu(&mut s);
    show_clist(&mut s, cdata);
    // Starts the event loop.
    s.run();
}
fn read_data(path: &str) -> CountryData {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let countries: CountryData = serde_json::from_str(&data)
        .expect("JSON was not well-formatted!");
    //println!("{:?}", countries);
    return countries;
}
fn main() {
    let data = read_data("./data.json");
    tui(data);
}
