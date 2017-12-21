use std::io::Read;
use std::fs::File;

#[derive(Debug)]
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
    index: usize,
}

fn solve(input: &str) -> usize {
    let particles: Vec<Particle> = input.split("\n").enumerate().map(|(index, x)| {
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
            index: index,
        }
    }).collect();

    // This is incorrect for particles with equal |a| that have any component of
    // v change sign, and similarly for particles with equal |a| and |v| that
    // have any component of p change sign.
    //
    // Thanks to https://github.com/xocolatl for pointing this out.
    particles.iter().min_by_key(|x| (
        x.ax.abs() + x.ay.abs() + x.az.abs(),
        x.vx.abs() + x.vy.abs() + x.vz.abs(),
        x.px.abs() + x.py.abs() + x.pz.abs()
    )).unwrap().index
}

#[test]
fn test() {
    assert_eq!(solve("p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>"), 0);
}

fn main() {
    let mut f = File::open("input/20").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("{}", solve(input.trim()));
}
