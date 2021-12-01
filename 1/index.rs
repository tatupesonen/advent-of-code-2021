use std::fs;

fn part1(input: &str) -> Option<i32> {
    let data = input.clone();
    let scans: Vec<Option<i32>> = data
        .split("\n")
        .map(|item| match item {
            "" => None,
            _ => Some(item.parse::<i32>().unwrap()),
        })
        .collect();

    let mut prev = scans[0].unwrap();
    let mut count = 0;
    for i in 1..scans.len() {
        if let Some(val) = scans[i] {
            if val > prev {
                count += 1;
            }
            prev = val;
        }
    }
    Some(count)
}

fn part2(input: &str) -> Option<usize> {
    let data = input.clone();
    let scans = data
        .split("\n")
        .filter_map(|item| match item {
            "" => None,
            _ => Some(item.parse::<i32>().unwrap()),
        })
        .collect::<Vec<i32>>();

    let result: usize = scans
        .windows(4)
        .filter(|&sub_arr| sub_arr[0] < sub_arr[3])
        .count();
    Some(result)
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
