extern crate itertools;

use itertools::Itertools;
use Direction::*;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction { North, South, East, West }

fn main() {
    let directions = vec![North, North, South, East, West, West];

    let unique_directions = directions.iter()
        .unique()
        .collect::<Vec<_>>();
    println!("{:?}", unique_directions);
}