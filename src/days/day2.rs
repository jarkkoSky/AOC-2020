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
    let a = input.split_ascii_whitespace().collect::<Vec<&str>>();

    let letter = a.get(1).unwrap().replace(":", "").chars().next().unwrap();
    let password: String = a.get(2).unwrap().to_string();

    let min_and_max: Vec<&str> = a.get(0).unwrap().split('-').collect();

    let min: usize = min_and_max.get(0).unwrap().parse().unwrap();
    let max: usize = min_and_max.get(1).unwrap().parse().unwrap();

    Input {
        min,
        max,
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

    println!(
        "Part 1 answer is: {}",
        inputs.iter().filter(|x| x.is_valid_part1()).count(),
    );

    println!(
        "Part 2 answer is: {}",
        inputs.iter().filter(|x| x.is_valid_part2()).count(),
    );
}
