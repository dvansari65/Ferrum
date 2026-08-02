use std::io;

pub mod poseidon;
pub mod merkle;

#[derive(Debug)]
pub struct Values {
    pub value_1: u32,
    pub value_2: u32,
}
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut input = String::new();
    println!("Enter target:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let target = input.trim().parse().unwrap();
    let mut right = 0;
    let mut left = arr.len() - 1;

    let mut obj: Vec<Values> = vec![];
    while right < left {
        let sum = arr[right] + arr[left];
        if sum == target {
            obj.push(Values {
                value_1: arr[right],
                value_2: arr[left],
            });
            continue;
        }
        right += 1;
        left -= 1;
    }
    println!("values:{:?}", obj);
    let trimmed = input.trim();
    println!("You entered:{:?}", trimmed);
}
