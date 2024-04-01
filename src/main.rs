use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    address: Address,
    phone_numbers: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Address {
    street: String,
    city: String,
    country: String,
}

fn main() {
    let json_config = "./test.json";
    let mut file = File::open(json_config).expect("File not found");

    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .expect("Unable to read file");

    let people: Vec<Person> =
        serde_json::from_str(&json_data).expect("JSON was not well-formatted");
    println!("{:#?}", people);
}
