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
    let path = Path::new("day25_input.txt");
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

    let mut all_locks: Vec<Vec<i64>> = Vec::new();
    let mut all_keys: Vec<Vec<i64>> = Vec::new();

    let mut cur_lock_or_key: VecDeque<Vec<char>> = VecDeque::new();

    let mut is_lock = -1;
    for line in input.lines() {
        if line.contains(".") || line.contains("#") {
            if is_lock == -1 {
                is_lock = if line.chars().all(|ch| ch == '#') {
                    1
                } else {
                    0
                };
            }
            if is_lock == 1 {
                cur_lock_or_key.push_back(line.chars().collect());
            } else {
                cur_lock_or_key.push_front(line.chars().collect());
            }
        } else {
            if is_lock == 1 {
                update_locks_or_keys(&mut cur_lock_or_key, &mut all_locks);
            } else {
                update_locks_or_keys(&mut cur_lock_or_key, &mut all_keys);
            }
            is_lock = -1;
        }
    }

    if is_lock == 1 {
        update_locks_or_keys(&mut cur_lock_or_key, &mut all_locks);
    } else {
        update_locks_or_keys(&mut cur_lock_or_key, &mut all_keys);
    }
    is_lock = -1;

    let mut ok_combinations: i64 = 0;
    for lock in all_locks.iter() {
        for key in all_keys.iter() {
            let mut is_ok = true;
            for k in 0..lock.len() {
                if lock[k] + key[k] > 5 {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                ok_combinations += 1;
            }
        }
    }

    writeln!(writer, "{}", ok_combinations).ok();
}

fn update_locks_or_keys(
    cur_lock_or_key: &mut VecDeque<Vec<char>>,
    all_locks_or_keys: &mut Vec<Vec<i64>>,
) {
    let mut lock_or_key_height: Vec<i64> = vec![0; cur_lock_or_key[0].len()];
    for i in 1..cur_lock_or_key.len() {
        for j in 0..cur_lock_or_key[0].len() {
            if cur_lock_or_key[i][j] == '#' {
                lock_or_key_height[j] += 1;
            }
        }
    }
    all_locks_or_keys.push(lock_or_key_height);
    cur_lock_or_key.clear();
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
