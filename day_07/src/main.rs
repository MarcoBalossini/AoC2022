use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;
use regex::Regex;
use itertools::Itertools;

const MAX_COUNT_SIZE: usize = 100000;
const TOTAL_SIZE: usize = 70000000;
const NEEDED_SPACE: usize = 40000000;

struct Node {
    size: usize,
    children: HashMap<String, Node>,
}
impl Node {
    pub fn new() -> Self {
        Node {
            size: 0,
            children: HashMap::new()
        }
    }

    pub fn parse<I: Iterator<Item = Instruction>>(&mut self, instructions: &mut I) {
        // Can move ahead some instructions from INSIDE the loop
        while let Some(i) = instructions.next() {
            match i.cmd.cmd {
                CmdType::CD => {
                    if i.cmd.param == ".." {
                        return;
                    }
                    let node = self.children
                        .entry(i.cmd.param)
                        .or_insert_with(Node::new);
                    node.parse(instructions);
                },
                CmdType::LS => {
                    self.size = i.out
                        .into_iter()
                        .filter_map(|l| {
                            let (a, name) = l.split_whitespace().collect_tuple().unwrap();
                            match a {
                                "dir" => {self.children.insert(String::from_str(name).unwrap(), Node::new()); None},
                                s => Some(s.parse::<usize>().unwrap()),
                            }
                        })
                        .sum();
                },
            }
        }
    }

    pub fn size_subdir(&self) -> usize {
        self.children
            .iter()
            .map(|c| c.1.size_subdir() + c.1.size)
            .sum::<usize>()
    }

    pub fn count_part1(&self) -> usize {
        let size = self.size+self.size_subdir();
        let mut res1_size = self.children
            .iter()
            .map(|c| {
                let count = c.1.count_part1();
                println!("[!] {}: {} /// {}", c.0, count, c.1.size+c.1.size_subdir());
                count
            })
            .sum();
        if size <= MAX_COUNT_SIZE {
            res1_size += size;
        }
        
        res1_size
    }

    pub fn part2(&self, min: &mut usize, needed: usize) {
        let size = self.size+self.size_subdir();
        if size < needed {
            return;
        }
        if &size < min {
            *min = size;
        }
        self.children
            .iter()
            .for_each(|c| c.1.part2(min, needed));
    }
}

struct Instruction  {
    cmd: Command,
    out: Vec<String>,
}

struct Command {
    cmd: CmdType,
    param: String,
}

enum CmdType {
    CD,
    LS,
}
impl CmdType {
    pub fn from_str(s: &str) -> CmdType {
        match s {
            "cd" => CmdType::CD,
            "ls" => CmdType::LS,
            _ => panic!(),
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
    let mut instructions: Vec<Instruction> = Vec::new();
    for l in lines {
        if l.trim().chars().nth(0).unwrap() == '$' {
            let re = Regex::new(r"\$ ([cdls]{2})\s*(\S*)").unwrap();
            let captures = re.captures(l).unwrap();
            let cmd = Command {
                cmd: CmdType::from_str(&captures[1]),
                param: String::from(&captures[2]),
            };

            instructions.push( Instruction { cmd: cmd, out: Vec::new() } );
        } else {
            instructions.last_mut().unwrap().out.push(String::from(l));
        }
    }

    let mut fs: Node = Node::new();
    fs.parse(&mut instructions.into_iter());

    //
    // Part 1
    //
    println!("[1] {}", fs.count_part1());

    //
    // Part 2
    //
    let mut min = TOTAL_SIZE;
    let to_delete = (fs.size+fs.size_subdir()) - NEEDED_SPACE;
    fs.part2(&mut min, to_delete);
    println!("[2] {}", min);
}