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
    let path = Path::new("day17_input.txt");
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

    let (mut register_a, mut register_b, mut register_c): (i64, i64, i64) = (-1, -1, -1);
    let mut program_instrs: Vec<i64> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        if split_line.is_empty() {
            continue;
        }
        if split_line[0] == "Register" {
            match &split_line[1][0..1] {
                "A" => register_a = split_line[2].parse().unwrap(),
                "B" => register_b = split_line[2].parse().unwrap(),
                "C" => register_c = split_line[2].parse().unwrap(),
                _ => (),
            }
        } else {
            let instrs: Vec<i64> = split_line[1]
                .split(",")
                .map(|inst| inst.parse().unwrap())
                .collect();
            program_instrs = instrs;
        }
    }

    let mut result: Vec<i64> = Vec::new();

    let mut i: usize = 0;
    while i < program_instrs.len() {
        let (op_code, operand) = (program_instrs[i], program_instrs[i + 1]);
        let combo_operand = match operand {
            0..=3 => operand,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => panic!("invalid operand"),
        };

        let mut jump_instr: i64 = -1;
        match op_code {
            0 => register_a /= 1 << combo_operand,
            1 => register_b ^= operand,
            2 => register_b = combo_operand % 8,
            3 => {
                if register_a != 0 {
                    jump_instr = operand;
                }
            }
            4 => register_b ^= register_c,
            5 => result.push(combo_operand % 8),
            6 => register_b = register_a / (1 << combo_operand),
            7 => register_c = register_a / (1 << combo_operand),
            _ => panic!("invalid op code"),
        }
        if jump_instr != -1 {
            i = jump_instr as usize;
        } else {
            i += 2;
        }
    }

    let result_str = result
        .iter()
        .map(|&val| val.to_string())
        .collect::<Vec<String>>()
        .join(",");
    writeln!(writer, "{}", result_str).ok();
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
