use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let mut reader = BufReader::new(f);

    let mut initial_state = String::new();
    reader.read_line(&mut initial_state);

    let initial = initial_state.trim_start_matches("initial state: ");
    let mut state: Vec<_> = initial.chars().map(|c| if c == '#' { true } else { false } ).collect();
    // Start at -200
    let start_point = -40;
    for _ in start_point..0 {
        state.insert(0, false);
    }
    for _ in 0..30 {
        state.push(false);
    }

    let mut _discard = String::new();
    reader.read_line(&mut _discard);

    let mut rules = HashMap::new();
    reader
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            let mut parts = line.split(" => ");
            let key = parts.next().unwrap();
            let mut key_num = 0;
            let mut index = 4;
            key.chars().for_each(|c| {
                if c == '#' {
                    key_num += i32::pow(2, index);
                }
                index -= 1;
            });
            let value = parts.next().unwrap();
            let value_bool = match value {
                "#" => true,
                "." => false,
                _ => panic!(),
            };
            rules.insert(key_num, value_bool);
        });
    // println!("{}", state.iter().fold("0: ".to_owned(), |acc, b| if *b == true { acc + "#" } else { acc + "." }));
    
    let mut start = Instant::now();
    for i in 0..(50_000_000_000 as i64) {
        let mut index = 0;
        let mut new_state: Vec<_> = state.windows(5).map(|window| {
            let mut j = 4;
            let key = window.iter().fold(0, |acc, prev_bool| {
                let mut next = acc;
                if *prev_bool == true {
                    next = acc + i32::pow(2, j);
                }
                j -= 1;
                next
            });
            index += 1;
            *rules.get(&key).unwrap()
        }).collect();
        let mut next_state = Vec::with_capacity(state.len());
        next_state.push(false);
        next_state.push(false);
        next_state.extend(new_state);
        next_state.push(false);
        next_state.push(false);
        state = next_state;
        if i % 1_000_000 == 0 {
            println!("{:?}", start.elapsed());
            println!("{}", i);
            start = Instant::now();
        }
        // println!("{}", state.iter().fold((i + 1).to_string() + ": ", |acc, b| if *b == true { acc + "#" } else { acc + "." }));
    }
    let mut index = start_point;
    let result = state.iter().fold(0, |acc, b| {
        let mut next = acc;
        if *b == true {
            next += index;
        }
        index += 1;
        next
    });
    println!("{}", result);
}