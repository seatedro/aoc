use std::{collections::HashMap, fs};

fn parse_input() -> Result<(Vec<i32>, Vec<i32>), String> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let contents =
        fs::read_to_string("src/1.txt").map_err(|e| format!("Failed to read file: {}", e))?;
    for line in contents.lines() {
        let mut numbers = line.split_whitespace();
        if let (Some(l), Some(r)) = (numbers.next(), numbers.next()) {
            if let (Ok(l_num), Ok(r_num)) = (l.parse::<i32>(), r.parse::<i32>()) {
                left.push(l_num);
                right.push(r_num);
            }
        }
    }

    return Ok((left, right));
}

pub fn parta(left: &mut Vec<i32>, right: &mut Vec<i32>) {
    left.sort();
    right.sort();

    if left.len() != right.len() {
        panic!("shouldn't be the case");
    }

    let mut ans: i32 = 0;
    for it in left.iter().zip(right.iter()) {
        let (l, r) = it;
        ans += (l - r).abs();
    }

    println!("Ans :: {}", ans);
}

pub fn partb(left: &mut Vec<i32>, right: &mut Vec<i32>) {
    let f = right.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|f| *f += 1).or_insert(1);
        map
    });

    let mut ans: i32 = 0;
    left.iter().for_each(|v| {
        ans += v * f.get(v).unwrap_or(&0);
    });

    println!("Ans: {}", ans);
}

pub fn solve() {
    let (mut left, mut right) = parse_input().unwrap();

    // parta(&mut left, &mut right);
    partb(&mut left, &mut right);
}
