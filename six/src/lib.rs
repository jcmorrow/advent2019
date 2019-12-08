use std::collections::HashMap;
use std::collections::HashSet;

pub fn parsed_orbits(orbits_string: String) -> (HashMap<String, Vec<String>>, String) {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    let mut left: HashSet<String> = HashSet::new();
    let mut right: HashSet<String> = HashSet::new();

    for orbit in orbits_string.split(&" ") {
        let offset = orbit.find(')').unwrap();
        let planets = orbit.split_at(offset);
        let mut left_planet = planets.0.to_owned();
        let mut right_planet = planets.1.to_owned().split_at(1).1.to_owned();
        orbits
            .entry(left_planet.clone())
            .or_insert_with(Vec::new)
            .push(right_planet.clone());
        left.insert(left_planet);
        right.insert(right_planet);
    }

    let center = left
        .difference(&right)
        .map(|x| (*x).to_string())
        .collect::<Vec<String>>()[0]
        .clone();

    (orbits, center)
}

pub fn orbit_count(
    orbits: &HashMap<String, Vec<String>>,
    current_planet: String,
    count: i32,
) -> i32 {
    let orbitting = match orbits.get(&current_planet) {
        Some(x) => x.clone(),
        None => return count,
    };
    let mut total_orbitting = 0;
    for planet in orbitting {
        total_orbitting += orbit_count(orbits, planet, count + 1);
    }
    count + total_orbitting
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let orbit_string = "COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L".to_owned();
        let orbits = parsed_orbits(orbit_string);
        assert_eq!(orbit_count(&orbits.0, orbits.1, 0), 42);
    }
}
