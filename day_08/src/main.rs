use std::env::args;
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Tree {
    height: usize,
    visible: bool,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

fn main() {
    let path: String;
    match args().nth(1) {
        Some(p) => path = p,
        None => path = String::from("./int"),
    };
    let content = read_to_string(&path).expect("[!] Can't read input file");
    let lines = content.lines();

    let mut wood: Vec<Vec<Tree>> = Vec::new();
    lines.for_each(|l| {
        let mut row: Vec<Tree> = Vec::new();
        l.chars().for_each(|c| row.push(Tree {
            height: c.to_digit(10).unwrap() as usize,
            visible: false,
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }));
        wood.push(row);
    });
    
    let len = wood.len();
    for i in 0..len {
        let mut max_r = 0;
        let mut max_r2 = 0;
        let mut max_c = 0;
        let mut max_c2 = 0;
        for j in 0..len {
            // row
            if j == 0 || wood[i][j].height > max_r {
                max_r = wood[i][j].height;
                wood[i][j].visible = true;
            }
            //reverse row
            if j == 0 || wood[i][len-j-1].height > max_r2 {
                max_r2 = wood[i][len-j-1].height;
                wood[i][len-j-1].visible = true;
            }
            // column
            if j == 0 || wood[j][i].height > max_c {
                max_c = wood[j][i].height;
                wood[j][i].visible = true;
            }
            // reverse column
            if j == 0 || wood[len-j-1][i].height > max_c2 {
                max_c2 = wood[len-j-1][i].height;
                wood[len-j-1][i].visible = true;
            }
        }
    }

    for i in 0..len {
        for j in 0..len {
            let mut up = true;
            let mut down = true;
            let mut left = true;
            let mut right = true;
            for k in 1..len {
                // right
                if j+k < len {
                    if right {
                        wood[i][j].right += 1;
                    }
                    if wood[i][j+k].height >= wood[i][j].height {
                        right = false;
                    }
                }
                //left
                if j >= k {
                    if left {
                        wood[i][j].left += 1;
                    }
                    if wood[i][j-k].height >= wood[i][j].height {
                        left = false;
                    }
                }
                // down
                if i+k < len {
                    if down {
                        wood[i][j].down += 1;
                    }
                    if wood[i+k][j].height >= wood[i][j].height {
                        down = false;
                    }
                }
                // up
                if i >= k {
                    if up {
                        wood[i][j].up += 1;
                    }
                    if wood[i-k][j].height >= wood[i][j].height {
                        up = false;
                    }
                }
            }
        }
    }

    //
    // Part 1
    //
    let count: usize = wood
        .iter()
        .map(|row| {
            row.iter()
                .filter(|t| t.visible == true)
                .count()
        })
        .sum();
    println!("[1] {count}");

    //
    // Part 2
    //
    let max_scenery: usize = wood
        .iter()
        .map(|row| {
            row.iter()
                .map(|t| t.up * t.down * t.left * t.right)
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("[2] {max_scenery}");
}