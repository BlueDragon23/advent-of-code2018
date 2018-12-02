use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input/1.txt").unwrap();
    let reader = BufReader::new(f);
    println!("{}", reader
        .lines()
        .map(|line| line.unwrap())
        .fold(0, |acc, x| acc + x.parse::<i32>().unwrap())
    );
}