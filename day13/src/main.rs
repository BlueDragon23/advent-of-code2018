use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::cmp::{Ordering};
extern crate itertools;
use itertools::Itertools;

fn main() {
    let f = File::open("input/input.txt").unwrap();
    let reader = BufReader::new(f);
    // Vec<(x, y, direction, next_turn)>
    let mut carts = Vec::new();
    let mut cart_map = HashSet::new();
    let mut line_number = 0;
    let mut id = 0;
    let track: Vec<Vec<_>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut index = 0;
            for c in line.chars() {
                if c == '^' || c == 'v' || c == '>' || c == '<' {
                    carts.push((index, line_number, c, 0, id));
                    cart_map.insert((index, line_number));
                    id += 1;
                }
                index += 1;
            }
            line_number += 1;
            line.replace("^", "|").replace("v", "|").replace("<", "-").replace(">", "-").chars().collect()
        }).collect();
    
    // for y in 0..track.len() {
    //     for x in 0..track[y].len() {
    //         if cart_map.contains(&(x, y)) {
    //             print!("{}", carts.iter().filter(|(_x, _y, _, _)| x == *_x && y == *_y).map(|(_, _, direction, _)| direction).collect::<Vec<_>>()[0]);
    //         } else {
    //             print!("{}", track[y][x]);
    //         }
    //     }
    //     print!("\n");
    // }
    let mut dead_carts = HashSet::new();
    loop {
        let mut next_carts = Vec::with_capacity(carts.len());
        for line_number in 0..track.len() {
            // For each line
            carts
                .iter()
                // filter carts on this line
                .filter(|(_, y, _, _, _)| *y == line_number)
                .for_each(|(x, y, direction, next_turn, id)| {
                    if dead_carts.contains(id) {
                        return;
                    }
                    let mut next_x = *x;
                    let mut next_y = *y;
                    match direction {
                        '^' => next_y -= 1,
                        'v' => next_y += 1,
                        '<' => next_x -= 1,
                        '>' => next_x += 1,
                        _ => panic!(),
                    }
                    let mut next_direction = *direction;
                    let mut next_next_turn = *next_turn;
                    let piece = track[next_y][next_x];
                    match piece {
                        '/' => match direction {
                            '^' => next_direction = '>',
                            '<' => next_direction = 'v',
                            '>' => next_direction = '^',
                            'v' => next_direction = '<',
                            _ => panic!(),
                        },
                        '\\' => match direction {
                            '^' => next_direction = '<',
                            '>' => next_direction = 'v',
                            '<' => next_direction = '^',
                            'v' => next_direction = '>',
                            _ => panic!(),
                        },
                        '+' => match next_turn {
                            0 => {
                                next_next_turn = 1;
                                next_direction = match direction {
                                    '^' => '<',
                                    '<' => 'v',
                                    'v' => '>',
                                    '>' => '^',
                                    _ => panic!(),
                                }
                            },
                            1 => {
                                next_next_turn = 2;
                                next_direction = *direction;
                            },
                            2 => {
                                next_next_turn = 0;
                                next_direction = match direction {
                                    '^' => '>',
                                    '<' => '^',
                                    'v' => '<',
                                    '>' => 'v',
                                    _ => panic!(),
                                }
                            },
                            _ => panic!(),
                        },
                        '|' => match direction {
                            '>' => panic!(),
                            '<' => panic!(),
                            _ => (),
                        },
                        '-' => match direction {
                            '^' => panic!(),
                            'v' => panic!(),
                            _ => (),
                        },
                        _ => panic!(),
                    }
                    cart_map.remove(&(*x, *y));
                    if cart_map.contains(&(next_x, next_y)) {
                        // Remove both the carts
                        dead_carts.insert(*id);
                        let other_id = carts.iter().chain(next_carts.iter()).filter(|(_x, _y, _, _, _)| next_x == *_x && next_y == *_y).map(|(_, _, _, _, id)| *id).collect::<Vec<_>>()[0];
                        dead_carts.insert(other_id);
                        let maybe_index = next_carts.iter().position(|(_, _, _, _, id)| *id == other_id);
                        match maybe_index {
                            Some(index) => {next_carts.remove(index); ()},
                            None => (),
                        };
                        cart_map.remove(&(next_x, next_y));
                        // for y in 0..track.len() {
                        //     print!("{:03}: ", y);
                        //     for x in 0..track[y].len() {
                        //         if cart_map.contains(&(x, y)) {
                        //             print!("{}", carts.iter().chain(next_carts.iter()).filter(|(_x, _y, _, _)| x == *_x && y == *_y).map(|(_, _, direction, _)| direction).collect::<Vec<_>>()[0]);
                        //         } else {
                        //             print!("{}", track[y][x]);
                        //         }
                        //     }
                        //     print!("\n");
                        // }
                    } else {
                        cart_map.insert((next_x, next_y));
                        next_carts.push((next_x, next_y, next_direction, next_next_turn, *id));
                    }
                });

        }
        if next_carts.len() == 1 {
            panic!("{:?}", next_carts);
        }
        carts = next_carts.into_iter().sorted_by(|(x1, y1, _, _, _), (x2, y2, _, _, _)| if Ord::cmp(y1, y2) == Ordering::Equal { Ord::cmp(x1, x2) } else { Ord::cmp(y1, y2) });
        
        // for y in 0..track.len() {
        //     for x in 0..track[y].len() {
        //         if cart_map.contains(&(x, y)) {
        //             print!("{}", carts.iter().filter(|(_x, _y, _, _)| x == *_x && y == *_y).map(|(_, _, direction, _)| direction).collect::<Vec<_>>()[0]);
        //         } else {
        //             print!("{}", track[y][x]);
        //         }
        //     }
        //     print!("\n");
        // }
    }
}