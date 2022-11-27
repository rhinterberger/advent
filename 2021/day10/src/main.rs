use std::fs;

fn main() {

    let nav_subsystem = read_input("input.txt");

    let mut error_score = 0;
    let mut scores  =Vec::new();

    for line in nav_subsystem.lines() {
        let mut expect = Vec::new();
        for ch in line.chars() {
            match ch {
                '(' => expect.push(')'),
                '{' => expect.push('}'),
                '[' => expect.push(']'),
                '<' => expect.push('>'),
                ')' | '}' | ']' | '>' => {
                    if ch != expect.pop().unwrap() {
                        expect.clear();
                        error_score += get_error_score(ch);
                        break;
                    }
                },
                _ => panic!("Illegal Character")
            }
        }

        let score = expect.iter()
            .rev()
            .fold(0, |score: i64, ch| -> i64 {
                calculate_score(score, *ch)
            });

        if score > 0 {
            scores.push(score);
        }

    }
    println!("Part 1: {}", error_score);

    scores.sort();
    println!("Part 2: {:?}", scores[scores.len()/2]);
}

fn calculate_score(score: i64, ch: char) -> i64{
    score * 5 + get_score(ch)
}

fn get_score(ch: char) -> i64 {
    match ch {
        ')' => 1,
        '}' => 3,
        ']' => 2,
        '>' => 4,
        _ => 0
    }
}

fn get_error_score(ch: char) -> i32 {
    match ch {
        ')' => 3,
        '}' => 1197,
        ']' => 57,
        '>' => 25137,
        _ => 0
    }
}

fn read_input(path: &str) -> String {

    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))

}