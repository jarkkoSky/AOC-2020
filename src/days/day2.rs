use std::fs;

#[derive(Debug)]
struct Input {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Input {
    fn is_valid_part1(&self) -> bool {
        let count = self.password.chars().filter(|&x| x == self.letter).count();

        if count >= self.min && count <= self.max {
            return true;
        }

        false
    }

    fn is_valid_part2(&self) -> bool {
        let fst_letter = self.password.chars().nth(self.min - 1).unwrap();
        let snd_letter = self.password.chars().nth(self.max - 1).unwrap();

        if (fst_letter == self.letter || snd_letter == self.letter) && fst_letter != snd_letter {
            return true;
        }

        false
    }
}

fn parse_input(input: &str) -> Input {
    let split_input = input.split_ascii_whitespace().collect::<Vec<&str>>();

    let min_and_max: Vec<usize> = split_input[0]
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect();

    let letter = split_input[1].replace(":", "").chars().next().unwrap();
    let password: String = split_input[2].to_string();

    Input {
        min: min_and_max[0],
        max: min_and_max[1],
        letter,
        password,
    }
}

pub fn run() {
    println!("## Day 2");

    let file_path = "src/inputs/day2.txt";
    let inputs: Vec<Input> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(parse_input)
        .collect();

    // 477
    println!(
        "Part 1 answer is: {}",
        inputs.iter().filter(|x| x.is_valid_part1()).count(),
    );

    // 686
    println!(
        "Part 2 answer is: {}",
        inputs.iter().filter(|x| x.is_valid_part2()).count(),
    );
}
