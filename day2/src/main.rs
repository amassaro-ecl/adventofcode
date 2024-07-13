use std::fs::read_to_string;

struct Dimension {
    l: i32,
    w: i32,
    h: i32,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_sides(line: &str) -> Dimension {
    let mut iter = line.splitn(3, "x");
    let l = iter.next().unwrap().parse::<i32>().unwrap();
    let w = iter.next().unwrap().parse::<i32>().unwrap();
    let h = iter.next().unwrap().parse::<i32>().unwrap();

    Dimension { l, w, h }
}

fn get_sq_feet_needed(lines: &Vec<String>) -> i32 {

    let mut feet_needed = 0;

    for line in lines.iter() {
        let dim = get_sides(line);
        let side1 = dim.l * dim.w;
        let side2 = dim.w * dim.h;
        let side3 = dim.h * dim.l;
        let sides = vec![side1, side2, side3];
        let smallest = *sides.iter().min().unwrap();

        feet_needed += (2 * side1) + (2 * side2) + (2 * side3) + smallest;
    }
    feet_needed
}

fn calc_ribbon_for_box(dimension: Dimension) -> i32 {
    let mut sides = vec![dimension.h, dimension.w, dimension.l];
    sides.sort();

    let smallest = sides[0];
    let second_smallest = sides[1];

    smallest + smallest + second_smallest + second_smallest
}

fn calc_total_ribbon(lines: &Vec<String>) -> i32 {

    let mut ribbon_needed = 0;
    for line in lines.iter() {
        let sides = get_sides(line);
        let volume = sides.l * sides.w * sides.h;
        ribbon_needed += calc_ribbon_for_box(sides) + volume;
    }

    ribbon_needed
}

fn main() {
    let filename = "./src/input.txt";
    let lines = read_lines(filename);
    let feet_needed = get_sq_feet_needed(&lines);

    println!("Paper feet needed: {feet_needed:?}");

    let ribbon_needed = calc_total_ribbon(&lines);

    println!("Ribbon feet needed: {ribbon_needed:?}");
}
