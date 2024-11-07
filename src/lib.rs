use std::fs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct ApartmentParser {}

pub fn load_html(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Unsuccessful load file")
}