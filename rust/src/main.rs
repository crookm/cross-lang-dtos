extern crate schemafy_core;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use text_io::read;

schemafy::schemafy!(
    root: MyDto
    "schemas/MyDto.json"
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Input your name:");
    let name_input: String = read!("{}\n");

    println!("Input your age:");
    let age_input: i32 = read!("{}\n");

    let my_dto = MyDto {
        name: Some(name_input),
        age: Some(age_input.into()),
    };

    println!();
    println!("{}", serde_json::to_string(&my_dto)?);
    Ok(())
}
