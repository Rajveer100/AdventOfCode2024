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
//     (1, 0), (0, 1), (-1, 0), (0, -1),
//     (1, 1), (1, -1), (-1, 1), (-1, -1)
// ];

fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>) {
    let path = Path::new("day3_input.txt");
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

    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)");
    let do_reg = Regex::new(r"do\(\)").unwrap();
    let dont_reg = Regex::new(r"don't\(\)").unwrap();

    match reg {
        Ok(pat) => {
            let matches: Vec<&str> = pat.find_iter(input.as_str()).map(|m| m.as_str()).collect();
            let match_start_indices: Vec<usize> =
                pat.find_iter(input.as_str()).map(|m| m.start()).collect();

            let mut total: i64 = 0;
            for (m, &m_start_index) in matches.iter().zip(match_start_indices.iter()) {
                if let Some(dont_match) = dont_reg.find_iter(&input[0..m_start_index]).last() {
                    let dont_start_index = dont_match.end();
                    if do_reg
                        .find(&input[dont_start_index..m_start_index])
                        .is_none()
                    {
                        continue;
                    }
                }

                let m: Vec<char> = m.to_string().chars().collect();
                let n1: i64 = m
                    .iter()
                    .skip(4)
                    .take_while(|&&ch| ch != ',')
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let n2: i64 = m
                    .iter()
                    .skip_while(|&&ch| ch != ',')
                    .skip(1)
                    .take_while(|&&ch| ch != ')')
                    .collect::<String>()
                    .parse()
                    .unwrap();
                total += n1 * n2;
            }
            writeln!(writer, "{}", total).ok();
        }
        Err(_) => panic!("invalid regex"),
    }
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
}
