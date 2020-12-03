use std::fs::File;
use std::io::Read;

fn main() {
    question_one();
    question_two();
}

struct Slope {
    right: usize,
    down: usize
}

fn question_one() {
    let map = read_input();
    let mut tree_count = 0;
    let inc_slope = Slope {right: 3, down: 1};
    let mut cur_slope = Slope {right: 0, down: 0};

    for row in map {
        let pos = row.as_bytes()[cur_slope.right] as char;
        if pos == '#' {
            tree_count += 1
        }
        cur_slope.right = (cur_slope.right + inc_slope.right) % row.len();
    }

    println!("Question one: Trees encountered: {}", tree_count);
}

fn question_two() {
    let map = read_input();

    let slopes = [Slope {right: 1, down: 1},
                  Slope {right: 3, down: 1},
                  Slope {right: 5, down: 1}, 
                  Slope {right: 7, down: 1},
                  Slope {right: 1, down: 2}];

    let mut result: i64 = 1;

    for inc_slope in &slopes {
        let mut tree_count = 0;
        let mut cur_slope = Slope {right: 0, down: 0}; 
        
        while cur_slope.down < map.len() {
            let row = &map[cur_slope.down];
            let pos = row.as_bytes()[cur_slope.right] as char;
            if pos == '#' {
                tree_count += 1
            }
            cur_slope.down += inc_slope.down;
            cur_slope.right = (cur_slope.right + inc_slope.right) % row.len();
        }
        result *= tree_count;
        println!("Tree count: {}", tree_count);
    }

    println!("Question two: Tree multiplication: {}", result);
}

fn read_input() -> Vec<String> {
    let mut file = File::open("input/day_3.txt").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut v: Vec<String> = Vec::new();

    for s in contents.lines() {
        v.push(s.to_string());
    }

    v
}