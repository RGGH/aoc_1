// didn't use chatGPT once!
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path: &Path = Path::new("data.txt");
    let result = load_from_file(path);
    println!("{:?}", result);
}

fn load_from_file(file_path: &Path) -> i64 {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<_> = reader
        .lines()
        .map(|line| line.unwrap())
        .into_iter()
        .map(|x| x.replace(" ", ""))
        .collect();

    let mut vec_left: Vec<i64> = vec![];
    let mut vec_right: Vec<i64> = vec![];

    for piece in numbers {
        let r = &piece[..5].parse::<i64>().unwrap();
        vec_left.push(*r);
        let s = &piece[6..].parse::<i64>().unwrap();
        vec_right.push(*s);
    }

    vec_left.sort();
    vec_right.sort();

    let a = vec_left;
    let b = vec_right;

    let c: Vec<i64> = a.iter().zip(b).map(|(a, b)| (a - b).abs()).collect();
    let answer: i64 = c.into_iter().sum();
    answer 
}
