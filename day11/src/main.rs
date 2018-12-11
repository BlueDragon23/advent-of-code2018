use std::cmp::{min, max};

fn main() {
    let input = 7165;
    let mut grid: Vec<(i32, i32)> = vec![(0, 0); 300 * 300];
    for x in 0..300 {
        for y in 0..300 {
            grid[y * 300 + x] = ((x + 1) as i32, (y + 1) as i32);
        }
    }
    let power: Vec<_> = grid.iter().map(|(x, y)| {
        let id = (x + 10) as i32;
        let mut power: i32 = (id * y + input) as i32;
        power = power * id;
        let mut string = power.to_string();
        string.pop();
        string.pop();
        power = string.pop().unwrap().to_digit(10).unwrap() as i32;
        power -= 5;
        power
    }).collect();
    let mut max_power = 0;
    let mut max_coord = (0, 0, 0);
    for x in 0..298 {
        for y in 0..298 {
            for size in 0..min(20, min(300 - x, 300 - y)) {
                let mut current_power = 0;
                for i in 0..size {
                    for j in 0..size {
                        if (y + j) * 300 + x + i >= 90000 {
                            println!("{}, {}, {}, {}, {}", x, y, size, i, j);
                        }
                        current_power += power[(y + j) * 300 + x + i];
                    }
                }
                if current_power > max_power {
                    max_power = current_power;
                    max_coord = (x, y, size);
                }
            }
        }
    }
    println!("{:?}", max_coord);
}