use std::fs;
use std::collections::HashMap;

fn main() {
    let raw_contents = fs::read_to_string("test_input.txt").expect("File not found");
    let contents = raw_contents.trim(); // remove trailing \n

    let orbits = input_to_orbits(contents);

    // Count the distance to COM for each individual planet.
    let num_orbits = orbits.keys().clone().fold(0, |sum, planet| {
        sum + count_to_center(orbits.clone(), *planet, 0)
    });

    println!("{}", num_orbits);
}

fn input_to_orbits(input: &str) -> HashMap<&str, &str> {
    input.split('\n').fold(HashMap::new(), |mut m, line| {
        let pair = line.split(')').into_iter();
        m.insert(pair.clone().nth(1).unwrap(), pair.clone().nth(0).unwrap()); m
    })
}

fn count_to_center(orbits: HashMap<&str, &str>, planet: &str, sum: usize) -> usize {
    match orbits.get(planet) {
        Some(&"COM") => sum + 1,
        Some(p) => count_to_center(orbits.clone(), p, sum + 1),
        None => unreachable!(),
    }
}
