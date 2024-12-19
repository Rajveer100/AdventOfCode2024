use std::fs::File;
use std::io::{self, Read};
use std::io::{stdout, BufWriter, Stdout, Write};

use std::mem::{replace, swap};
// use std::cmp::{max, min, Ordering, Reverse};

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use std::ops::{
    Add, AddAssign, Bound::Excluded, Bound::Included, Bound::Unbounded, Div, DivAssign, Mul,
    MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::path::Path;
use std::time::Instant;

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
    let path = Path::new("day19_input.txt");
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

    let mut towel_patterns: Vec<String> = Vec::new();
    let mut desired_designs: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.contains(",") {
            let patterns: Vec<String> = line.split(", ").map(|s| s.to_string()).collect();
            towel_patterns = patterns;
        } else if !line.is_empty() {
            desired_designs.push(line.to_string());
        }
    }

    let mut has_pattern: HashSet<&str> = HashSet::new();
    for towel_pattern in towel_patterns.iter() {
        has_pattern.insert(towel_pattern.as_str());
    }

    let mut total: i64 = 0;
    for desired_design in desired_designs.iter() {
        let mut exists_pattern: Vec<Vec<bool>> =
            vec![vec![false; desired_design.len()]; desired_design.len()];
        let desired_design_chars: Vec<char> = desired_design.chars().collect();
        for i in 0..desired_design.len() {
            let mut sub_str = String::new();
            for j in i..desired_design.len() {
                sub_str.push(desired_design_chars[j]);
                if has_pattern.contains(&sub_str.as_str()) {
                    exists_pattern[i][j] = true;
                }
                for k in i..j {
                    if exists_pattern[i][k] && has_pattern.contains(&sub_str[k - i + 1..]) {
                        exists_pattern[i][j] = true;
                        break;
                    }
                }
            }
        }
        if exists_pattern[0][desired_design.len() - 1] {
            total += 1;
        }
    }

    writeln!(writer, "{}", total).ok();
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
