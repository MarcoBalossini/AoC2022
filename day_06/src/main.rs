use std::env::args;
use std::fs::read_to_string;
use std::collections::HashSet;

fn main() {
    let path = args().nth(1).expect("Need a path");
    let window: usize = args().nth(2).expect("How much?").parse().unwrap();
    //let path = String::from("./int");

    let content = read_to_string(&path).expect("[!] Can't read input file");
    let line = content.lines();

    let res1 = line
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>()
        .windows(window)
        .map(|w| {
            let set = w.iter().collect::<HashSet<&char>>();
            if set.len() == window {
                true
            } else {
                false
            }
        })
        .enumerate()
        .find(|(_, x)| *x==true);

    match res1 {
        Some((index, _)) => println!("[!] Result: {}", index+window),
        _ => println!("[!] Error"),
    }
}