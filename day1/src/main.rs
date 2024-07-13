use std::fs;

fn end_floor(data: &str) -> i32 {
    let mut floor = 0;
    for ch in data.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
    }
    floor
}

fn first_basement_position(data: &str) -> i32 {

    let mut floor = 0;
    let mut pos = 0;
    for ch in data.chars() {
        if ch == '(' {
            floor += 1;
        } else if ch == ')' {
            floor -= 1;
        }
        pos += 1;
        if floor < 0 {
            return pos;
        }
    }

    0
}

fn main() {
    let data: String = fs::read_to_string("./src/input.txt").unwrap();
    let mut result = end_floor(&data);
    println!("Floor last seen: {result:?}");

    result = first_basement_position(&data);
    println!("First basement position: {result:?}");
}
