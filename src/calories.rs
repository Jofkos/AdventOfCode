use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Scoreboard<const S: usize> {
    scoreboard: [i64; S]
}

impl<const S: usize> Scoreboard<S> {
    fn new() -> Self {
        return Scoreboard { scoreboard: [0; S] };
    }

    fn add_potentially(&mut self, new_val: i64) {
        let mut target_index = S;
        if new_val > self.scoreboard[0] {
            target_index = 0;
        } else if new_val < self.scoreboard[0] {
            for (index, _) in self.scoreboard.iter().enumerate().rev() {
                if new_val < self.scoreboard[index] {
                    if index + 1 < S {
                        target_index = index + 1;
                    }
                    break;
                }
            }
        }

        if target_index < S {
            for index in (target_index .. S - 1).rev() {
                self.scoreboard[index + 1] = self.scoreboard[index]
            }

            self.scoreboard[target_index] = new_val;
        }
    }
}

pub fn solve_calories<const S: usize>(day_n: i64) {
    let filename = "input/calories_per_elf.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut scoreboard = Scoreboard::<S>::new();

    let mut current_count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            scoreboard.add_potentially(current_count);
            current_count = 0;
        } else {
            current_count += line.parse::<i64>().unwrap();
        }
    }
    println!("[Day {}] Sum top {}: {}", day_n, scoreboard.scoreboard.len(), scoreboard.scoreboard.iter().sum::<i64>());
    println!("[Day {}] The scoreboard: {:#?}", day_n, scoreboard.scoreboard);
}