use std::fs;

fn main()
{
    let filecontents = fs::read_to_string("input.txt").unwrap();

    println!("Part 1 : Total Fuel needed {}", calc_fuel(filecontents.to_string()));
    println!("Part 2 : Total Fuel needed {}", calc_total_fuel(filecontents.to_string()));
}

fn calc_fuel(filecontents: String) -> f32
{
    let mut sum:f32 = 0.0;

    let modules = filecontents.lines().into_iter().map(|valstr| valstr.parse::<f32>());
    for mass in modules
    {
        sum += fuel(mass.unwrap());
    }
    sum
}

fn fuel(mass:f32) -> f32
{
    (mass/3.0).floor()-2.0
}

fn calc_total_fuel(filecontents: String) -> f32
{
    let mut sum:f32 = 0.0;
    let modules = filecontents.lines().into_iter().map(|valstr| valstr.parse::<f32>());

    for module in modules {
        sum += real_fuel(module.unwrap());
    }
    sum
}

fn real_fuel(mass:f32) -> f32
{
    let mut sum :f32 = 0.0;
    let mut fuel_needed = fuel(mass);
    while fuel_needed > 0.0
    {
        sum += fuel_needed;
        fuel_needed = fuel(fuel_needed);
    }
    sum
}