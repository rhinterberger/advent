use std::fs;
use std::collections::{HashMap};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut reactions: HashMap<String, (Element, Vec<Element>)> = HashMap::new();
    let mut diffs: HashMap<String, i64> = HashMap::new();

    for line in filecontents.lines() {
        let mut s = line.split("=>");

        let needed_elements = s.next().unwrap().split(',').map(|element| {
            let mut eldata = element.trim().split(' ');
            let amount = eldata.next().map(|num| num.parse::<i64>().unwrap()).unwrap();
            let name = String::from(eldata.next().unwrap());
            Element { name, amount }
        }).collect::<Vec<Element>>();

        let produced_element: Element = s.next().unwrap().split(',').map(|element| {
            let mut eldata = element.trim().split(' ');
            let amount = eldata.next().map(|num| num.parse::<i64>().unwrap()).unwrap();
            let name = String::from(eldata.next().unwrap());
            Element { name, amount}
        }).next().unwrap();

        reactions.insert(produced_element.name.clone(), (produced_element, needed_elements));
    }

    println!("Ore needed per FUEL: {}", refine(&Element {name:String::from("FUEL"), amount: 1}, &reactions, &mut diffs));
    diffs.clear();

    let mut increase = 1000000;
    let mut amount= increase;
    while increase != 0 {
        while 1000000000000 - refine(&Element {name:String::from("FUEL"), amount}, &reactions, &mut diffs) >= 0 {
            amount += increase;
            diffs.clear();
        }
        amount -= increase;
        increase = increase / 10;
    }
    println!("Max FUEL: {}", amount);
}

#[derive(Debug, Clone)]
struct Element {
    name: String,
    amount: i64,
}

fn refine(start_reaction: &Element, reactions: &HashMap<String,(Element,Vec<Element>)>, diffs: &mut HashMap<String, i64>  ) -> i64
{
    let (yielded, components) = &reactions[&start_reaction.name];

    let mut produce: Element = Element { amount : start_reaction.amount, name : start_reaction.name.clone()};
    if diffs.contains_key(&start_reaction.name) {
        produce.amount = start_reaction.amount - *diffs.get(&start_reaction.name).unwrap();
        diffs.remove(&start_reaction.name);
    }

    let production_factor = (produce.amount as f64 / yielded.amount as f64).ceil();
    let overproduction = (yielded.amount as f64 * production_factor - produce.amount as f64) as i64;
    if overproduction != 0 {
        diffs.insert(yielded.name.clone(), overproduction);
    }

    let mut ore = 0;
    for el in components {
        if el.name == String::from("ORE") {
            return el.amount * production_factor as i64;
        }
        let produce = Element { amount: (el.amount as f64 * production_factor) as i64, name: el.name.clone() };
        ore = ore + refine(&produce, reactions, diffs);
    }
    ore
}