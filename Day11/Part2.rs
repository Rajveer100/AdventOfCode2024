use std::fs::File;
use std::io::{self, Read};
use std::io::{stdout, BufWriter, Stdout, Write};

// use std::mem::{swap, replace};
// use std::cmp::{max, min, Ordering, Reverse};

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use std::ops::{
    Add, AddAssign, Bound::Excluded, Bound::Included, Bound::Unbounded, Div, DivAssign, Mul,
    MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::path::Path;
use std::time::Instant;
use std::usize;

use regex::Regex;

// use std::rc::Rc;
// use std::cell::RefCell;

// use itertools::Itertools;

// const MOD: i64 = 1_000_000_007;
// const MOD: i64 = 998_244_353;

// const BIG_INF: i64 = 1_000_000_000_000_000_000;
// const SMALL_INF: i32 = 1_000_000_000;

// const MAX_LIM: i32 = 2_000_000;

// const DIR4: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
// const DIR8: [(i32, i32); 8] = [
//     (1, 0),
//     (0, 1),
//     (-1, 0),
//     (0, -1),
//     (1, 1),
//     (1, -1),
//     (-1, 1),
//     (-1, -1),
// ];

fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>) {
    let path = Path::new("day11_input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(err) => panic!("couldn't open {}: {}", display, err),
        Ok(file) => file,
    };

    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Err(err) => panic!("couldn't open {}: {}", display, err),
        Ok(_) => (),
    }

    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    let mut stone_values: HashMap<(String, i64), i64> = HashMap::new();

    let mut total: i64 = 0;
    for stone in stones.iter() {
        total += get_val(stone.clone(), 0, &mut stone_values);
    }

    writeln!(writer, "{}", total).ok();
}

fn get_val(stone: String, count: i64, stone_values: &mut HashMap<(String, i64), i64>) -> i64 {
    if count == 75 {
        return 1;
    }
    if let Some(&retrieved_stone_value) = stone_values.get(&(stone.clone(), count)) {
        return retrieved_stone_value;
    }
    let stone_value = match stone.len() % 2 {
        1 => {
            let new_stone = if stone == "0" {
                "1".to_string()
            } else {
                (stone.parse::<i64>().unwrap() * 2024).to_string()
            };
            get_val(new_stone, count + 1, stone_values)
        }
        0 => {
            let l_new_stone = stone.chars().take(stone.len() / 2).collect::<String>();
            let r_new_stone = stone.chars().skip(stone.len() / 2).collect::<String>();

            let mut value: i64 = 0;
            for split_stone in [l_new_stone, r_new_stone].iter() {
                let new_stone = if split_stone.chars().all(|ch| ch == '0') {
                    "0".to_string()
                } else {
                    split_stone
                        .chars()
                        .skip_while(|&ch| ch == '0')
                        .collect::<String>()
                };
                value += get_val(new_stone, count + 1, stone_values);
            }
            value
        }
        _ => 0,
    };
    stone_values.insert((stone, count), stone_value);
    stone_value
}

#[allow(dead_code)]
fn gcd<T>(a: T, b: T) -> T
where
    T: Default + Copy + PartialEq + PartialOrd + Rem<Output = T>,
{
    return if b == T::default() { a } else { gcd(b, a % b) };
}

#[allow(dead_code)]
fn lcm<T>(a: T, b: T) -> T
where
    T: Default
        + Copy
        + PartialEq
        + PartialOrd
        + Rem<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    (a * b) / gcd(a, b)
}

#[derive(Default)]
struct Reader {
    buffer: Vec<String>,
}

#[allow(dead_code)]
impl Reader {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed to parse!");
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read!");

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn next_chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }

    fn next_lines(&self) -> Vec<String> {
        let mut input: Vec<String> = Vec::new();
        for line in io::stdin().lines() {
            if line.as_ref().unwrap().len() == 0 {
                break;
            }
            input.push(line.unwrap());
        }
        input
    }
}

fn main() {
    // let start = Instant::now();

    let mut reader = Reader::default();
    let writer = &mut BufWriter::new(stdout());

    let is_multi_test: bool = false;
    let mut t: i32 = 1;

    if is_multi_test {
        t = reader.next();
    }

    for _i in 0..t {
        // write!(writer, "Case #{}: ", _i + 1).ok();
        solve(&mut reader, writer);
    }

    // let elapsed = start.elapsed();
    // let seconds = elapsed.as_secs_f64();

    // writeln!(writer, "Elapsed time: {} seconds", seconds).ok();
}
