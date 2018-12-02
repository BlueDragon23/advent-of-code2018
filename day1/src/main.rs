use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;

fn main() {
    let f = File::open("input/1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut set = HashSet::new();
    let numbers = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut acc: i32 = 0;
    let mut index = 0;
    loop {
        acc += numbers[index];
        index = (index + 1) % numbers.len();
        if set.contains(&acc) {
            println!("{}", acc);
            return;
        }
        set.insert(acc);
    }
}