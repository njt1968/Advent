use crate::read_input;
const NUMS: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn part_1() {
    let cmds = read_input("data/7_prod.txt");
    let mut current_dir = Dir::new();
    let mut result = Dirs::new();
    for cmd in cmds {
        if cmd.contains("cd") && !cmd.contains("..") && !cmd.contains("/") && cmd.contains("$") {
            if current_dir.size_of_files > 0 {
                current_dir.name = match current_dir.name.is_empty() {
                    true => "/".to_owned(),
                    false => parse_cd_dir(&current_dir.name),
                };
                result.v.push(current_dir.clone());
                current_dir.reset();
            }
            current_dir.size_of_files = 0;
            current_dir.name = cmd;
        } else if !cmd.contains("$") && !cmd.contains("dir") {
            let x = parse_file_size(&cmd);
            current_dir.size_of_files += x;
        } else if cmd.contains("dir") {
            current_dir.contains.push(parse_dir(&cmd))
        }
    }
    let end = result.calc_totals();
    let mut puzzle_solution = 0;
    for (name, size) in end {
        if size <= 100000 {
            println!("{} {}", name, size)
            // puzzle_solution += size
        }
    }
    // println!("{puzzle_solution}")
}

pub fn parse_file_size(s: &str) -> i32 {
    let size: i32 = s.split_whitespace().nth(0).unwrap().parse().unwrap();
    size
}

pub fn parse_cd_dir(s: &str) -> String {
    let dir_name: String = s.split_whitespace().nth(2).unwrap().to_owned();
    dir_name
}

pub fn parse_dir(s: &str) -> String {
    let dir_name: String = s.split_whitespace().nth(1).unwrap().to_owned();
    dir_name
}

#[derive(Debug)]
pub struct Dir {
    name: String,
    size_of_files: i32,
    contains: Vec<String>,
}

impl Dir {
    pub fn new() -> Dir {
        Dir {
            name: "".to_owned(),
            size_of_files: 0,
            contains: vec![],
        }
    }
    pub fn reset(&mut self) {
        self.name = "".to_owned();
        self.size_of_files = 0;
        self.contains = vec![];
    }
    pub fn clone(&mut self) -> Dir {
        Dir {
            name: self.name.clone(),
            size_of_files: self.size_of_files,
            contains: self.contains.clone(),
        }
    }
}
#[derive(Debug)]
pub struct Dirs {
    v: Vec<Dir>,
}

impl Dirs {
    pub fn new() -> Dirs {
        Dirs { v: vec![] }
    }
    pub fn calc_totals(self) -> Vec<(String, i32)> {
        let mut total = 0;
        let mut res = vec![];
        for dir in &self.v {
            total += dir.size_of_files;
            for name in &dir.contains {
                for d in &self.v {
                    if &d.name == name {
                        total += d.size_of_files
                    }
                }
            }
            res.push((dir.name.clone(), total));
            total = 0;
        }
        res
    }
}
