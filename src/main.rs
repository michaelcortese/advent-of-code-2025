#![allow(unused)]

use cp::day1;
use std::collections::*;
use std::fs::read_to_string;
use std::{fs::Permissions, io::stdin, mem::take};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    arr
}

fn take_string() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    vec
}
fn to_string(vec: Vec<char>) -> String {
    vec.iter().collect::<String>()
}

fn solve(t: Vec<usize>, j: usize) {
    // ======================= Code Here =========================
}

pub fn main() {
    let mut inputs = read_lines("day1.txt");
    println!("{}", day1::solve(inputs, 4168));
}
