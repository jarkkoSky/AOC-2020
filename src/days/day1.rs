use itertools::Itertools;
use std::fs;

// 128397680
fn part_2(numbers: &Vec<i32>) {
    numbers
        .iter()
        .tuple_combinations()
        .find(|&(x, y, z)| x + y + z == 2020)
        .map(|(x, y, z)| x * y * z)
        .map(|res| println!("Part 2 answer is: {}", res));
}

// 605364
fn part_1(numbers: &Vec<i32>) {
    for v in numbers {
        numbers
            .iter()
            .find(|&&x| x + v == 2020)
            .map(|other_number| println!("Part 1 Answer is {}", other_number * v));
    }
}

pub fn run() {
    println!("## Day 1");

    let file_path = "src/inputs/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let numbers: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();

    part_1(&numbers);
    part_2(&numbers);
}
