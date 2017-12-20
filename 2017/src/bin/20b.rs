use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

struct Particle {
    px: i32,
    py: i32,
    pz: i32,
    vx: i32,
    vy: i32,
    vz: i32,
    ax: i32,
    ay: i32,
    az: i32,
}

fn solve(input: &str) -> usize {
    let mut particles: Vec<Particle> = input.split("\n").map(|x| {
        let x = x.replace("p=<", "").replace(">, v=<", ",").replace(">, a=<", ",").replace(" ", "").replace(">", "");
        let mut iter = x.split(",").map(|x| i32::from_str_radix(x, 10).unwrap());
        Particle{
            px: iter.next().unwrap(),
            py: iter.next().unwrap(),
            pz: iter.next().unwrap(),
            vx: iter.next().unwrap(),
            vy: iter.next().unwrap(),
            vz: iter.next().unwrap(),
            ax: iter.next().unwrap(),
            ay: iter.next().unwrap(),
            az: iter.next().unwrap(),
        }
    }).collect();

    // 150 iterations upper bound is a guess that is valid for my (and probably
    // everyone's) puzzle input.
    for i in 0..150 {
        let mut positions_seen = HashMap::new();

        for particle in particles.iter() {
            let pos = (particle.px, particle.py, particle.pz);

            let prev_count = *positions_seen.get(&pos).unwrap_or(&0);
            positions_seen.insert(pos, prev_count + 1);
        }

        particles.retain(|x| positions_seen.get(&(x.px, x.py, x.pz)) == Some(&1));

        for mut particle in particles.iter_mut() {
            particle.vx += particle.ax;
            particle.vy += particle.ay;
            particle.vz += particle.az;

            particle.px += particle.vx;
            particle.py += particle.vy;
            particle.pz += particle.vz;
        }

        //println!("{}: {}", i, particles.len());
    }

    particles.len()
}

#[test]
fn test() {
    assert_eq!(solve("p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>\np=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>\np=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>\np=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>"), 1);
}

fn main() {
    let mut f = File::open("input/20").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
