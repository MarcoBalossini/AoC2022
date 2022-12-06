use std::env::args;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Clone, Copy)]
struct Move {
    from: usize,
    to: usize,
    n: usize,
}

fn main() {
    let path = args().nth(1).expect("Need a path");
    //let path = String::from("./int");

    let content = read_to_string(&path).expect("[!] Can't read input file");
    let lines = content.lines();

    let mut n_columns: usize = 0;
    let mut map_strings: Vec<&str> = Vec::new();
    let mut moves: Vec<&str> = Vec::new();
    let mut is_move = false;
    for l in lines {
        if l.is_empty() {

        } else if is_move {moves.push(l)}
        else {
            if l.clone().trim().chars().nth(0).unwrap() == '1' {
                is_move = true;
                let re = Regex::new(r"\s+(\d+) $").unwrap();
                let captures = re.captures(l).unwrap();
                n_columns = captures[1].parse::<usize>().unwrap();
            } else {
                map_strings.push(l);
            }
        }
    }
    map_strings.reverse();

    let mut map: Vec<Vec<char>> = vec![Vec::new();n_columns];
    map_strings
        .iter()
        .for_each(|l| {
            for i in 0..n_columns {
                let c = l.chars().nth(1+i*4).unwrap();
                if c != ' ' {
                    map[i].push(c);
                }
            }
        });

    let moves: Vec<Move> = moves
        .iter()
        .map(|s| parse_move(s))
        .collect();

    //
    // Part 1
    //
    let mut map1 = map.clone();
    moves
        .iter()
        .for_each(|m| {
            let i = m.n;
            for _ in 0..i {
                let tmp = map1[m.from-1].pop().unwrap();
                map1[m.to-1].push(tmp);
            }
        });

    let res1: String = map1
        .iter()
        .map(|c| c.last().unwrap())
        .collect();

    println!("{res1}");

    //
    // Part 2
    //
    let mut map2 = map.clone();
    moves
        .iter()
        .for_each(|m| {
            let i = m.n;
            let mut tmp_vec: Vec<char> = Vec::new();
            for _ in 0..i {
                let tmp = map2[m.from-1].pop().unwrap();
                tmp_vec.push(tmp);
            }
            while !tmp_vec.is_empty() {
                map2[m.to-1].push(tmp_vec.pop().unwrap());
            }
        });

    let res1: String = map2
        .iter()
        .map(|c| c.last().unwrap())
        .collect();

    println!("{res1}");
}

fn parse_move(m_string: &str) -> Move {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = re.captures(m_string).unwrap();
    let n = captures[1].parse::<usize>().unwrap();
    let from = captures[2].parse::<usize>().unwrap();
    let to = captures[3].parse::<usize>().unwrap();
    Move { from: from, to: to, n: n }
}