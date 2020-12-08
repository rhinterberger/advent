use std::collections::HashMap;
use std::fs;

fn main() {

    let input = read_input("input.txt");
    let all_answers = split_to_groups(&input);

    let number_of_answers = all_answers
        .iter()
        .fold(0, |num_answers, group| num_answers + group.answers.len());
    println!("{}", number_of_answers);

    let number_of_answers = all_answers
        .iter()
        .fold(0, |num_answers, group| num_answers + group.answer_every_person());
    println!("{}", number_of_answers);
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
}

fn split_to_groups(input: &str) -> Vec<Group> {
    let all_answers:Vec<Group> = input.split("\n\n")
        .map(parse_group)
        .collect();
    all_answers
}

fn parse_group(input: &str) -> Group {
    let mut group_answers = Group::new();
    group_answers.group_answers(input);
    group_answers
}

struct Group {
    answers: HashMap<char,i32>,
    members: i32
}
impl Group {
    fn new() -> Group {
        Group { answers: HashMap::new(), members: 0 }
    }

    fn group_answers(self: &mut Self, input: &str) {
        input.lines().for_each(|person_answer| self.person_answers(person_answer))
    }

    fn person_answers(self: &mut Self, input: &str) {
        for answer in input.chars() {
            self.update_answers_table(answer);
        };
        self.members = self.members +1 ;
    }

    fn update_answers_table(self: &mut Self, answer: char) {
        let mut new_num = 1;
        match self.answers.get(&answer)
        {
            Some(num_answers) => { new_num = num_answers +1; }
            None => (),
        };
        self.answers.insert(answer, new_num);
    }

    fn answer_every_person(self: &Self) -> i32 {
        self.answers.iter().fold(0, |num, answer| {
            if *answer.1 == self.members {
                return num + 1;
            }
            num
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_person() {
        let answers_input = "abc";
        let mut person = Group::new();
        person.person_answers(answers_input);
        assert_eq!(person.answers.len(), 3);
    }

    #[test]
    fn answer_group() {
        let answers_input = "a
b
c";
        let mut group = Group::new();

        group.group_answers(answers_input);
        assert_eq!(group.answers.len(), 3);
    }

    #[test]
    fn all_answers() {
        let answers_input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        let answers = split_to_groups(answers_input);
        assert_eq!(answers.len(), 5);
    }

    #[test]
    fn one_person_three_answers() {
        let answers_input = "abc";
        let mut group = Group::new();

        group.group_answers(answers_input);
        assert_eq!(group.answer_every_person(), 3);
    }

    #[test]
    fn three_persons_no_answer() {
        let answers_input = "a
b
c";
        let mut group = Group::new();

        group.group_answers(answers_input);
        assert_eq!(group.answer_every_person(), 0);
    }

    #[test]
    fn two_persons_one_answer() {
        let answers_input = "ab
ac";
        let mut group = Group::new();
        group.group_answers(answers_input);
        assert_eq!(group.answer_every_person(), 1);
    }

    #[test]
    fn four_persons_one_answer() {
        let answers_input = "a
a
a
a";
        let mut group = Group::new();
        group.group_answers(answers_input);
        assert_eq!(group.answer_every_person(), 1);
    }

    #[test]
    fn one_person_one_answer() {
        let answers_input = "b";
        let mut group = Group::new();
        group.group_answers(answers_input);
        assert_eq!(group.answer_every_person(), 1);
    }
}