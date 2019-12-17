use std::fs;
use std::collections::{ HashMap};

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();
    let orbit_split  = filecontents.lines().map(|orbit_str| orbit_str.split(")"));

    let mut orbit_tree = HashMap::new();
    for mut orbit in orbit_split
    {
        let direct_orbits = orbit_tree.entry(orbit.next().unwrap()).or_insert(Vec::new());
        direct_orbits.push(orbit.next().unwrap());
    }

    let mut count =0;
    for object in orbit_tree.keys() {
        count += indirect_orbits( &orbit_tree[object], &orbit_tree);
    }
    println!("all orbits: {}", count);

    let path_length = intersect_paths(&get_path("YOU", &orbit_tree), &get_path("SAN", &orbit_tree));
    println!("lenght: {}", path_length);
}

fn indirect_orbits(orbits: &Vec<&str>, orbit_tree: &HashMap<&str, Vec<&str>>) -> i32
{
    let mut indirect_count =  orbits.len() as i32;
    for i in 0..orbits.len() {
        if orbit_tree.contains_key(orbits[i])
        {
            let direct_orbits = &orbit_tree[orbits[i]];
            indirect_count += indirect_orbits(&direct_orbits, &orbit_tree);
        }
    }
    indirect_count
}

fn get_path<'a>(start: &str, orbit_tree: &'a HashMap<&'a str, Vec<&str>>) -> Vec<&'a str>
{
    let mut search = start;
    let mut path: Vec<&str> = Vec::new();

    while search != "COM" {
        for (key,val) in orbit_tree.iter() {
            if val.contains(&search)
            {
                search = key;
                path.push(key);
            }
        }
    }
    path
}

fn intersect_paths(start: &Vec<&str>, end:&Vec<&str>) -> usize
{
    let path_length = 0;

    for j in 0..start.len() {
        for i in 0..end.len() {
            if end[i] == start[j]  {
               return i+j
            }
        }
    }
    path_length
}