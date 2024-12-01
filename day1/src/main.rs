use std::fs::read_to_string;

fn main() {
    let mut left: Vec<String> = Vec::new();
    let mut right: Vec<String> = Vec::new();
    let mut distance_vector: Vec<i32> = Vec::new();
    let lines_array = read_lines("./input.txt");

    for line in lines_array {
        let line : Vec<_> = line.split(" ").filter(|x| *x != "").collect();
        left.push(line[0].to_string());
        right.push(line[1].to_string());
    }

    left.sort();
    right.sort();

    for (index, _element) in left.iter().enumerate() {
        let distance = get_distance(left[index].clone(), right[index].clone());

        distance_vector.push(distance);
    }

    let result: i32 = distance_vector.iter().sum();
    println!("result: {result}");
}

fn get_distance(left: String, right: String) -> i32 {
    let left: i32 = left.parse().expect("Failed parsing String to i32");
    let right: i32 = right.parse().expect("Failed parsing String to i32");

    return (left - right).abs();
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
