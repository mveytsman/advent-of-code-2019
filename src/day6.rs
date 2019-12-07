use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Node<'a> {
    id: usize,
    planet: &'a str,
    parent: Option<usize>,
    depth: i32,
}

pub fn count_orbits(input: &str) -> i32 {
    let (_, _, count) = build_tree(input);
    count
}
pub fn count_transfers(input: &str, start: &str, end: &str) -> i32 {
    let (tree, index, _) = build_tree(input);
    let mut ancestors = HashSet::new();
    let mut i = index[start];
    loop {
        ancestors.insert(i);
        match tree[i].parent {
            Some(j) => i = j,
            None => break,
        }
    }

    i = index[end];
    let mut common_ancestor = 0;
    loop {
        if ancestors.contains(&i) {
            common_ancestor = i;
            break;
        }

        match tree[i].parent {
            Some(j) => i = j,
            None => break,
        }
    }
    dbg!(&tree[index[start]].depth) + dbg!(&tree[index[end]].depth) - 2*dbg!(&tree[common_ancestor].depth) -2
}
fn build_tree(input: &str) -> (Vec<Node>, HashMap<&str, usize>, i32) {
    let orbits = parse_input(input);
    let mut count = 0;

    let mut index: HashMap<&str, usize> = HashMap::new();
    index.insert("COM", 0);

    let mut queue = vec![Node {
        id: 0,
        planet: "COM",
        parent: None,
        depth: 0,
    }];
    let mut i = 1;
    let mut queue_pointer = 0;
    while queue_pointer < queue.len() {
        let node = &queue[queue_pointer].clone();

        count += node.depth;
        let planets = orbits.get(node.planet);
        match planets {
            Some(planets) => {
                for p in planets {
                    queue.push(Node {
                        id: i,
                        planet: p,
                        parent: Some(node.id),
                        depth: node.depth + 1,
                    });
                    index.insert(p, i);
                    i += 1;
                }
            }
            None => (),
        }
        queue_pointer += 1;
    }
    (queue, index, count)
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
        );
        assert_eq!(
            4,
            count_transfers(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
                "YOU",
                "SAN"
            )
        );
    }
}
