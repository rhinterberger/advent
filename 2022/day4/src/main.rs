use std::fs;

fn main() {
    let assignments_input = fs::read_to_string("input.txt").expect("Failed to open [input.txt]");
    let assignments = parse_input(assignments_input);

    let mut overlap_complete = 0;
    let mut overlap_partial = 0;

    for current in assignments.chunks(2) {
        match check_overlap(current[0], current[1]) {
            Some(Overlap::Complete) => overlap_complete += 1,
            Some(Overlap::Partial) => overlap_partial += 1,
            None => ()
        }
    }

    println!("{}", overlap_complete);
    println!("{}", overlap_partial + overlap_complete);
}

fn parse_input(input: String) -> Vec<(i32,i32)> {
    let mut assignments: Vec<(i32,i32)> = Vec::new();

    for assignment_pair in input.lines() {
        let mut pair = assignment_pair
            .split(',')
            .map(convert_pair)
            .collect::<Vec<(i32, i32)>>();

        assignments.append(&mut pair);
    }
    assignments
}

fn convert_pair(pair: &str) -> (i32,i32) {
    let mut pair_split = pair.split('-');
    let start = pair_split.next().unwrap().parse::<i32>().unwrap();
    let end = pair_split.next().unwrap().parse::<i32>().unwrap();
    (start, end)
}

enum Overlap {
    Partial,
    Complete
}
fn check_overlap(assign_1: (i32,i32), assign_2: (i32, i32)) -> Option<Overlap> {

    if  assign_1.0 <= assign_2.0 && assign_2.1 <= assign_1.1 || assign_2.0 <= assign_1.0 && assign_1.1 <= assign_2.1 {
        return Some(Overlap::Complete);
    }

    if  assign_1.0 <= assign_2.0 && assign_2.0 <= assign_1.1 || assign_2.0 <= assign_1.0 && assign_1.0 <= assign_2.1 {
        return Some(Overlap::Partial);
    }
    None
}