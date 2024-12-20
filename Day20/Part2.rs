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
    let path = Path::new("day20_input.txt");
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

    let mut race_track: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        race_track.push(line_chars);
    }

    let mut src: (i32, i32) = (-1, -1);
    let mut dest: (i32, i32) = (-1, -1);
    for i in 0..race_track.len() {
        for j in 0..race_track[0].len() {
            if race_track[i][j] == 'S' {
                src = (i as i32, j as i32);
            } else if race_track[i][j] == 'E' {
                dest = (i as i32, j as i32);
            }
        }
    }

    let mut race_q: VecDeque<((i32, i32), i32, i64)> = VecDeque::new();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; race_track[0].len()]; race_track.len()];
    let mut times: Vec<Vec<i64>> = vec![vec![0; race_track[0].len()]; race_track.len()];

    race_q.push_back((src, 0, 0));
    visited[src.0 as usize][src.1 as usize] = true;

    let mut positions: Vec<(i32, i32)> = Vec::new();

    let mut original_time: i64 = 0;
    while let Some((pos, cheats, time)) = race_q.pop_front() {
        positions.push(pos);
        if pos == dest {
            original_time = time;
            continue;
        }

        let (x, y) = pos;
        for &(dx, dy) in DIR4.iter() {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0
                && nx < race_track.len() as i32
                && ny >= 0
                && ny < race_track[0].len() as i32
                && !visited[nx as usize][ny as usize]
                && race_track[nx as usize][ny as usize] != '#'
            {
                visited[nx as usize][ny as usize] = true;
                times[nx as usize][ny as usize] = time + 1;
                race_q.push_back(((nx, ny), 0, time + 1));
            }
        }
    }

    let mut saved_time_cnt: i64 = 0;
    for &p1 in positions.iter() {
        for &p2 in positions.iter() {
            if p1 == p2 {
                continue;
            }
            let time_src_p1 = times[p1.0 as usize][p1.1 as usize];
            let cheat_time_p1_p2 = (p1.0 - p2.0).abs() as i64 + (p1.1 - p2.1).abs() as i64;
            let time_p2_dest = original_time - times[p2.0 as usize][p2.1 as usize];
            if cheat_time_p1_p2 <= 20
                && original_time - (time_src_p1 + cheat_time_p1_p2 + time_p2_dest) >= 100
            {
                saved_time_cnt += 1;
            }
        }
    }

    writeln!(writer, "{}", saved_time_cnt).ok();
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
