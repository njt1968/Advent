use crate::read_input;

pub fn part_1() {
    let inputs = read_input("3_prod.txt");
    let mut sum = 0;
    for rucksack in inputs {
        let rucksack_section_capacity = rucksack.len() / 2;
        let comp_1 = &rucksack[..rucksack_section_capacity];
        let comp_2 = &rucksack[rucksack_section_capacity..];
        let mut shared = 'z';
        for c in comp_1.chars() {
            if comp_2.contains(c) {
                shared = c;
            }
        }
        sum += char_to_value(shared);
    }
    println!("Total: {sum}");
}

pub fn part_2() {
    let inputs = read_input("3_prod.txt");
    let groups = groups_of_three(inputs);
    let mut badge = 'a';
    let mut result = 0;
    for group in groups {
        for c in group[0].chars() {
            if group[1].contains(c)  {
                if group[2].contains(c) {
                    badge = c
                }
            }
        }
        result += char_to_value(badge)
    }
    println!("Total: {result}")
}

pub fn groups_of_three(input: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = vec![];
    let mut count = 0;
    let mut temp : Vec<String> = vec![];
    for s in input {
        temp.push(s);
        count += 1;
        if count == 3 {
            result.push(temp);
            count = 0;
            temp = vec![];
        }
    }
    result
}


pub fn char_to_value(c: char) -> u32 {
    let letters = ('a'..='z').into_iter().chain(('A'..='Z').into_iter()).position(|x| x == c).map(|x| x + 1).unwrap() as u32;
    letters    
}
