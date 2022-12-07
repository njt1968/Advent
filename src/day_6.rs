use std::{
    fs::File,
    io::{BufReader, Read},
};

fn get_inp(f: &str) -> String {
    let file = File::open(f).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();
    buf
}

pub fn part_1() {
    let input = get_inp("data/6_prod.txt");
    println!("{:?}", input.len());
    dbg!(find_marker(&input, 14));
}

// pub fn find_marker(s: &str) -> usize {
// 	let x: Vec<_> =s.chars().enumerate().collect();
// 	let z: Vec<_> = s.chars().collect();
// 	for (i, c) in x {
// 		if c != z[i+1] && c != z[i+2] && c != z[i+3] {
// 			if z[i+1] != z[i+2] && z[i+1] != z[i+3] {
// 				if z[i+2] != z[i+3] {
// 					return i+4;
// 				}
// 			}
// 		}
// 	}

// 	0 as usize
// }

pub fn find_marker(s: &str, marker_size: usize) -> usize {
    let chars: Vec<_> = s.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        let marker_chars: Vec<_> = chars[i+1..i+marker_size].iter().collect();
        if !marker_chars.contains(&c) {
            if marker_chars.iter().all(|&x| marker_chars.iter().filter(|&&y| y == x).count() == 1) {
                return i+marker_size;
            }
        }
    }
    0 as usize
}
