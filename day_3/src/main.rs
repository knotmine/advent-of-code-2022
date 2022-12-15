use std::collections::{HashSet, HashMap};
use std::fs;

fn main() {
    println!("In file {}", "./input.txt");
    let contents = fs::read_to_string("./input.txt")
        .expect("Missing Input file");

    let mut priority = HashMap::new();
    let mut i = 1;
    for a in 'a'..='z' {
        priority.insert(a, i);
        i+=1;
    }
    for a in 'A'..='Z' {
        priority.insert(a, i);
        i+=1;
    }
    let mut sum = 0;
    for l in contents.split_terminator('\n') {
        let len = l.len();
        let sack1: HashSet<char> = l[0 .. (len/2)].chars().collect();
        let sack2: HashSet<char> = l[len/2 .. len].chars().collect();

        for a in sack1.intersection(&sack2) {
            sum += priority[a];
            // println!("{}\n{}\nSum {}={} {}", &l[0 .. (len/2)], &l[len/2+1 .. len], a, priority[a], sum);
        }
    }

    println!("Sum {}", sum);

    sum = 0;
    let mut iter = contents.split_terminator('\n').into_iter();
    loop {
        let l1= iter.next();
        let l2= iter.next();
        let l3= iter.next();
        if l1 == None || l2 == None || l3 == None {
            break;
        }
        
        let sack1: HashSet<char> = HashSet::from(l1.unwrap().chars().collect::<HashSet<_>>());
        let sack2: HashSet<char> = HashSet::from(l2.unwrap().chars().collect::<HashSet<_>>());
        let sack3: HashSet<char> = HashSet::from(l3.unwrap().chars().collect::<HashSet<_>>());

        for a in sack1.intersection(&sack2.intersection(&sack3).cloned().collect()) {
            sum += priority[a];
            // println!("Sum {}={} {}", a, priority[a], sum);
        }
    }

    println!("Sum {}", sum);
}