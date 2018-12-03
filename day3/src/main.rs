use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let f = File::open("input/1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut map = HashMap::<(u32, u32), u32>::new();
    let mut seen_twice = HashSet::<(u32, u32)>::new();
    let mut safe = HashSet::new();
    for i in 1..1368 {
      safe.insert(i);
    }
    let mut count = 0;
    let mut index = 0;
    reader
      .lines()
      .map(|x| x.unwrap())
      .map(|x| x
        .split(|c| c == '@' || c == ',' || c == ':' || c == 'x')
        .map(|c| c.trim())
        .map(|c| c.parse::<u32>())
        .filter(|o| o.is_ok())
        .map(|o| o.unwrap()).collect::<Vec<_>>())
      .for_each(|v| {
        index += 1;
        let left = v.get(0).unwrap();
        let top = v.get(1).unwrap();
        let width = v.get(2).unwrap();
        let height = v.get(3).unwrap();
        let mut no_collisions = true;
        for i in *left..(left + width) {
          for j in *top..(top + height) {
            if map.get(&(i, j)).is_some() {
              no_collisions = false;
              safe.remove(map.get(&(i, j)).unwrap());
              if !seen_twice.contains(&(i, j)) {
                count += 1;
                seen_twice.insert((i, j));
              }
            } else {
              map.insert((i, j), index);
            }
          }
        }
        if !no_collisions {
          safe.remove(&index);
        }
      });
    println!("{}", count);
    println!("{:?}", safe);
}