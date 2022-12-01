use std::env::args;
use std::fs::read_to_string;

fn main() {
    let path = args().nth(1).expect("Need a path");

    let content = read_to_string(path)
        .expect("[!] Can't read input file");
    
//    let lists = content.split("\n\n");
//    let mut sums: Vec<usize> = Vec::new();
//     for l in lists {
//         let sum: usize = l.lines()
//             .map(|x| x.parse::<usize>().unwrap())
//             .sum();
//         sums.push(sum);
//     }

    let mut sums: Vec<usize> = content
        .split("\n\n")
        .into_iter()
        .map(|l| l.lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum())
        .collect();

    // Part 1
    let max = sums.iter().max().unwrap();
    println!("[1] The max is: {max}");

    // Part 2
    sums.sort_by(|a, b| b.cmp(a));

    let max3 = sums[0] + sums[1] + sums[2];
    println!("[2] The max 3 is: {max3}");
}