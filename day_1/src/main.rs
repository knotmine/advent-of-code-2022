use std::{collections::BinaryHeap, fs};

fn main() {
    println!("In file {}", "./input.txt");

    let contents = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let mut max = 0;
    let mut sum = 0;
    let mut sum_heap: BinaryHeap<u32> = BinaryHeap::new();

    for l in contents.split_terminator('\n') {
        if l.trim().is_empty() {
            if sum > max {
                max = sum;
            }
            sum_heap.push(sum);
            sum = 0;
        }
        else {
            let val = l.trim().parse::<u32>().ok().unwrap();
            sum = sum + val;
        }
    }

    println!("max value: {} ", max);

    println!("Top 3 sums: {}", sum_heap.pop().unwrap() + sum_heap.pop().unwrap() + sum_heap.pop().unwrap());
}