extern crate chrono;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::boxed;
use std::collections::HashSet;
use std::collections::HashMap;
use chrono::{NaiveDateTime, Timelike};

fn main() {
    let f = File::open("input/1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut ordered = reader
      .lines()
      .map(|x| x.unwrap())
      .map(|mut x| {
        let event = x.split_off(19);
        let datetime = NaiveDateTime::parse_from_str(&x, "[%Y-%m-%d %H%M] ").unwrap();
        (datetime, event)
      })
      .collect::<Vec<_>>();
    ordered
      .sort_by(|(datetime1, _event1), (datetime2, _event2)| datetime1.cmp(&datetime2));
    let mut time_asleep: HashMap<&str, Vec<u32>> = HashMap::new();
    let mut current_id = "";
    ordered.iter().for_each(|(datetime, event)| {
      if event == "wakes up" {
        // Decrement the rest of the list
        let entry = time_asleep.entry(current_id).or_insert(Vec::new());
        for i in datetime.minute()..60 {
          entry.splice(i, entry.get(i).unwrap() + 1);
        }
        let mut part1: Vec<u32> = entry.iter().take(datetime.minute() as usize).collect();
        let part2 = entry.iter().skip(datetime.minute() as usize).map(|x| x - 1);
        part1.extend(part2);
        *entry = part1;
      } else if event == "falls asleep" {
        // Increment the rest of the list
        let entry = time_asleep.entry(current_id).or_insert(Vec::new());
        *entry = entry.iter().map(|x| x + 1).collect();
      } else if event.starts_with("Guard") {
        // Find id
        let mut temp = event.split("#");
        temp.next();
        current_id = temp.next().unwrap().split(" ").next().unwrap().trim();
        time_asleep.entry(current_id).or_insert(Vec::new());
      }
    });
}