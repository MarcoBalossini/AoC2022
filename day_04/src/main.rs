use std::env::args;
use std::fs::read_to_string;
use std::cmp::{max, min};

#[derive(Clone, Copy)]
struct Assignment {
    from: usize,
    to: usize,
}

struct Pair {
    ass1: Assignment,
    ass2: Assignment,
}

fn main() {
    let path = args().nth(1).expect("Need a path");
    //let path = String::from("./in");

    let content = read_to_string(&path).expect("[!] Can't read input file");
    let lines = content.lines();

    let assignments: Vec<Pair> = lines
        .map(|l|
            l.split(",")
            .map(|a|
                a.split("-")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>()
            )
            .map(|v| Assignment {from: v[0], to: v[1]})
            .collect::<Vec<Assignment>>()
        )
        .map(|l_a| Pair { ass1: l_a[0], ass2: l_a[1] })
        .collect();

    //
    // Part 1
    //
    let count: usize = assignments
        .iter()
        .map(|pair| is_contained(pair))
        .sum();
    println!("[1] {count}");

    //
    // Part 2
    //
    let count2: usize = assignments
        .iter()
        .map(|pair| do_overlap(pair))
        .sum();
        println!("[2] {count2}");
}

fn is_contained(pair: &Pair) -> usize {
    let ass1: Assignment = pair.ass1;
    let ass2: Assignment = pair.ass2;
    if (ass1.from<=ass2.from && ass1.to>=ass2.to) || (ass2.from<=ass1.from && ass2.to>=ass1.to) {
        return 1;
    }
    0
}

fn do_overlap(pair: &Pair) -> usize {
    let ass1: Assignment = pair.ass1;
    let ass2: Assignment = pair.ass2;
    if max(ass1.from, ass2.from) <= min(ass1.to, ass2.to) {
        return 1;
    }
    0
}