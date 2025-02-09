use std::fs;

fn main() {
    let sample: &str = "3   4\n\
    4   3\n\
    2   5\n\
    1   3\n\
    3   9\n\
    3   3\n";
    
    let puzzle: String = fs::read_to_string("../input.txt").unwrap();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let lines: Vec<&str> = puzzle.trim().split("\n").collect();
    
    for l in lines {
        let pair: Vec<i32> = l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        left.push(pair[0]);
        right.push(pair[1]);
    }
    
    left.sort();
    right.sort();
        
    assert_eq!(left.len(), right.len());
    
    let mut total: i32 = 0;
    
    for (n, l) in left.iter().enumerate() {
        let r: i32 = right[n];
        total += (l - r).abs();
    }
    
    println!("{}", total);
}
