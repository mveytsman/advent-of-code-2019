use std::collections::HashMap;
use std::collections::VecDeque;

pub fn count_orbits(input: &str) -> i32 {
    let orbits = parse_input(input);
    let mut count = 0;

    let mut queue: VecDeque<(&str, i32)> = VecDeque::new();
    for planet in orbits.get("COM").unwrap() {
        queue.push_back((planet, 1));
    }

    while !queue.is_empty() {
        let (planet, c) = queue.pop_front().unwrap();

        count += c;
        let planets = orbits.get(planet);
        match planets {
            Some(planets) => {
                for p in planets {
                    queue.push_back((p, c + 1))
                }
            }
            None => (),
        }
    }
    while queue.len() > 0 {}
    count
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    let space_objects = input.split("\n");
    for line in space_objects {
        let orbit: Vec<&str> = line.split(")").collect();
        if orbit.len() == 2 {
            match orbits.get_mut(orbit[0]) {
                Some(planets) => {
                    planets.push(orbit[1]);
                }
                None => {
                    orbits.insert(orbit[0], vec![orbit[1]]);
                }
            }
        }
    }
    orbits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5() {
        assert_eq!(
            42,
            count_orbits("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")
        )
    }
}
