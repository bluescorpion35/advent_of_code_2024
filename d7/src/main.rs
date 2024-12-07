// Really bad solution - part 1 was fine but part 2 only sometimes works and the check had to be raised from 20000

use crate::Bool3::Undecided;
use crate::Bool3::*;
use crate::Operators::*;
use aoclib::read_to_vec;
use rand::prelude::*;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::io::Write;

fn main() {
    let file = read_to_vec("input");
    let (mut key, mut val): (Vec<i64>, Vec<Vec<i32>>) = (vec![], vec![vec![]]);
    for line in &file {
        let temp = line.split(": ").collect::<Vec<&str>>();
        key.push(temp[0].parse::<i64>().unwrap());
        val.push(
            temp[1]
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }
    let mut pairs: Vec<Pair> = vec![];
    val.remove(0);
    for i in 0..key.len() {
        pairs.push(Pair::create(key[i], &val[i]))
    }
    let mut total = 0;
    let mut counter = 0;
    for mut pair in pairs {
        counter += 1;
        println!("{counter}");
        let mut pair = pair.clone();
        let mut solution: Bool3 = Undecided;
        let mut counter = 0;
        let mut opers: Vec<Operators> = vec![];
        while !solution.decided() {
            counter += 1;
            let mut operations: Vec<Operators> = vec![];
            for i in 0..pair.val.len() - 1 {
                let mut rng = rand::thread_rng();
                let rand = rng.gen_range(1..=3);
                if rand == 1 {
                    operations.push(Mult)
                } else if rand == 2 {
                    operations.push(Add)
                } else if rand == 3 {
                    operations.push(Bar)
                }
            }
            if Operators::check_operations(Pair {
                operations: operations.clone(),
                key: pair.key,
                val: pair.val.clone(),
            }) {
                opers = operations.clone();
                solution = True;
            }
            std::io::stdout().flush().unwrap();
            if counter > 100000 {
                solution = False;
            }
        }
        if solution.conv().unwrap() {
            total += Operators::operate(&pair.val, &opers);
        }
    }
    println!("{}", total);
}

impl Pair {
    pub fn create(key: i64, val: &Vec<i32>) -> Pair {
        Pair {
            key: key,
            val: val.to_vec(),
            operations: vec![],
        }
    }
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nk:{} v: {:?}", self.key, self.val)
    }
}

#[derive(Clone)]
struct Pair {
    key: i64,
    val: Vec<i32>,
    operations: Vec<Operators>,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.key)?;
        for (i, value) in self.val.iter().enumerate() {
            if i > 0 {
                write!(f, "{}", ", ")?;
            }
            write!(f, "{}", value)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum Operators {
    Mult,
    Add,
    Bar,
}

impl Operators {
    pub fn check_operations(pair: Pair) -> bool {
        if Self::operate(&pair.val, &pair.operations) == pair.key {
            true
        } else {
            false
        }
    }
    pub fn operate(nums: &Vec<i32>, operations: &Vec<Operators>) -> i64 {
        let mut total: i128 = nums[0].into();
        if nums.len() == operations.len() + 1 {
            for i in 0..nums.len() - 1 {
                match operations[i] {
                    Mult => total *= nums[i + 1] as i128,
                    Add => total += nums[i + 1] as i128,
                    Bar => {
                        let num_digits = nums[i + 1].abs().to_string().len() as i128;
                        total = total * 10i128.pow(num_digits as u32) + nums[i + 1] as i128;
                    }
                }
            }
            total as i64
        } else {
            println!("PANIC\nNUMS: {nums:?}\nOPERS:{operations:?}");
            panic!("invalid")
        }
    }
}



enum Bool3 {
    True,
    False,
    Undecided,
}

impl Bool3 {
    pub fn conv(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            Bool3::True => Ok(true),
            Bool3::False => Ok(false),
            _ => panic!("error"),
        }
    }
    pub fn decided(&self) -> bool {
        match self {
            Bool3::Undecided => false,
            Bool3::True => true,
            Bool3::False => true,
        }
    }
}
