use std::fs;

fn main() {

    let mut nav_subsystem = read_input("input.txt");

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
                        match ch {
                            ')' => error_score += 3,
                            '}' => error_score += 1197,
                            ']' => error_score += 57,
                            '>' => error_score += 25137,
                            _ => ()
                        }
                        break;
                    }
                },
                _ => panic!("Illegal Character")
            }
        }

        let mut score = 0_i64;
        while !expect.is_empty() {
            match expect.pop().unwrap() {
                ')' => score = 5 * score + 1,
                '}' => score = 5 * score + 3,
                ']' => score = 5 * score + 2,
                '>' => score = 5 * score + 4,
                _ => ()
            }
        }
        if score > 0 {
            scores.push(score);
        }

    }
    println!("Part 1: {}", error_score);

    scores.sort();
    println!("Part 2: {:?}", scores[scores.len()/2]);
}

fn read_input(path: &str) -> String {

    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))

}