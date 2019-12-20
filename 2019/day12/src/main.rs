use std::ops::{Add, Sub};
use itertools::Itertools;
use std::collections::{ HashSet};
use num::integer::lcm;

fn main() {
    let mut moons = [
        Moon { position: Point{x:-15, y:1, z:4}, velocity: Point {x:0, y:0 ,z:0} },
        Moon { position: Point{x:1, y:-10, z:-8}, velocity: Point {x:0, y:0 ,z:0} },
        Moon { position: Point{x:-5, y:4, z:9}, velocity: Point {x:0, y:0 ,z:0} },
        Moon { position: Point{x:4, y:6, z:-2}, velocity: Point {x:0, y:0 ,z:0} },
    ];

    // State (coordinates per moon) per axis
    let mut seen : [HashSet<[(i64, i64);4]>;3] = [HashSet::new(), HashSet::new(), HashSet::new()];
    let mut period:[i64;3] = [0,0,0];

    let mut step = 0i64;

    loop {
        moons = iterate(&mut moons);

        let mut state = [[(0i64,0i64);4];3];

        for i in 0..moons.len() {
            state[0][i] = moons[i].get_axis('x');
            state[1][i] = moons[i].get_axis('y');
            state[2][i] = moons[i].get_axis('z');
        }

        for i in 0..period.len() {
            if period[i] == 0 {
                match seen[i].contains(&state[i]) {
                    true => { period[i] = step; },
                    _ =>  { seen[i].insert(state[i]); },
                }
            }
        }

        if period[0] != 0 && period[1]!=0 && period[2] !=0 {break;}
        step += 1;

        if step == 1000 {
            println!("Part1: {}", moons[0].e_tot() + moons[1].e_tot() + moons[2].e_tot() +moons[3].e_tot());
        }
    }
    println!("Part2: {}", lcm(period[0], lcm(period[1], period[2])));
}

fn iterate(moons: &mut [Moon;4]) -> [Moon;4]
{
    for combi in (0..4usize).combinations(2) {
        let g = moons[combi[0]].gravity(&moons[combi[1]]);
        moons[combi[0]].velocity = moons[combi[0]].velocity.sub(g);
        moons[combi[1]].velocity = moons[combi[1]].velocity.add(g);
    }

    for i in 0..moons.len() {
        moons[i].position = moons[i].position.add(moons[i].velocity);
    }
    *moons
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Moon {
    position: Point, velocity: Point
}
impl Moon {
    fn e_pot(&self) -> i64
    {
        self.position.x.abs()+self.position.y.abs()+self.position.z.abs()
    }

    fn e_kin(&self) -> i64
    {
        self.velocity.x.abs()+self.velocity.y.abs()+self.velocity.z.abs()
    }

    fn e_tot(&self) -> i64
    {
        self.e_kin()*self.e_pot()
    }

    fn gravity (self, other: &Self) -> Point
    {
        Point {
            x: (self.position.x - other.position.x).signum(),
            y: (self.position.y - other.position.y).signum(),
            z: (self.position.z - other.position.z).signum()
        }
    }

    fn get_axis(&self, axis: char) -> (i64, i64)
    {
        match axis {
            'x' => (self.position.x, self.velocity.x),
            'y' => (self.position.y, self.velocity.y),
            'z' => (self.position.z, self.velocity.z),
            _ => (-1i64,-1i64)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i64, y: i64, z: i64,
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y,  z: self.z + other.z }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y,  z: self.z - other.z }
    }
}