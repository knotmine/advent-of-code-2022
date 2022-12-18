use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("File read failed");

    let mut count = 0;
    let mut ocount = 0;
    for l in contents.split_terminator('\n') {
        let a :Vec<_> = l.split_terminator(&[',','-'][..]).map(|x| x.parse::<i32>().unwrap()).collect();
        if (a[0] <= a[2] && a[1] >= a[3]) || (a[0] >= a[2] && a[1] <= a[3]) {
            count += 1;
            
        }

        if !((a[0] < a[2] && a[1] < a[2]) || (a[2] < a[0] && a[3] < a[0])) {
            ocount += 1;
            println!("{} => {}", l, ocount);
        } 
    }
    println!("Count: {} {} ", count, ocount);
}
