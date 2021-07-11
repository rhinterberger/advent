use std::collections::HashMap;
use std::fs;

fn main() {
    let mut bags = Bags::new();
    let rules = bags.parse_rules(&read_input("input.txt"));
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
}

struct Bags {
    rules: HashMap<String,HashMap<String,i32>>
}
impl Bags {
    fn test_color(color: &str) {

    }

    fn parse_rules(&mut self, input: &String) {

        for line in input.lines() {
            let parsed_line = self.parse_line(line);
            self.rules.extend(parsed_line);
        }
    }

    fn parse_line(&self, input: &str) -> HashMap<String,HashMap<String,i32>> {
        let mut result = HashMap::new();
        let mut rule = input.split("bags contain");

        let name = rule.next().unwrap().trim().to_string();
        let contents = self.parse_content(rule.next().unwrap());

        result.insert(name,contents);
        result
    }

    fn parse_content(&self, input: &str) -> HashMap<String,i32> {
        let mut result = HashMap::new();

        if input == " no other bags." {
            let mut no_other_bags = HashMap::new();
            no_other_bags.insert("no other bags".to_string(), 0);
            return no_other_bags;
        }

        input.split(",").for_each(|content| {
            let prepared_content = self.prepare_content(content);

            let mut content_parts = prepared_content.split_whitespace();
            let num = content_parts.next().unwrap().parse::<i32>().unwrap();
            let mut color = String::from(content_parts
                .next()
                .unwrap());
            color = color + " " + content_parts.next().unwrap();
            result.insert(color, num);
        });
        result
    }

    fn prepare_content(&self, content: &str) -> String {
        content
            .replace(".", "")
            .replace("bags", "bag")
            .replace("bag", "")
    }

    fn new() -> Bags {
        let rules: HashMap<String,HashMap<String,i32>> = HashMap::new();
        Bags { rules }
    }
  /*
    fn check_all_bags(&self, color: &str) {
        let mut num = 0;
        for current_color in rules.keys() {
            num = check_bag(color, rules.get(current_color));
        }
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules() -> String {
        String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.")
    }

    #[test]
    fn all_rules() {
        let mut bags = Bags::new();
        bags.parse_rules(&rules());
        println!("{:?}", bags.rules);
    }

    #[test]
    fn single_bag_line() {
        let bags = Bags::new();
        let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let parsed_rule = bags.parse_line(rule);

        let mut result: HashMap<String,HashMap<String,i32>> = HashMap::new();
        let mut contents: HashMap<String,i32> = HashMap::new();
        contents.insert("bright white".to_string(), 1);
        contents.insert("muted yellow".to_string(), 2);

        result.insert("light red".to_string(), contents);
        assert_eq!(parsed_rule, result);
    }

    #[test]
    fn bag_contents() {
        let bags = Bags::new();
        let content_rule = " 1 bright white bag, 2 muted yellow bags.";
        let parsed_content = bags.parse_content(content_rule);

        let mut contents: HashMap<String,i32> = HashMap::new();
        contents.insert("bright white".to_string(), 1);
        contents.insert("muted yellow".to_string(), 2);

        assert_eq!(parsed_content, contents);
    }

    #[test]
    fn prep_content() {
        let bags = Bags::new();
        let content_rule = " 1 bright white bag, 2 muted yellow bags.";
        let prepared_content = bags.prepare_content(content_rule);
        assert_eq!(prepared_content," 1 bright white , 2 muted yellow ");
    }

    #[test]
    fn allowed_bags_single() {
        let bags = Bags::new();
        let content_rule = " 2 shiny gold bags, 9 faded blue bags.";
        let allowed_bags = bags.parse_content(content_rule);

        bags.test_color(color: &str);
    }
}