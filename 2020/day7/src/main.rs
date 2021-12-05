use std::fs;

fn main() {
    let rules = (&read_input("input.txt"));

    let mut can_contain: Vec<Vec<&str>> = Vec::new();
    can_contain.push(vec!["shiny gold", ""]);

    loop{
        let mut found= false;
        for search in can_contain.clone() {
            for rule in rules {
                let bags_contain = rule.split(" bags contain ").collect::<Vec<&str>>();

                if bags_contain[1].contains(search[0]) && !can_contain.contains(&bags_contain) {
                    can_contain.push(bags_contain);
                    found = true;
                }
            }
        }
        if found == false {
            break;
        }
    }
    println!("{}", can_contain.len()-1);

}
fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
