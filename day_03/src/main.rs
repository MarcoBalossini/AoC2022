use std::env::args;
use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let path = args().nth(1).expect("Need a path");
    //let path = String::from("./in");

    //
    // Part 1
    //
    let content = read_to_string(&path).expect("[!] Can't read input file");
    let lines: Vec<&str> = content.lines().collect();

    let halves: Vec<(&str, &str)> = lines
        .iter()
        .map(|line| line.split_at(line.len()/2))
        .collect();

    let points: isize = halves
        .iter()
        .map(|&h| letter_to_int(strings_to_letter(vec![h.0, h.1])))
        .sum();

    println!("[1] {points}");

    //
    // Part 2
    //
    let content = read_to_string(&path).expect("[!] Can't read input file");
    let lines: Vec<&str> = content.lines().collect();

    let points: isize = lines.chunks(3)
        .into_iter()
        .map(|c| letter_to_int(strings_to_letter(vec![c[0], c[1], c[2]])))
        .sum();

    println!("[2] {points}");
}

fn letter_to_int(letter: char) -> isize {
    let c = letter as isize;
    if c > 96 {
        let n = c - 'a' as isize + 1;
        return n;
    }
    let n = c - 'A' as isize + 27;
    n
}

fn strings_to_letter(strings: Vec<&str>) -> char {
    *strings
        .iter()
        .map(|s| s.chars().into_iter().collect::<HashSet<char>>())
        .reduce(|a, b| 
            a.intersection(&b)
            .map(|c| *c)
            .collect()
        ).unwrap()
        .iter()
        .next()
        .unwrap()
}