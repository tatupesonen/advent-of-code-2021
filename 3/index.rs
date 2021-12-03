use std::collections::HashMap;
use std::fs;

fn part1(input: &String) -> Option<String> {
    let data = input.clone();

    let bits_in_line = data.lines().collect::<Vec<&str>>()[0].chars().count();

    let no_newlines = data.lines();
    let mut hm: HashMap<String, i32> = HashMap::new();
    no_newlines.for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            let key = create_key(idx, c);
            match hm.get_mut(&key) {
                Some(v) => *v += 1,
                None => {
                    hm.insert(key, 1);
                }
            }
        });
    });

    let mut gamma_rate: Vec<i32> = Vec::new();
    let mut epsilon_rate: Vec<i32> = Vec::new();

    for i in 0..bits_in_line {
        let pair = get_pair(&hm, i);
		println!("{:?}", pair);
        match pair {
            (zeros, ones) if ones > zeros => {
                // it's a gamma rate and we don't inverse
                gamma_rate.push(1);
                epsilon_rate.push(0);
            }
            (zeros, ones) if zeros > ones => {
                gamma_rate.push(0);
                epsilon_rate.push(1);
            }
            _ => panic!("Wtf"),
        }
    }
    println!(
        "Gamma rate: {}",
        gamma_rate
            .iter()
            .map(|&e| e.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    println!(
        "Epsilon rate: {}",
        epsilon_rate
            .iter()
            .map(|&e| e.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    None
}

fn get_pair(hm: &HashMap<String, i32>, index: usize) -> (&i32, &i32) {
    let zeros = hm.get(&create_key(index, '0')).unwrap();
    let ones = hm.get(&create_key(index, '1')).unwrap();
    (zeros, ones)
}

fn create_key(idx: usize, c: char) -> String {
    format!("{}-{}", idx, c)
}

fn part2(input: &String) -> Option<String> {
    let data = input.clone();

    let bits_in_line = data.lines().collect::<Vec<&str>>()[0].chars().count();

    let no_newlines = data.lines();
    let mut hm: HashMap<String, i32> = HashMap::new();
    no_newlines.for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            let key = create_key(idx, c);
            match hm.get_mut(&key) {
                Some(v) => *v += 1,
                None => {
                    hm.insert(key, 1);
                }
            }
        });
    });

    no_newlines.clone().for_each(|line| {
        line.chars().enumerate().for_each(|(idx, c)| {
            let key = create_key(idx, c);
            match hm.get_mut(&key) {
                Some(v) => *v += 1,
                None => {
                    hm.insert(key, 1);
                }
            }
        });
    });

    // for i in 0..bits_in_line {
    //     let pair = get_pair(&hm, i);
	// 	// Consider only the first bit
    //     match pair {
    //         (zeros, ones) if ones > zeros => {

    //         }
    //         (zeros, ones) if zeros > ones => {
    //         }
    //         _ => panic!("Wtf"),
    //     }
    // }
    None
}

// This code is here just to run your parts and to read input.
fn main() {
    let input = read_input();
    let output1 = part1(&input);
    let output2 = part2(&input);
    match output1 {
        Some(output) => println!("Solution 1: {}", output),
        None => println!("Part 1 doesn't return anything."),
    }
    match output2 {
        Some(output) => println!("Solution 2: {}", output),
        None => println!("Part 2 doesn't return anything."),
    }
}

fn read_input() -> String {
    let filename = "input.txt";
    fs::read_to_string(filename).expect("Couldn't read the input file.")
}
