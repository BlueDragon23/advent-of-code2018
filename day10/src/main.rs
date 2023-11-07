use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::cmp::{min, max};
extern crate regex;
use regex::Regex;

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let reader = BufReader::new(f);
    let re = Regex::new(r"position=<([- ]\d+), ([- ]\d+)> velocity=<([- ]\d+), ([- ]\d+)>").unwrap();
    let tuples = reader.lines()
        .map(|x| x.unwrap())
        .map(|line| {
            re.captures_iter(&line)
            .map(|x| (x[1].trim().parse::<i64>().unwrap(), x[2].trim().parse::<i64>().unwrap(), x[3].trim().parse::<i64>().unwrap(), x[4].trim().parse::<i64>().unwrap())).collect::<Vec<_>>()[0]
        })
        .collect::<Vec<_>>();
    let mut min_area = 100_000_000;
    let mut min_time = 0;
    let mut min_tuple = (0, 0, 0, 0);
    for i in 1..100000 {

        let (f_min_x, f_min_y, f_max_x, f_max_y) = tuples.iter().fold((1_000_000, 1_000_000, -1_000_000, -1_000_000), |(min_x, min_y, max_x, max_y), (sx, sy, vx, vy)| {
            let x = sx + vx * i;
            let y = sy + vy * i;
            (min(x, min_x), min(y, min_y), max(x, max_x), max(y, max_y))
        });
        let area = (f_max_x - f_min_x) * (f_max_y - f_min_y);
        if area < min_area {
            min_time = i;
            min_area = area;
            min_tuple = (f_min_x, f_min_y, f_max_x, f_max_y);
            //println!("Shrunk to: {}, {}, {:?}", min_area, min_time, min_tuple);
        }
    }
    if min_time == 0 {
        panic!("We didn't increment");
    }
    println!("{}, {}, {:?}", min_area, min_time, min_tuple);
    let (min_x, min_y, max_x, max_y) = min_tuple;
    let width = max_x - min_x;
    let height = max_y - min_y;
    let mut result = vec!['.'; (height * width + width) as usize];
    // let height = max_y - min_y;
    tuples.iter().for_each(|(sx, sy, vx, vy)| {
        let x = sx + vx * min_time;
        let y = sy + vy * min_time;
        result[((y - min_y) * width + (x - min_x)) as usize] = '#';
    });
    for line in result.chunks(width as usize) {
        println!("{:?}", line);
    }
}