use std::collections::HashMap;
use std::collections::HashSet;

pub fn orbit_count(orbits_string: String) -> HashMap<String, Vec<String>> {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    let mut left: HashSet<&str> = HashSet::new();
    let mut right: HashSet<&str> = HashSet::new();

    for orbit in orbits_string.split(&"\n") {
        let offset = orbits_string.find(')').unwrap();
        let planets = orbit.split_at(offset);
        orbits
            .entry(planets.0.to_owned())
            .or_insert_with(Vec::new)
            .push(planets.1.to_owned());
        left.insert(planets.0);
        right.insert(planets.1);
    }
    let center = left
        .difference(&right)
        .map(|x| (*x).to_string())
        .collect::<Vec<String>>()[0]
        .clone();

    // Iterate starting at the origin and count every planet that orbits the current planet.
    println!("{}", center);

    let mut seen_planets: Vec<String> = Vec::new();
    let mut current_planet = center;
    for (orbitted, orbitting) in orbits {
        for planet in orbitting
            seen_planets.push(planet)

        end
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
