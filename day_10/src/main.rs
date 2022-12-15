use std::collections::BTreeMap;
use std::env::args;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Addx(isize),
    Noop,
}
impl Instruction {
    pub fn from_input(s: &str) -> Instruction {
        let re = Regex::new(r"(\S+)\s*(-?\d*)").unwrap();
        let captures = re.captures(s).unwrap();
        let dir = &captures[1];
        match dir {
            "addx" => Instruction::Addx(captures[2].parse::<isize>().unwrap()),
            "noop" => Instruction::Noop,
            _ => panic!(),
        }
    }
}

struct Machine {
    instructions: Vec<Instruction>,
    history: BTreeMap<isize, isize>,
}
impl Machine {
    pub fn new(instructions: Vec<Instruction>) -> Machine {
        let mut m = Machine {
            instructions,
            history: BTreeMap::new(),
        };
        m.history.insert(0, 1);
        m
    }

    pub fn process_instructions(&mut self) {
        for instruction in self.instructions.iter() {
            let i = self.history.len() as isize;
            let prec = *self.history.get(&(i - 1)).unwrap();
            match instruction {
                Instruction::Addx(n) => {
                    self.history.insert(i, prec);
                    self.history.insert(i + 1, prec + n);
                },
                Instruction::Noop => {self.history.insert(i, prec);},
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
    
    let mut machine = Machine::new(instructions);
    machine.process_instructions();

    //
    // Part 1
    //
    let sum = machine.history
        .iter()
        .filter(|(i, _)| **i % 40 == 19)
        .map(|(i, v)| (i+1) as isize *v)
        .sum::<isize>();

    println!("[1] Sum: {}", sum);
    
    //
    // Part 2
    //
    machine.history
        .iter()
        .for_each(|(i, v)| {
            print!("{}", if (*v>=(i%40) - 1) && (*v<=(i%40) + 1) {'#'} else {'.'});
            if (i+1) % 40 == 0 {
                println!();
            }
        });
}