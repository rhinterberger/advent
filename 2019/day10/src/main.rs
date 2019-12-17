use std::{fs};
use std::collections::HashMap;

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    let mut map : Vec<Vec<u8>> =  Vec::new();
    for line in filecontents.lines() {
        map.push(Vec::from(line));
    }

    let asteroids:Vec<Asteroid> = get_asteroids(&map);

    let mut max_visible_asteroids: HashMap<i32,Asteroid> = HashMap::new();
    let mut best_asteroid=Asteroid { x:0, y:0, angle:0, distance:0};
    for a1 in &asteroids {
        let visible_asteroids = get_visible_asteroids(a1, &asteroids);
        if max_visible_asteroids.len() != max_visible_asteroids.len().max(visible_asteroids.len()) {
            max_visible_asteroids = visible_asteroids;
            best_asteroid = *a1;
        }
    }

    println!("{:?} {}", best_asteroid, max_visible_asteroids.len());

    // Part 2 Works only if visible_asteroids > 200 (no second laser rotation)
    let mut q1: Vec<&Asteroid> = max_visible_asteroids.iter().filter_map(|(_key, asteroid)| {
        if asteroid.angle <= 9000 && asteroid.angle >= 0 { Some(asteroid) } else { None }
    }).collect();
    let mut q2: Vec<&Asteroid> = max_visible_asteroids.iter().filter_map(|(_key, asteroid)| {
        if asteroid.angle < 0 && asteroid.angle >= -18000 { Some(asteroid) } else { None }
    }).collect();
    let mut q3:Vec<&Asteroid> = max_visible_asteroids.iter().filter_map(|(_key, asteroid)| {
        if asteroid.angle > 9000 && asteroid.angle <=18000 { Some(asteroid) } else { None }
    }).collect();

    q1.sort_by(|a1, a2| a2.angle.partial_cmp(&a1.angle).unwrap());
    q2.sort_by(|a1, a2| a2.angle.partial_cmp(&a1.angle).unwrap());
    q3.sort_by(|a1, a2| a2.angle.partial_cmp(&a1.angle).unwrap());

    q2.append(&mut q3);
    q1.append(&mut q2);

    println!("{:?} : {}", q1[199], q1[199].x*100 + q1[199].y);
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Asteroid {
    x: usize,
    y: usize,
    angle: i32,
    distance: i32
}
impl Asteroid {
    fn angle(&self, other: &Asteroid) -> i32 {
        // Hash for f64 is not implemented -> use i32 for angle * 100
        ((self.y as f64 - other.y as f64).atan2(other.x as f64 - self.x as f64).to_degrees() * 100.0) as i32
    }

    fn distance(&self, other: &Asteroid) -> i32
    {
        (other.x as i32 - self.x as i32).abs() + (other.y as i32 - self.y as i32).abs()
    }
}

fn get_asteroids(map: &Vec<Vec<u8>>) -> Vec<Asteroid>
{
    let mut asteroids: Vec<Asteroid> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' as u8 {
                asteroids.push(Asteroid { x, y, angle: 0, distance: 0});
            }
        }
    }
    asteroids
}

fn get_visible_asteroids(a1: &Asteroid, asteroids: &Vec<Asteroid>) -> HashMap<i32,Asteroid>
{
    let mut visible_asteroids:HashMap<i32,Asteroid>= HashMap::new();

    for a2 in asteroids {
        let current_asteroid = Asteroid  { x: a2.x, y: a2.y, angle: a1.angle(a2), distance: a1.distance(a2) };

        if visible_asteroids.contains_key(&current_asteroid.angle) && visible_asteroids[&current_asteroid.angle].distance > current_asteroid.distance {
            visible_asteroids.insert(current_asteroid.angle, current_asteroid);
        } else {
            visible_asteroids.insert(current_asteroid.angle, current_asteroid);
        }
    }
    visible_asteroids
}