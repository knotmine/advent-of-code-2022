use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("File read failed");

    let mut stack = true;
    let mut cols;
    let mut lines = Vec::new();
    let mut crates: Vec<Vec<&str>> = Vec::new();
    for l in contents.split_terminator('\n') {
        if stack {
            if !l.is_empty() {
                lines.push(l);
            }
            else {
                stack = false;
                cols = lines.pop().unwrap().split_whitespace().count();
                crates = Vec::with_capacity(cols);
                for _ in 0..cols {
                    crates.push(Vec::new());
                }
                // println!("{} => {}", cols, crates.len());

                // Build the cargo
                while lines.len() > 0 {
                    let pop = lines.pop().unwrap();
                    let mut pile = 0;
                    let mut t = 0;
                    let tokens : Vec<&str> = pop.split_terminator(" ").collect();
                    // println!("{:?}", tokens);
                    while  t < tokens.len() {
                        // println!("{} {}", pile, t);
                        if tokens[t] == "" {
                            t += 3;
                        }
                        else {
                            crates[pile].push(tokens[t]);
                        }
                        pile += 1;
                        t += 1;
                    }
                    // println!("{:#?}", crates);
                }
            }
        }
        else {
            // println!("{:?}", crates);
            let tokens : Vec<&str> = l.split_terminator(' ').collect();
            let n = tokens[1].parse::<usize>().unwrap();
            let from = tokens[3].parse::<usize>().unwrap();
            let to = tokens[5].parse::<usize>().unwrap();
            
            for _ in 0..n {
                let c = crates[from-1].pop().unwrap();
                crates[to-1].push(c);
            }

            // part 2
            // let i = crates[from-1].len() - n;
            // let mut back = crates[from-1].split_off(i);
            // crates[to-1].append(&mut back);
        }
    }
    // QNNTGTPFN
    // GGNPJBTTR
    for mut c in crates {
        print!("{}", c.pop().unwrap());
    }
}
