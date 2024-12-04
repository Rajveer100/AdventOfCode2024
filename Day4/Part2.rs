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

const X_MAS_TYPES: [[[char; 3]; 3]; 4] = [
    [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
    [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
    [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
    [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
];

fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>) {
    let path = Path::new("day4_input.txt");
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

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        matrix.push(Vec::new());
        for ch in line.chars() {
            matrix.last_mut().unwrap().push(ch);
        }
    }

    let mut total_count: i64 = 0;
    for &x_mas_type in X_MAS_TYPES.iter() {
        total_count += x_mas_count(&matrix, x_mas_type)
    }

    writeln!(writer, "{}", total_count).ok();
}

fn x_mas_count(matrix: &Vec<Vec<char>>, x_mas_type: [[char; 3]; 3]) -> i64 {
    let mut total: i64 = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !(i + x_mas_type.len() - 1 < matrix.len()
                && j + x_mas_type[0].len() - 1 < matrix[0].len())
            {
                continue;
            }
            let mut ok = true;
            for i_index in 0..x_mas_type.len() {
                for j_index in 0..x_mas_type[0].len() {
                    ok &= x_mas_type[i_index][j_index] == '.'
                        || (matrix[i + i_index][j + j_index] == x_mas_type[i_index][j_index]);
                }
            }
            if ok {
                total += 1;
            }
        }
    }
    total
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
