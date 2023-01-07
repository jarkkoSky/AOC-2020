use std::fs;

fn get_char_at(
    matrix: &Vec<Vec<char>>,
    vertical_index: usize,
    horizontal_index: usize,
) -> Option<&char> {
    let horizontal_row = matrix.get(vertical_index)?;

    let horizontal_row_length = horizontal_row.len();
    let overlap_count = horizontal_index / horizontal_row_length;

    horizontal_row.get(horizontal_index - horizontal_row_length * overlap_count)
}

fn travel(matrix: &Vec<Vec<char>>, right_amount: usize, down_amount: usize) -> i32 {
    let mut tree_counter = 0;
    let mut start_horizontal_index = 0;

    for vertical in 1..matrix.iter().count() {
        start_horizontal_index += right_amount;

        if let Some(&ch) = get_char_at(matrix, vertical * down_amount, start_horizontal_index) {
            if ch == '#' {
                tree_counter += 1;
            }
        }
    }

    tree_counter
}

pub fn run() {
    println!("## Day 3");

    let file_path = "src/inputs/day3.txt";

    let matrix: Vec<Vec<char>> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    //225
    println!("Part 1 answer is: {}", travel(&matrix, 3, 1));

    // 1115775000
    println!(
        "Part 2 answer is: {}",
        travel(&matrix, 1, 1)
            * travel(&matrix, 3, 1)
            * travel(&matrix, 5, 1)
            * travel(&matrix, 7, 1)
            * travel(&matrix, 1, 2)
    );
}
