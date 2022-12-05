use crate::read_input;

pub fn part_1() {
    let mut result = 0;
    let elf_pairs = read_input("4_prod.txt");

    // Parse input string into Vec<(i32, i32)>
    let parsed: Vec<Vec<(i32, i32)>> = elf_pairs
        .iter()
        .map(|s| {
            // Split input string on "," and parse into Vec<&str>
            let parts: Vec<&str> = s.split(",").collect();
            // Parse each element of Vec<&str> into a tuple of (i32, i32)
            parts
                .iter()
                .map(|s| {
                    let range: Vec<&str> = s.split("-").collect();
                    (range[0].parse().unwrap(), range[1].parse().unwrap())
                })
                .collect()
        })
        .collect();

    // Iterate over elf_pairs and parsed input simultaneously and check if elf range overlap
    for inner in parsed {
        let (elf_1_start, elf_1_finish) = (inner[0].0, inner[0].1);
        let (elf_2_start, elf_2_finish) = (inner[1].0, inner[1].1);
        if (elf_1_start <= elf_2_start && elf_1_finish >= elf_2_finish)
            || (elf_2_start <= elf_1_start && elf_2_finish >= elf_1_finish)
        {
            result += 1;
        }
    }
    println!("{}", result);
}

pub fn part_2() {
    let mut result = 0;
    let elf_pairs = read_input("4_prod.txt");

    // Parse input string into Vec<(i32, i32)>
    let parsed: Vec<Vec<(i32, i32)>> = elf_pairs
        .iter()
        .map(|s| {
            // Split input string on "," and parse into Vec<&str>
            let parts: Vec<&str> = s.split(",").collect();
            // Parse each element of Vec<&str> into a tuple of (i32, i32)
            parts
                .iter()
                .map(|s| {
                    let range: Vec<&str> = s.split("-").collect();
                    (range[0].parse().unwrap(), range[1].parse().unwrap())
                })
                .collect()
        })
        .collect();

    // Iterate parsed input and check if elf ranges overlap
    for inner in parsed {
        let (elf_1_start, elf_1_finish) = (inner[0].0, inner[0].1);
        let (elf_2_start, elf_2_finish) = (inner[1].0, inner[1].1);
        if elf_1_start == elf_2_start
            || elf_1_start == elf_2_finish
            || elf_2_start == elf_1_finish
            || elf_2_finish == elf_1_finish
            || elf_1_start < elf_2_start && elf_1_finish > elf_2_start
            || elf_2_start < elf_1_start && elf_2_finish > elf_1_start
        {
            result += 1
        }
    }
    println!("{}", result);
}

