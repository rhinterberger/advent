use std::fs;

fn main() {
    let mut adaptors = read_input("input.txt");
    adaptors.push(0);
    adaptors.sort();
    adaptors.push(adaptors.last().unwrap() + 3);


    let mut one = 0;
    let mut three = 0;
    for i in 1..adaptors.len() {
        match adaptors[i] - adaptors[i-1] {
            1 => { one+=1; },
            3 => { three+=1; },
            _ => ()
        };
    }
    println!("{}", one * three);

    let mut configurations = vec![0u64; adaptors.len()];
    configurations[0] = 1;

    for start in 0 .. adaptors.len() -1 {
        for step in start+1 .. adaptors.len() {
            if adaptors[step] <= (adaptors[start] + 3) {
                configurations[step] += configurations[start];
            }
            else {
                break;
            }
        }
    }

    println!("{}", configurations.last().unwrap());

}

fn read_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}
