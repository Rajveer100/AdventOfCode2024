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
use std::{u32, usize};

use regex::Regex;

// use std::rc::Rc;
// use std::cell::RefCell;

// use itertools::Itertools;

// const MOD: i64 = 1_000_000_007;
// const MOD: i64 = 998_244_353;

// const BIG_INF: i64 = 1_000_000_000_000_000_000;
// const SMALL_INF: i32 = 1_000_000_000;

// const MAX_LIM: i32 = 2_000_000;

const DIR4: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
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
    let path = Path::new("day15_input.txt");
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

    let mut warehouse: Vec<Vec<char>> = input
        .lines()
        .take_while(|line| line.contains("#"))
        .map(|line| line.chars().collect())
        .collect();
    let moves: Vec<char> = input
        .lines()
        .skip_while(|line| line.contains("#") || line.contains(" "))
        .collect::<String>()
        .chars()
        .collect();

    let mut start: (i32, i32) = (-1, -1);
    for i in 0..warehouse.len() {
        for j in 0..warehouse[0].len() {
            if warehouse[i][j] == '@' {
                start = (i as i32, j as i32);
            }
        }
    }

    for &cur_move in moves.iter() {
        let index: usize = match cur_move {
            'v' => 0,
            '>' => 1,
            '^' => 2,
            '<' => 3,
            _ => panic!("invalid character"),
        };
        let (mut x, mut y) = (start.0 + DIR4[index].0, start.1 + DIR4[index].1);
        if x >= 0 && x < warehouse.len() as i32 && y >= 0 && y < warehouse[0].len() as i32 {
            if warehouse[x as usize][y as usize] == '.' {
                warehouse[x as usize][y as usize] = '@';
                warehouse[start.0 as usize][start.1 as usize] = '.';
                start = (x, y);
            } else if warehouse[x as usize][y as usize] == 'O' {
                let mut cnt: i32 = 0;
                loop {
                    x += DIR4[index].0;
                    y += DIR4[index].1;
                    if !(x >= 0
                        && x < warehouse.len() as i32
                        && y >= 0
                        && y < warehouse[0].len() as i32)
                    {
                        break;
                    }
                    if warehouse[x as usize][y as usize] == '#' {
                        cnt = 0;
                        break;
                    }
                    cnt += 1;
                    if warehouse[x as usize][y as usize] == '.' {
                        break;
                    }
                }

                if cnt == 0 {
                    continue;
                }

                let (mut shift_x, mut shift_y) = (x, y);
                while cnt > 0 {
                    let n_shift_x = shift_x - DIR4[index].0;
                    let n_shift_y = shift_y - DIR4[index].1;
                    warehouse[shift_x as usize][shift_y as usize] = 'O';
                    warehouse[n_shift_x as usize][n_shift_y as usize] = '.';
                    shift_x = n_shift_x;
                    shift_y = n_shift_y;
                    cnt -= 1;
                }
                warehouse[shift_x as usize][shift_y as usize] = '@';
                warehouse[start.0 as usize][start.1 as usize] = '.';
                start = (shift_x, shift_y);
            }
        }
    }

    let mut sum: i64 = 0;
    for i in 0..warehouse.len() {
        for j in 0..warehouse[0].len() {
            if warehouse[i][j] == 'O' {
                sum += 100 * i as i64 + j as i64;
            }
        }
    }

    writeln!(writer, "{}", sum).ok();
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
