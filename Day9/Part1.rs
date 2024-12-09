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
    let path = Path::new("day9_input.txt");
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

    let mut file_blocks: Vec<i64> = input
        .chars()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, ch)| (ch as u32) as i64 - 48)
        .collect();
    let mut free_blocks: Vec<i64> = input
        .chars()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, ch)| (ch as u32) as i64 - 48)
        .collect();

    let mut check_sum: i64 = 0;

    let mut free_block_pos: usize = 0;
    let mut free_blocks_queue: VecDeque<(i64, i64)> = VecDeque::new();

    let (mut l_pos, mut r_pos): (usize, usize) = (0, file_blocks.len() - 1);
    let mut l_expanded_pos: i64 = l_pos as i64;

    let mut is_file_block = true;
    while l_pos <= r_pos {
        if is_file_block {
            let file_block = file_blocks[l_pos];
            let r_expanded_pos = l_expanded_pos + file_block - 1;

            let sum: i64 = l_pos as i64
                * ((r_expanded_pos * (r_expanded_pos + 1) / 2)
                    - (l_expanded_pos * (l_expanded_pos - 1) / 2));
            check_sum += sum;

            l_pos += 1;
            l_expanded_pos = r_expanded_pos + 1;
        } else {
            if free_block_pos < free_blocks.len() {
                free_blocks_queue.push_back((free_blocks[free_block_pos], l_expanded_pos));
                free_block_pos += 1;
            }

            while let Some(first_free_block) = free_blocks_queue.front() {
                if first_free_block.0 != 0 && l_pos <= r_pos {
                    let movable_block = file_blocks[r_pos];
                    let movable_blocks = free_blocks_queue[0].0.min(movable_block);
                    let r_expanded_pos = free_blocks_queue[0].1 + movable_blocks - 1;

                    let sum: i64 = r_pos as i64
                        * ((r_expanded_pos * (r_expanded_pos + 1) / 2)
                            - (free_blocks_queue[0].1 * (free_blocks_queue[0].1 - 1) / 2));
                    check_sum += sum;

                    free_blocks_queue[0].0 -= movable_blocks;
                    free_blocks_queue[0].1 = r_expanded_pos + 1;

                    file_blocks[r_pos] -= movable_blocks;
                    if file_blocks[r_pos] == 0 {
                        r_pos -= 1;
                        if !free_blocks.is_empty() {
                            free_blocks.pop();
                        }
                    }

                    l_expanded_pos = free_blocks_queue[0].1 + free_blocks_queue[0].0;

                    if free_blocks_queue[0].0 == 0 {
                        free_blocks_queue.pop_front();
                        break;
                    }
                } else {
                    free_blocks_queue.pop_front();
                    break;
                }
            }
        }
        is_file_block = !is_file_block;
    }

    writeln!(writer, "{}", check_sum).ok();
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
