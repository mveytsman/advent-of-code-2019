#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Moon {
    pub position: (i32, i32, i32),
    pub velocity: (i32, i32, i32),
}
impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        }
    }
    fn apply_graviy(&mut self, other: &Moon) {
        self.velocity.0 += (other.position.0 - self.position.0).signum();
        self.velocity.1 += (other.position.1 - self.position.1).signum();
        self.velocity.2 += (other.position.2 - self.position.2).signum();
    }

    fn apply_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn kinetic_energy(self) -> i32 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }
    fn potential_energy(self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
    pub fn total_energy(self) -> i32 {
        self.kinetic_energy() * self.potential_energy()
    }
}

pub fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            let m = moons[j];
            moons.get_mut(i).unwrap().apply_graviy(&m);
        }
    }
    for moon in moons {
        moon.apply_velocity();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_gravity() {
        let mut moon1 = Moon::new(1, 0, 2);
        let mut moon2 = Moon::new(2, -10, -7);

        moon1.apply_graviy(&moon2);
        moon2.apply_graviy(&moon1);

        assert_eq!((1, -1, -1), moon1.velocity);
        assert_eq!((-1, 1, 1), moon2.velocity);
    }
    #[test]
    fn test_step() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        step(&mut moons);
        assert_eq!(
            vec![
                Moon {
                    position: (2, -1, 1),
                    velocity: (3, -1, -1)
                },
                Moon {
                    position: (3, -7, -4),
                    velocity: (1, 3, 3)
                },
                Moon {
                    position: (1, -7, 5),
                    velocity: (-3, 1, -3)
                },
                Moon {
                    position: (2, 2, 0),
                    velocity: (-1, -3, 1)
                }
            ],
            moons


        );


	for i in 0..9 {
	    step(&mut moons);
	}

	assert_eq!(179,moons.iter().map(|m| m.total_energy()).sum());
    }
}
