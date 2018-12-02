use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let f = File::open("input/1.txt").unwrap();
    let reader = BufReader::new(f);
    let (doubles, triples) = reader
        .lines()
        .map(|line| line.unwrap())
        .fold((0, 0), |(d, t), x| {
            let mut map = HashMap::new();
            for c in x.chars() {
                let counter = map.entry(c).or_insert(0);
                *counter += 1;
            }
            return (d + (if map.values().any(|&v| v == 2) { 1 } else { 0 }), t + (if map.values().any(|&v| v == 3) { 1 } else { 0 }));
        });
    println!("{}", doubles * triples);
}