use std::fs;

#[derive(Debug)]
struct Matrix {
    value: Vec<Vec<char>>,
}

impl Matrix {
    fn get(&self, vertical_index: usize, horizontal_index: usize) -> char {
        let horizontal_row = &self.value[vertical_index];
        let horizontal_row_length = horizontal_row.len();

        if horizontal_index > (horizontal_row_length - 1) {
            let overlap_count = horizontal_index / horizontal_row_length;
            let new_horizontal_index = horizontal_index - horizontal_row_length * overlap_count;

            return horizontal_row[new_horizontal_index];
        }

        horizontal_row[horizontal_index]
    }

    fn horizontal_rows_count(&self) -> usize {
        self.value.iter().count()
    }

    fn travel(&self, right_amount: usize, down_amount: usize) -> i32 {
        let mut tree_counter = 0;
        let mut start_horizontal_index = 0;

        for vertical in 1..self.horizontal_rows_count() {
            let vertical_index = vertical * down_amount;

            start_horizontal_index += right_amount;

            if vertical_index > self.horizontal_rows_count() {
                break;
            }

            if self.get(vertical_index, start_horizontal_index) == '#' {
                tree_counter += 1;
            }
        }

        tree_counter
    }
}

pub fn run() {
    println!("## Day 3");

    let file_path = "src/inputs/day3.txt";

    let matrix = Matrix {
        value: fs::read_to_string(file_path)
            .expect("Should have been able to read the file")
            .lines()
            .map(|x| x.chars().collect())
            .collect(),
    };

    //225
    println!("Part 1 answer is: {}", matrix.travel(3, 1));

    // 1115775000
    println!(
        "Part 2 answer is: {}",
        matrix.travel(1, 1)
            * matrix.travel(3, 1)
            * matrix.travel(5, 1)
            * matrix.travel(7, 1)
            * matrix.travel(1, 2)
    );
}
