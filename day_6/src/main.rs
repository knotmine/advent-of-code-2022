use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("File read failed");
    
    let stream: Vec<char> = contents.chars().collect();
    print_type_of(&contents);
    print_type_of(&stream);
    let mlen = 14;
    let mut i = mlen - 1;
    while i < stream.len() {
        let mut j = 0;
        let mut found = true;
        'outer: while j < mlen {
            let mut k = j + 1;
            while k < mlen {
                if stream[i-j] == stream[i-k] {
                    i += mlen - k;
                    found = false;
                    break 'outer;
                }
                k += 1;
            }
            j += 1;
        }

        if found {
            println!("{:?}", &stream[(i-mlen+1)..=i]);
            break;
        }
        // let a = stream[i-3];
        // let b = stream[i-2];
        // let c = stream[i-1];
        // let d = stream[i-0];

        // if c == d {
        //     i += 3;
        // }
        // else if b == c || b == d {
        //     i += 2;
        // }
        // else if a == b || a == c || a ==d {
        //     i += 1;
        // }
        // else {
        //     println!("{:?}", &stream[(i-3)..=i]);
        //     break;
        // }
        // print!("{}, ", i);
    }

    println!("{}", i+1);
}
