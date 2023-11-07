use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    println!("{}", line.len());
    loop {
        let next_line = line
            .chars()
            .collect::<Vec<_>>()
            .chunks(2).fold(String::new(), |acc, chunk| {
                if let [a, b] = chunk {
                    if a.to_uppercase().to_string() == b.to_uppercase().to_string() && a != b {
                        return acc;
                    }
                    let mut next = acc.clone();
                    next.push(*a);
                    next.push(*b);
                    return next;
                }
                acc
            });
        let next_next_line = next_line
            .chars()
            .skip(1)
            .collect::<Vec<_>>()
            .chunks(2).fold(next_line[0..1], |acc, chunk| {
                if let [a, b] = chunk {
                    if a.to_uppercase().to_string() == b.to_uppercase().to_string() && a != b {
                        return acc;
                    }
                    let mut next = acc.clone();
                    next.push(*a);
                    next.push(*b);
                    return next;
                }
                acc
            });
        println!("{}", next_line);
        if line.len() == next_line.len() {
            panic!("{}", next_line.len());
        }
        line = next_line;
    }
}