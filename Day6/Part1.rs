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
    let path = Path::new("day6_input.txt");
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

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let cur_line: Vec<char> = line.chars().collect();
        map.push(cur_line);
    }

    let mut start: (i32, i32) = (-1, -1);
    let mut dir: (i32, i32) = (-1, -1);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                '>' => {
                    start = (i as i32, j as i32);
                    dir = (0, 1);
                }
                '<' => {
                    start = (i as i32, j as i32);
                    dir = (0, -1);
                }
                'v' => {
                    start = (i as i32, j as i32);
                    dir = (1, 0);
                }
                '^' => {
                    start = (i as i32, j as i32);
                    dir = (-1, 0);
                }
                _ => (),
            }
        }
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    travel_map(start.0, start.1, &mut dir, &map, &mut visited);

    let mut visits: i64 = 0;
    for i in 0..visited.len() {
        for j in 0..visited[0].len() {
            if visited[i][j] {
                visits += 1;
            }
        }
    }

    writeln!(writer, "{}", visits).ok();
}

fn travel_map(
    x: i32,
    y: i32,
    dir: &mut (i32, i32),
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) {
    visited[x as usize][y as usize] = true;

    let (mut nx, mut ny) = (x + dir.0, y + dir.1);
    if nx >= 0 && nx < map.len() as i32 && ny >= 0 && ny < map[0].len() as i32 {
        if map[nx as usize][ny as usize] == '#' {
            if dir.0 != 0 {
                dir.1 = -dir.0;
                dir.0 = 0;
            } else {
                dir.0 = dir.1;
                dir.1 = 0;
            }
            (nx, ny) = (x, y);
        }
        travel_map(nx, ny, dir, map, visited);
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
