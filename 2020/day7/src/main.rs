use std::collections::HashMap;
use std::fs;

fn main() {
    let rules = &read_input("input.txt");
    let mut all_search = vec!["shiny gold"];

    loop {
        let mut found = false;
        for search in all_search.clone() {
            for rule in rules {
                if rule.1.contains(search) && !all_search.contains(&rule.0.as_str()) {
                    all_search.push(rule.0);
                    found = true;
                }
            }
        }
        if found == false {
            break;
        }
    }
    println!("{}", all_search.len() - 1);

    let bags = count_bags("shiny gold", rules);
    println!("{}", bags - 1);

}

fn count_bags(search: &str, rules: &HashMap<String, String> ) -> i32 {

    let rule = rules.get(search).unwrap();
    if rule.contains("no other") {
        return 1;
    }

    rule
        .split(", ")
        .map(|part| {
            match part.split_once(" ") {
                Some((amount, bag)) =>
                    count_bags(bag, rules) * amount.parse::<i32>().unwrap(),
                None => 0
            }
        })
        .sum::<i32>() + 1
}


fn read_input(path: &str) -> HashMap<String, String> {
    let mut lines_hash: HashMap<String,String> = HashMap::new();

    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .lines()
        .for_each(|line| {
            let ll = line.replace(" bags","").replace(" bag","");
            let mut l = ll.split(" contain ");
            lines_hash.insert(
                l.next().unwrap().to_string(),
                l.next().unwrap().replace(".","").to_string());
        });

    lines_hash
}