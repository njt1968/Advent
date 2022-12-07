use crate::read_input;
use std::collections::VecDeque;

pub fn part_1() {
    let input = read_input("5_prod_crates.txt");
    let mut z = read_crates(input);
    let x = get_move_inputs("5_prod_instructions.txt");
    z.use_crane(x, true);
    println!("Part 1: {}", z.top_crates_string());
}

pub fn part_2() {
    let input = read_input("5_prod_crates.txt");
    let mut z = read_crates(input);
    let x = get_move_inputs("5_test_instructions.txt");
    z.use_crane(x, false);
    println!("Part 2: {}", z.top_crates_string());
}

fn read_crates(v: Vec<String>) -> Crates {
    let mut data = VecDeque::new();
    let mut columns = 0;
    for s in v {
        let x: Vec<_> = s.chars().skip(1).step_by(4).collect();
        if !x.contains(&'1') {
            data.push_back(x);
        } else {
            columns = x.len();
        }
    }
    Crates { data, columns }
}


pub fn get_move_inputs(file: &str) -> Vec<Instructions> {
    let input = read_input(file);
    let temp: Vec<i32> = input
        .iter()
        .flat_map(|s| {
            s.split_whitespace()
                .filter(|s| !s.contains("m") && !s.contains("o"))
                .map(|s| s.parse::<i32>().unwrap())
        })
        .collect();
    let chunks = temp.chunks(3);
    let output: Vec<Instructions> = chunks
        .map(|chunk| Instructions {
            amount: chunk[0],
            from: chunk[1],
            to: chunk[2],
        })
        .collect();

    output
}

pub struct Crates {
    data: VecDeque<Vec<char>>,
    columns: usize,
}

impl Crates {
    pub fn crate_move_9000(&mut self, amount: i32, from: i32, to: i32) {
        let from_column = (from - 1) as usize;
        let to_column = (to - 1) as usize;
        for _ in 0..amount {
            for row in &mut self.data {
                if let ' ' = row[from_column] {
                    continue;
                }
                let crate_char = row[from_column];
                row[from_column] = ' ';
                let last_empty = self.last_empty(to);
                match last_empty {
                    None => {
                        self.create_new_row();
                        self.data[0][to_column] = crate_char;
                    }
                    Some(row) => self.data[row][to_column] = crate_char,
                }
                break;
            }
        }
    }
    pub fn crate_move_9001(&mut self, amount: i32, from: i32, to: i32) {
        let from_column = (from - 1) as usize;
        let to_column = (to - 1) as usize;
        let mut to_move = vec![];
        for _ in 0..amount {
            for row in &mut self.data {
                if let ' ' = row[from_column] {
                    continue;
                }
                let crate_char = row[from_column];
                to_move.push(crate_char);
                row[from_column] = ' ';
                break;
            }
        }

        to_move.reverse();
        for c in to_move {
            let last_empty = self.last_empty(to);
            match last_empty {
                None => {
                    self.create_new_row();
                    self.data[0][to_column] = c;
                }
                Some(row) => self.data[row][to_column] = c,
            }
        }
    }

    pub fn create_new_row(&mut self) {
        let mut new_row = vec![];
        for _ in 0..self.columns {
            new_row.push(' ');
        }
        self.data.push_front(new_row);
    }

    pub fn use_crane(&mut self, instructions: Vec<Instructions>, single: bool) {
        if single {
            for set in instructions {
                self.crate_move_9000(set.amount, set.from, set.to);
            }
        } else {
            for set in instructions {
                self.crate_move_9001(set.amount, set.from, set.to);
            }
        }
    }
    pub fn top_crates(self) -> Vec<(char, usize)> {
        let mut found = vec![];
        let mut result = vec![];
        for row in &self.data {
            for (i, c) in row.iter().enumerate() {
                match c {
                    ' ' => (),
                    _ => {
                        if !found.contains(&i) {
                            result.push((*c, i + 1));
                            found.push(i)
                        }
                    }
                }
            }
        }
        result.sort_by(|a, b| a.1.cmp(&b.1));
        result
    }
    pub fn top_crates_string(self) -> String {
        let mut res = String::new();
        for (c, _) in self.top_crates() {
            res.push(c)
        }
        res
    }
    pub fn last_empty(&self, column: i32) -> Option<usize> {
        let j = (column - 1) as usize;
        let mut last_empty = None;
        for (row, crates) in self.data.iter().enumerate() {
            if crates[j] == ' ' {
                last_empty = Some(row);
            }
        }
        last_empty
    }
}

#[derive(Debug)]
pub struct Instructions {
    amount: i32,
    from: i32,
    to: i32,
}
