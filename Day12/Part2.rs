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
    let path = Path::new("day12_input.txt");
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

    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total: i64 = 0;
    for ch in 'A'..='Z' {
        let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == ch && !visited[i][j] {
                    let mut regions: VecDeque<(i32, i32)> = VecDeque::new();
                    regions.push_back((i as i32, j as i32));
                    visited[i][j] = true;

                    let mut area: i64 = 0;
                    let mut sides_x: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); 4]; map.len()];
                    let mut sides_y: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); 4]; map[0].len()];

                    while let Some((x, y)) = regions.pop_front() {
                        area += 1;
                        for (i, &(dx, dy)) in DIR4.iter().enumerate() {
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            if nx >= 0
                                && nx < map.len() as i32
                                && ny >= 0
                                && ny < map[0].len() as i32
                            {
                                if map[nx as usize][ny as usize] != ch {
                                    if dx != 0 {
                                        sides_x[x as usize][i].push(y as usize);
                                    } else {
                                        sides_y[y as usize][i].push(x as usize);
                                    }
                                } else if !visited[nx as usize][ny as usize] {
                                    regions.push_back((nx, ny));
                                    visited[nx as usize][ny as usize] = true;
                                }
                            } else {
                                if dx != 0 {
                                    sides_x[x as usize][i].push(y as usize);
                                } else {
                                    sides_y[y as usize][i].push(x as usize);
                                }
                            }
                        }
                    }

                    let mut sides_sum: i64 = 0;
                    for i in 0..sides_x.len() {
                        for j in 0..sides_x[0].len() {
                            sides_x[i][j].sort();
                            let mut cur_sum: i64 = 0;
                            for k in 0..sides_x[i][j].len() {
                                if cur_sum == 0 || sides_x[i][j][k] - sides_x[i][j][k - 1] > 1 {
                                    cur_sum += 1;
                                }
                            }
                            sides_sum += cur_sum;
                        }
                    }

                    for i in 0..sides_y.len() {
                        for j in 0..sides_y[0].len() {
                            sides_y[i][j].sort();
                            let mut cur_sum: i64 = 0;
                            for k in 0..sides_y[i][j].len() {
                                if cur_sum == 0 || sides_y[i][j][k] - sides_y[i][j][k - 1] > 1 {
                                    cur_sum += 1;
                                }
                            }
                            sides_sum += cur_sum;
                        }
                    }

                    total += area * sides_sum;
                }
            }
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
