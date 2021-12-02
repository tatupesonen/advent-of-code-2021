use std::fs;

#[derive(Debug)]
enum Directions {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn part1(input: &String) -> Option<String> {
    let mut input = input.clone();
    // Remove last line
    input.pop().unwrap();
    let directions = input
        .split('\n')
        .map(|e| {
            // Get the direction and the amount
            let split: Vec<&str> = e.split(" ").collect();
            let (direction, amount) = (split[0], split[1].parse::<i32>().unwrap());
            // test out these fancy enums
            match direction {
                "forward" => Directions::Forward(amount),
                "down" => Directions::Down(amount),
                "up" => Directions::Up(amount),
                _ => panic!("Invalid direction provided."),
            }
        })
        .collect::<Vec<Directions>>();

    let mut depth = 0;
    let mut pos = 0;
    directions
        .into_iter()
        .for_each(|direction| match direction {
            Directions::Down(val) => depth += val,
            Directions::Up(val) => depth -= val,
            Directions::Forward(val) => pos += val,
        });
    Some((depth * pos).to_string())
}

fn part2(input: &String) -> Option<String> {
    let mut input = input.clone();
    // Remove last line
    input.pop().unwrap();
    let directions = input
        .split('\n')
        .map(|e| {
            // Get the direction and the amount
            let split: Vec<&str> = e.split(" ").collect();
            let (direction, amount) = (split[0], split[1].parse::<i32>().unwrap());
            // test out these fancy enums
            match direction {
                "forward" => Directions::Forward(amount),
                "down" => Directions::Down(amount),
                "up" => Directions::Up(amount),

                _ => panic!("Invalid direction provided."),
            }
        })
        .collect::<Vec<Directions>>();

    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    directions.into_iter().for_each(|direction| {
        match direction {
            Directions::Down(val) => {
                aim += val;
            }
            Directions::Up(val) => {
                aim -= val;
            }
            Directions::Forward(val) => {
                pos += val;
                depth += aim * val;
            }
        }
        println!(
            "depth: {}, pos: {}, aim: {}, total: {}",
            depth,
            pos,
            aim,
            depth * pos
        );
    });
    Some((depth * pos).to_string())
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
