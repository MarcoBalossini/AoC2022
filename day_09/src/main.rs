use std::env::args;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Down(usize),
    Up(usize),
    Right(usize),
    Left(usize),
}
impl Instruction {
    pub fn from_input(s: &str) -> Instruction {
        let re = Regex::new(r"([UDRL]) (\d+)").unwrap();
        let captures = re.captures(s).unwrap();
        let dir = &captures[1];
        let steps = captures[2].parse::<usize>().unwrap();
        match dir {
            "U" => Instruction::Up(steps),
            "D" => Instruction::Down(steps),
            "R" => Instruction::Right(steps),
            "L" => Instruction::Left(steps),
            _ => panic!(),
        }
    }

    pub fn get_move(&self) -> (isize, isize) {
        match self {
            Instruction::Down(_) => (0, -1),
            Instruction::Up(_) => (0, 1),
            Instruction::Right(_) => (1, 0),
            Instruction::Left(_) => (-1, 0),
        }
    }

    pub fn get_steps(&self) -> usize {
        match self {
            Instruction::Down(n) => *n,
            Instruction::Up(n) => *n,
            Instruction::Right(n) => *n,
            Instruction::Left(n) => *n,
        }
    }
}

struct Position {
    positions: Vec<(isize, isize)>,
}
impl Position {
    pub fn new(size: usize) -> Self {
        Position {
            positions: vec![(0, 0); size],
        }
    }

    pub fn move_elf(&mut self, ins: Instruction, visited: &mut Vec<(isize, isize)>) {
        let step = ins.get_move();
        for _ in 0..ins.get_steps() {
            self.positions[0].0 += step.0;
            self.positions[0].1 += step.1;
            for i in 1..self.positions.len() {
                let dx = match self.positions[i-1].0 - self.positions[i].0 {
                    0 => 0,
                    x => if x>0 {1} else {-1},
                };
                let dy = match self.positions[i-1].1 - self.positions[i].1 {
                    0 => 0,
                    x => if x>0 {1} else {-1},
                };
                if self.positions[i].0+dx != self.positions[i-1].0 || self.positions[i].1+dy != self.positions[i-1].1 {
                    self.positions[i].0 += dx;
                    self.positions[i].1 += dy;
                }
                if i == self.positions.len()-1 &&
                            !visited.contains(&(self.positions[i].0, self.positions[i].1)) {
                    visited.push((self.positions[i].0, self.positions[i].1));
                }
            }
        }
    }
}

fn main() {
    let path: String;
    match args().nth(1) {
        Some(p) => path = p,
        None => path = String::from("./int"),
    };
    let content = read_to_string(&path).expect("[!] Can't read input file");
    let lines = content.lines();

    let instructions: Vec<Instruction> = lines
        .map(|l| Instruction::from_input(l))
        .collect();
    
    //
    // Part 1
    //
    let mut visited: Vec<(isize, isize)> = Vec::new();
    visited.push((0, 0));
    let mut pos: Position = Position::new(2);
    
    instructions
        .iter()
        .for_each(|i| {
            pos.move_elf(*i, &mut visited);
        });

    println!("Part 1: {}", visited.len());

    //
    // Part 2
    //
    let mut visited: Vec<(isize, isize)> = Vec::new();
    visited.push((0, 0));
    let mut pos: Position = Position::new(10);
    
    instructions
        .iter()
        .for_each(|i| {
            pos.move_elf(*i, &mut visited);
        });

    println!("Part 2: {}", visited.len());
}