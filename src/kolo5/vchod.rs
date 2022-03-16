#![feature(map_first_last)]
#![feature(bench_black_box)]
#![allow(non_snake_case)]
use std::collections::{BTreeMap, btree_map::Entry};

use smallvec::{SmallVec, smallvec};

type Point = (i32, i32);
fn square_distance(p: Point, c: Point) -> f32 {
    (((p.0 - c.0) as i64).pow(2) + ((p.1 - c.1) as i64).pow(2)) as f32
}
// check a 'house' against all corners of the zoo combined with the path to the door, return the minimum distance
// since the tourists always take the optimal path
fn point_distance(p: Point, N: i32) -> u32 {
    //     |        |               
    //   --3--------2--             
    //     |        |              
    //   --0--------1--             
    //     |        |     

    let corners = [
        (0, 0),
        (N, 0),
        (N, N),
        (0, N),
    ];
    let mut mask = [false; 4];

    if p.0 < 0 {
        mask[0] = true;
        mask[3] = true;
    } else if p.0 > N {
        mask[1] = true;
        mask[2] = true;
    }
    if p.1 < 0 {
        mask[0] = true;
        mask[1] = true;
    } else if p.1 > N {
        mask[2] = true;
        mask[3] = true;
    }
    
    let mut min = f32::MAX;
    for i in 0..4 {
        if mask[i] == true {
            let dist = square_distance(p, corners[i]);
            min = min.min(dist);
        }
    }

    let a = min.sqrt();
    let b = roundf(a);
    b as u32
}

// check a 'house' against all corners of the zoo combined with the path to the door, return the minimum distance
// since the tourists always take the optimal path
fn point_distance_with_corners(p: Point, N: i32, corner: &[u32]) -> u32 {
    //     |        |               
    //   --3--------2--             
    //     |        |              
    //   --0--------1--             
    //     |        |     

    let corners = [
        (0, 0),
        (N, 0),
        (N, N),
        (0, N),
    ];
    let mut mask = [false; 4];

    if p.0 < 0 {
        mask[0] = true;
        mask[3] = true;
    } else if p.0 > N {
        mask[1] = true;
        mask[2] = true;
    }
    if p.1 < 0 {
        mask[0] = true;
        mask[1] = true;
    } else if p.1 > N {
        mask[2] = true;
        mask[3] = true;
    }
    
    let mut min = u32::MAX;
    for i in 0..4 {
        if mask[i] == true {
            // let dist = roundf(square_distance(p, corners[i]).sqrt()) as u32 + corner[i];
            let a = square_distance(p, corners[i]).sqrt() ;
            let b = corner[i];
            let c = roundf(a) as u32 + b;
            min = min.min(c);
        }
    }

    min
}

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    string
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();
    
    let N = split.next().unwrap().parse::<i32>().unwrap();
    let Q = split.next().unwrap().parse::<usize>().unwrap();
    
    let mut tree: BTreeMap<u32, SmallVec<[Point; 1]>> = BTreeMap::new();

    for _ in 0..Q {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let command = split.next().unwrap();
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        match command {
            "+" => {
                let dist = point_distance((x, y), N);
                match tree.entry(dist) {
                    Entry::Vacant(vacant) => {vacant.insert(smallvec![(x, y)]);}
                    Entry::Occupied(mut occupied) => occupied.get_mut().push((x, y)),
                }                
            },
            "-" => {
                let dist = point_distance((x, y), N);
                match tree.entry(dist) {
                    Entry::Vacant(_) => unreachable!(),
                    Entry::Occupied(mut occupied) => {
                        let vec = occupied.get_mut();
                        let pos = vec.iter().position(|p| *p == (x,y)).unwrap();
                        vec.swap_remove(pos);

                        if vec.len() == 0 {
                            occupied.remove_entry();
                        }
                    },
                }
            },
            "?" => {
                let corner_distance = [
                    (0, 0),
                    (N, 0),
                    (N, N),
                    (0, N),
                ].map(|c| ((c.0 - x).abs() + (c.1 - y).abs()) as u32);

                let mut min = u32::MAX;
                let mut max = 0;
                for i in 0..4 {
                    let v = corner_distance[i];
                    min = min.min(v);
                    max = max.max(v);
                }

                let max_diff = max - min;
                let max = *tree.last_key_value().unwrap().0;

                let mut max_total_dist = 0;
                for (_, points) in tree.range(max.saturating_sub(max_diff)..=max) {
                    for p in points {
                        let dist = point_distance_with_corners(*p, N, &corner_distance);
                        max_total_dist = max_total_dist.max(dist);
                    }
                }

                println!("{}", max_total_dist);
            }
            _ => unreachable!()
        }
    }
}

const TOINT: f32 = 1.0 / f32::EPSILON;
pub fn roundf(mut x: f32) -> f32 {
    let i = x.to_bits();
    let e: u32 = i >> 23 & 0xff;
    let mut y: f32;

    if e >= 0x7f + 23 {
        return x;
    }
    if i >> 31 != 0 {
        x = -x;
    }
    if e < 0x7f - 1 {
        std::hint::black_box(x + TOINT);
        return 0.0 * x;
    }
    y = x + TOINT - TOINT - x;
    if y > 0.5f32 {
        y = y + x - 1.0;
    } else if y <= -0.5f32 {
        y = y + x + 1.0;
    } else {
        y = y + x;
    }
    if i >> 31 != 0 {
        -y
    } else {
        y
    }
}