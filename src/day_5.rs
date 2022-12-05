use crate::read_input;
use std::collections::VecDeque;

pub fn part_1() {
    let input = read_input("5_prod_crates.txt");
    let mut z = read_crates(input);
    let x = get_move_inputs("5_prod_instructions.txt");
    z.use_crane(x);
    dbg!(z.top_crates_string());
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

pub fn last_empty(v: &VecDeque<Vec<char>>, column: i32) -> Option<usize> {
    let j = (column - 1) as usize;
    let mut last_empty = None;
    for (row, crates) in v.into_iter().enumerate() {
        if crates[j] == ' ' {
            last_empty = Some(row);
        }
    }
    last_empty
}

pub fn get_move_inputs(file: &str) -> Vec<Vec<i32>> {
    let mut output = vec![];
    let input = read_input(file);
    let input: Vec<i32> = input
        .iter()
        .flat_map(|s| {
            s.split_whitespace()
                .filter(|s| !s.contains("m") && !s.contains("o"))
                .map(|s| s.parse::<i32>().unwrap())
        })
        .collect();
    let mut count = 0;
    let mut temp = vec![];
    for num in input {
        temp.push(num);
        count += 1;
        match count % 3 == 0 {
            true => {
                output.push(temp);
                temp = vec![];
            }
            _ => (),
        };
    }
    output
}

pub struct Crates {
    data: VecDeque<Vec<char>>,
    columns: usize,
}

impl Crates {
    pub fn crate_move_9000(&mut self, amount: i32, from: i32, to: i32) {
        let i = (from - 1) as usize;
        let j = (to - 1) as usize;
        for _ in 0..amount {
            for row in &mut self.data {
                if let ' ' = row[i] {
                    continue;
                }
                let x = row[i];
                row[i] = ' ';
                let last_empty = last_empty(&self.data, to);
                match last_empty {
                    None => {
                        self.create_new_row();
                        self.data[0][j] = x;
                    }
                    Some(row) => self.data[row][j] = x,
                }
                break;
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

    pub fn use_crane(&mut self, instructions: Vec<Vec<i32>>) {
        for set in instructions {
            self.crate_move_9000(set[0], set[1], set[2]);
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
}
