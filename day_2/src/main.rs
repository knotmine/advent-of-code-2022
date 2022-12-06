use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("File read error");

    let pick_pts = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),

        // Adding ABC points for part 2
        ("A", 1),
        ("B", 2),
        ("C", 3),
    ]);

    let wins_pts = HashMap::from([
        ("A Y", 6),
        ("B Z", 6),
        ("C X", 6),
        ("A X", 3),
        ("B Y", 3),
        ("C Z", 3),
        ("A Z", 0),
        ("B X", 0),
        ("C Y", 0)
    ]);

    let part2_hint_pick = HashMap::from([
        ("A Y", "A"),
        ("B Z", "C"),
        ("C X", "B"),
        ("A X", "C"),
        ("B Y", "B"),
        ("C Z", "A"),
        ("A Z", "B"),
        ("B X", "A"),
        ("C Y", "C")
    ]);

    let part2_win_pts = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);

    let mut score = 0;
    let mut score2 = 0;
    for l in contents.split_terminator('\n') {
        let moves: Vec<&str> = l.split_whitespace().collect();

        let pick = pick_pts.get(moves[1]).unwrap();
        let result = wins_pts.get(l).unwrap();

        score += pick + result;

        // Part 2 calc
        let pick2 = pick_pts.get(part2_hint_pick.get(l).unwrap()).unwrap();
        let result2 = part2_win_pts.get(moves[1]).unwrap();

        score2 += pick2 + result2;

    }

    println!("Total score: {} ", score);
    println!("Total score2: {} ", score2);
}
