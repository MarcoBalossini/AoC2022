use std::env::args;
use std::fs::read_to_string;

struct Match {
    opp: Move,
    me: Move,
}

struct Match2 {
    opp: Move,
    res: Result,
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn main() {
    //let path = args().nth(1).expect("Need a path");
    let path = String::from("./in");
    part1(&path);

    part2(&path);
}

fn part1(path: &str) {
    let content = read_to_string(path).expect("[!] Can't read input file");

    let lines = content.lines();

    let mut set: Vec<Match> = Vec::new();
    for l in lines {
        let moves: Vec<&str> = l.split(' ').collect();
        let m = Match {
            opp: from_move(moves[0], true),
            me: from_move(moves[1], false),
        };
        set.push(m);
    }

    let points: usize = set.iter().map(|m| points1(m.opp, m.me)).sum();

    println!("[1] The score is: {points}")
}

fn part2(path: &str) {
    let content = read_to_string(path).expect("[!] Can't read input file");

    let lines = content.lines();

    let mut set: Vec<Match2> = Vec::new();
    for l in lines {
        let moves: Vec<&str> = l.split(' ').collect();
        let m = Match2 {
            opp: from_move(moves[0], true),
            res: from_move_res(moves[1]),
        };
        set.push(m);
    }

    let points: usize = set.iter()
        .map(|m| points2(m.opp, m.res))
        .sum();

    println!("[2] The score is: {points}")
}

fn from_move(mv: &str, opp: bool) -> Move {
    match mv {
        "A" if opp => Move::Rock,
        "B" if opp => Move::Paper,
        "C" if opp => Move::Scissors,
        "X" if !opp => Move::Rock,
        "Y" if !opp => Move::Paper,
        "Z" if !opp => Move::Scissors,
        _ => panic!(),
    }
}

fn from_move_res(mv: &str) -> Result {
    match mv {
        "X" => Result::Lose,
        "Y" => Result::Draw,
        "Z" => Result::Win,
        _ => panic!(),
    }
}

fn points1(opp: Move, me: Move) -> usize {
    let t = (opp as isize - me as isize + 3) % 3;
    let pts: usize = match t {
        1 => 0,
        0 => 3,
        2 => 6,
        _ => panic!()
    };
    match me {
        Move::Rock => pts + 1,
        Move::Paper => pts + 2,
        Move::Scissors => pts + 3
    }
}

fn points2(opp: Move, res: Result) -> usize {
    let pts = match (opp, res) {
        (Move::Rock, Result::Win) => 8,
        (Move::Rock, Result::Draw) => 4,
        (Move::Rock, Result::Lose) => 3,
        (Move::Paper, Result::Win) => 9,
        (Move::Paper, Result::Draw) => 5,
        (Move::Paper, Result::Lose) => 1,
        (Move::Scissors, Result::Win) => 7,
        (Move::Scissors, Result::Draw) => 6,
        (Move::Scissors, Result::Lose) => 2,
    };
    println!("{pts}");
    pts
}