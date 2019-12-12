use itertools::*;

struct Moon {
    pos: (isize,isize,isize),
    vel: (isize,isize,isize),
}

impl Moon {
    fn energy(&self) -> isize {
        // Then, it might help to calculate the total energy in the system. 
        
        // A moon's potential energy is the sum of the absolute values of its x, y, and z position coordinates. 
        let pot_e = self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs();
        // A moon's kinetic energy is the sum of the absolute values of its velocity coordinates.
        let kin_e = self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs();
        // The total energy for a single moon is its potential energy multiplied by its kinetic energy.         
        pot_e * kin_e
    }
}

fn energy(moons: &[Moon]) -> isize {
    moons.iter().map(|m| m.energy()).sum()
}

fn step(moons: &mut [Moon]) {
    use std::cmp::Ordering::*;

    // Simulate the motion of the moons in time steps. Within each time step, first update the velocity of every moon by applying gravity. Then, once all moons' velocities have been updated, update the position of every moon by applying velocity. Time progresses by one step once all of the positions are updated.

    // To apply gravity, consider every pair of moons. On each axis (x, y, and z), the velocity of each moon changes by exactly +1 or -1 to pull the moons together. For example, if Ganymede has an x position of 3, and Callisto has a x position of 5, then Ganymede's x velocity changes by +1 (because 5 > 3) and Callisto's x velocity changes by -1 (because 3 < 5). However, if the positions on a given axis are the same, the velocity on that axis does not change for that pair of moons.

    for (a,b) in (0..moons.len()).tuple_combinations() {
        assert!(a<b);
        let (a,b) = {
            let (group1,group2) = moons.split_at_mut(b);
            (&mut group1[a],&mut group2[0])
        };
        fn gravitate(a: &mut isize, b: &mut isize, va: &mut isize, vb: &mut isize) {
            match a.cmp(&b) {
                Greater => {
                    *va -= 1;
                    *vb += 1;
                },
                Less => {
                    *va += 1;
                    *vb -= 1;
                }
                _ => ()
            }
        }
        gravitate(&mut a.pos.0, &mut b.pos.0, &mut a.vel.0, &mut b.vel.0);
        gravitate(&mut a.pos.1, &mut b.pos.1, &mut a.vel.1, &mut b.vel.1);
        gravitate(&mut a.pos.2, &mut b.pos.2, &mut a.vel.2, &mut b.vel.2);
    }

    // Once all gravity has been applied, apply velocity: simply add the velocity of each moon to its own position. For example, if Europa has a position of x=1, y=2, z=3 and a velocity of x=-2, y=0,z=3, then its new position would be x=-1, y=2, z=6. This process does not modify the velocity of any moon.
    for m in moons.iter_mut() {
        m.pos.0 += m.vel.0;
        m.pos.1 += m.vel.1;
        m.pos.2 += m.vel.2;
    }
}

fn main() {
    /* Each moon has a 3-dimensional position (x, y, and z) and a 3-dimensional velocity. The position of each moon is given in your scan; the x, y, and z velocity of each moon starts at 0.*/    
    let mut moons : Vec<_> = [
        (17,  -12, 13),
        (2,  1, 1),
        (-1,  -17, 7),
        (12,  -14, 18),
    ].iter().map(|&pos| Moon {pos, vel: (0,0,0)}).collect();

    for _ in 0..1000 {
        step(&mut moons);
    }

    println!("Total energy after 1000 steps: {}", energy(&moons));    
}
