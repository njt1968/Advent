use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn calorie_counter() {
    let file = File::open("1_data.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut most, mut second_most, mut third_most) = (0, 0, 0);
    let mut temp = 0;
    for line in reader.lines() {
        match line {
            Ok(l) => match l.len() {
                0 => {
                    if temp > most {
						second_most = most;
						third_most = second_most;
                        most = temp;
                    } else if temp > second_most {
						third_most = second_most;
						second_most = temp;
					} else if temp > third_most {
						third_most = temp
					}
                    temp = 0
                }
                _ => {
                    let cals = l.parse::<u32>().unwrap();
                    temp += cals
                }
            },
            Err(_err) => (),
        }
    }
    println!("Result: \n1. {} calories.\n2. {} calories.\n3. {} calories", most, second_most, third_most);
	println!("\nSum: {} calories.", most + second_most + third_most);
}
