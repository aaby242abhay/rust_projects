//adds strictsness to the codebase given we know something can only have certain values
//enums are a way to define a type by enumerating its possible values

use std::fmt::Display;

enum Direction {
    North,
    South,
    East,
    West
}

fn main(){
    moveAround(Direction :: North);
}

fn moveAround(direction : Direction){
    println!("Moving in the {:?} direction", direction);
}
