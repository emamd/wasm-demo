extern crate serde_json;
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
enum Direction { North, South, East, West }

#[derive(Serialize, Deserialize, Debug)]
struct Example {
    favorite_animal: String,
    favorite_direction: Direction
}

fn main() {
    let data = r#" { "favorite_animal": "Bear", "favorite_direction": "North" } "#;
    let parsed: Example = serde_json::from_str(data).unwrap();
    println!("{:?}", parsed);
}