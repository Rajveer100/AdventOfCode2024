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
    let path = Path::new("day13_input.txt");
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

    let mut machines: Vec<(i64, i64)> = Vec::new();
    let mut prizes: Vec<(i64, i64)> = Vec::new();

    for line in input.lines() {
        if line.contains("Button") {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            let unit_x: i64 = split_line[2]
                .chars()
                .skip(2)
                .take_while(|&ch| ch != ',')
                .collect::<String>()
                .parse()
                .unwrap();
            let unit_y: i64 = split_line[3]
                .chars()
                .skip(2)
                .collect::<String>()
                .parse()
                .unwrap();
            machines.push((unit_x, unit_y));
        } else if line.contains("Prize") {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            let prize_x: i64 = split_line[1]
                .chars()
                .skip(2)
                .take_while(|&ch| ch != ',')
                .collect::<String>()
                .parse()
                .unwrap();
            let prize_y: i64 = split_line[2]
                .chars()
                .skip(2)
                .collect::<String>()
                .parse()
                .unwrap();
            prizes.push((prize_x, prize_y));
        }
    }

    let mut total_cost: i64 = 0;
    for i in 0..prizes.len() {
        let eq1 = (machines[2 * i].0, machines[2 * i + 1].0);
        let eq2 = (machines[2 * i].1, machines[2 * i + 1].1);

        let num = prizes[i].0 * eq2.0 - prizes[i].1 * eq1.0;
        let denom = eq1.1 * eq2.0 - eq2.1 * eq1.0;
        if denom != 0 && num % denom == 0 {
            let y = num / denom;
            if (prizes[i].0 - eq1.1 * y) % eq1.0 == 0 {
                let x = (prizes[i].0 - eq1.1 * y) / eq1.0;
                total_cost += 3 * x + y;
            }
        }
    }

    writeln!(writer, "{}", total_cost).ok();
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
