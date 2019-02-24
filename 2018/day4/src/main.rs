use std::env;
use std::process;
use std::fs;
use std::str;
use std::cmp::Ordering;

use chrono::prelude::*;
use std::collections::HashMap;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You have to provide an inputfile");
        process::exit(1);
    }

    let filename = &args[1];
    let mut filecontents = fs::read_to_string(filename).unwrap();

    let mut events:Vec<Event> = Vec::new();
    initialize_events(&mut events, &mut filecontents);

    let sleeper = find_sleepy_guard(&mut events);
    let minute = find_minute(&mut events, sleeper);

    println!("Sleeper {} * Minute {} = {}", sleeper, minute, minute * sleeper);
}

fn initialize_events(events: &mut Vec<Event>, filecontent: &mut String) {

    for line in filecontent.lines() {
       events.push(Event::new(line));
    }

    events.sort();
}

fn find_sleepy_guard(events: &mut Vec<Event>) -> i32 {

    let mut current_guard = 0;
    let mut asleep = 0;

    let mut guards = HashMap::new();

    for event in events {
        if event.guard != current_guard && event.guard != 0 {
            current_guard = event.guard;
            asleep=0;
            continue;
        }

        if event.awake == false && event.guard == 0 {
            asleep = event.ts;
            continue;
        }

        if event.awake == true && event.guard == 0 {
            let duration = event.ts - asleep;
            let guardentry = guards.entry(current_guard).or_insert(0);
            *guardentry += duration;
        }
    }

    let mut max_sleep = 0;
    let mut sleeper = 0;
    for g in guards {
        if g.1 >= max_sleep {max_sleep = g.1; sleeper=g.0;}
    }

    println!("Sleepiest Guard: {} with {} minutes of sleep",sleeper,max_sleep/60);
    sleeper
}

/*
Trigger 1. guard found
  Trigger 2 - falls asleep
  Clear Trigger 2 - awakes
    set sleeping minutes

Clear trigger 1 - other guard found
*/

fn find_minute(events: &mut Vec<Event>, sleeper: i32) -> i32
{
    let mut minutes = [0; 60];
    let mut found= false;
    let mut startsleep: i32 = -1;

    for event in events
    {
        if event.guard == sleeper && !found {
            found = true;

        }
        else if event.awake == false && found {
            startsleep = event.minute as i32;

        }
        else if event.awake == true && found && startsleep > 1 {
            for min in startsleep .. event.minute as i32 {
                minutes[min as usize ] += 1;
            }
            startsleep = -1;
        }
        else if event.guard != sleeper {
            found = false;
        }

    }
    let max = minutes.iter().max().unwrap();

    minutes.iter().position(|x| x==max).unwrap() as i32
}

#[derive(Eq, Debug)]
struct Event {
    ts: i64,
    guard: i32,
    awake: bool,
    minute: u32,
}

impl Ord for Event
{
    fn cmp(&self, other: &Event) -> Ordering {
        self.ts.cmp(&other.ts)
    }
}

impl PartialEq for Event
{
    fn eq(&self, other: &Event) -> bool {
        self.ts == other.ts
    }
}

impl PartialOrd for Event
{
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Event {
    fn new(ev_line: &str) -> Event {
        let ( date_string, event_string) = ev_line.split_at(18);

        let mut datetime = String::new();
        datetime.push_str(date_string);
        datetime.push_str(":00");
        datetime = datetime.replace("[","").replace("]","");

        let time = Utc.datetime_from_str(datetime.as_str(),"%Y-%m-%d %H:%M:%S");
        let ts= time.unwrap().timestamp();

        let mut awake= true;
        let mut guard = 0;

        match event_string.trim() {
            "wakes up" => awake = true,
            "falls asleep" => awake = false,
            _ => {
                guard = event_string.split_whitespace().nth(1).unwrap().replace("#","").parse::<i32>().unwrap();
            }
        }

        Event { ts, guard, awake, minute: time.unwrap().minute() }
    }
}
