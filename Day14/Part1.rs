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

const BATHROOM_HEIGHT: usize = 103;
const BATHROOM_WIDTH: usize = 101;

fn solve(reader: &mut Reader, writer: &mut BufWriter<Stdout>) {
    let path = Path::new("day14_input.txt");
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

    let mut robot_positions: Vec<(i64, i64)> = Vec::new();
    let mut robot_velocities: Vec<(i64, i64)> = Vec::new();

    for line in input.lines() {
        let position_velocity: Vec<&str> = line.split_whitespace().collect();
        let position: Vec<i64> = position_velocity[0][2..]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        let velocity: Vec<i64> = position_velocity[1][2..]
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect();
        robot_positions.push((position[1], position[0]));
        robot_velocities.push((velocity[1], velocity[0]));
    }

    let mut simulation_queue: VecDeque<((i64, i64), (i64, i64), i64)> = VecDeque::new();
    for (&robot_position, &robot_velocity) in robot_positions.iter().zip(robot_velocities.iter()) {
        simulation_queue.push_back((robot_position, robot_velocity, 0));
    }

    let mut quadrant_counts: Vec<i64> = vec![0; 4];
    while let Some((robot_position, robot_velocity, iterations)) = simulation_queue.pop_front() {
        if iterations == 100 {
            if robot_position.0 < (BATHROOM_HEIGHT / 2) as i64
                && robot_position.1 < (BATHROOM_WIDTH / 2) as i64
            {
                quadrant_counts[0] += 1;
            } else if robot_position.0 < (BATHROOM_HEIGHT / 2) as i64
                && robot_position.1 > (BATHROOM_WIDTH / 2) as i64
            {
                quadrant_counts[1] += 1;
            } else if robot_position.0 > (BATHROOM_HEIGHT / 2) as i64
                && robot_position.1 < (BATHROOM_WIDTH / 2) as i64
            {
                quadrant_counts[2] += 1;
            } else if robot_position.0 > (BATHROOM_HEIGHT / 2) as i64
                && robot_position.1 > (BATHROOM_WIDTH / 2) as i64
            {
                quadrant_counts[3] += 1;
            }
            continue;
        }

        let mut next_robot_position = (
            robot_position.0 + robot_velocity.0,
            robot_position.1 + robot_velocity.1,
        );
        if next_robot_position.0 < 0 {
            next_robot_position.0 += BATHROOM_HEIGHT as i64;
        }
        if next_robot_position.0 >= BATHROOM_HEIGHT as i64 {
            next_robot_position.0 -= BATHROOM_HEIGHT as i64;
        }
        if next_robot_position.1 < 0 {
            next_robot_position.1 += BATHROOM_WIDTH as i64;
        }
        if next_robot_position.1 >= BATHROOM_WIDTH as i64 {
            next_robot_position.1 -= BATHROOM_WIDTH as i64;
        }
        simulation_queue.push_back((next_robot_position, robot_velocity, iterations + 1));
    }

    let safety_factor: i64 = quadrant_counts
        .into_iter()
        .reduce(|acc, c| acc * c)
        .unwrap();
    writeln!(writer, "{}", safety_factor).ok();
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
