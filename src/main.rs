use std::collections::HashMap;
use Direction::*;

#[derive(Debug, PartialEq)]
enum Direction { North, South, East, West }

fn main() {
    let mut users_facing = HashMap::new();
    users_facing.insert("Alice", North);
    users_facing.insert("Bob", South);
    users_facing.insert("Carol", East);

    let users_not_facing_north = users_facing.iter()
        .filter(|&(_, d)| *d != North)
        .collect::<HashMap<_,_>>();
    println!("{:?}", users_not_facing_north);
}

