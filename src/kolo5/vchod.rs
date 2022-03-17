#![feature(map_first_last)]
#![feature(bench_black_box)]
#![allow(non_snake_case)]
use std::collections::{BTreeMap, btree_map::Entry};

use smallvec::{SmallVec, smallvec};

type Point = (i32, i32);
fn square_distance(p: Point, c: Point) -> i64 {
    // maximum 56 bits for the length, must use i64
    ((p.0 - c.0) as i64).pow(2) + ((p.1 - c.1) as i64).pow(2)
}
// check a 'house' against all corners of the zoo combined with the path to the door, return the minimum distance
// since the tourists always take the optimal path
fn point_distance(p: Point, n: i32) -> u32 {
    let corners = [
        (0, 0),
        (n, 0),
        (n, n),
        (0, n),
    ];
    
    let mut min = i64::MAX;
    for_visible_corners(n, p, |i| {
        let dist = square_distance(p, corners[i]);
        min = min.min(dist);
    });

    let a = (min as f32).sqrt();
    let b = roundf(a);
    b as u32
}

// check a 'house' against all corners of the zoo combined with the path to the door, return the minimum distance
// since the tourists always take the optimal path
fn point_distance_with_corners(p: Point, n: i32, corner: &[u32]) -> u32 {
    let corners = [
        (0, 0),
        (n, 0),
        (n, n),
        (0, n),
    ];
    
    let mut min = u32::MAX;
    for_visible_corners(n, p, |i| {
            // let dist = roundf(square_distance(p, corners[i]).sqrt()) as u32 + corner[i];
            let a = (square_distance(p, corners[i]) as f32).sqrt() ;
            let b = corner[i];
            let c = roundf(a) as u32 + b;
            min = min.min(c);
    });

    min
}

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    string
}

fn for_visible_corners<F: FnMut(usize)>(n: i32, p: Point, mut fun: F) {
    //  3 |              | 2
    //  --(0, n)----(n, n)--
    //    |              |
    //    |              |            
    //  --(0, 0)----(n, 0)--
    //  0 |              | 1
    
    let mut mask = [false; 4];
    if p.0 < 0 {
        mask[0] = true;
        mask[3] = true;
    } else if p.0 > n {
        mask[1] = true;
        mask[2] = true;
    }
    if p.1 < 0 {
        mask[0] = true;
        mask[1] = true;
    } else if p.1 > n {
        mask[2] = true;
        mask[3] = true;
    }

    for i in 0..4 {
        if mask[i] == true {
            fun(i);
        }
    }
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();
    
    let n = split.next().unwrap().parse::<i32>().unwrap();
    let q = split.next().unwrap().parse::<usize>().unwrap();
    
    let mut trees: [BTreeMap<i64, u32>; 4] = [
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    ];

    let corners = [
        (0, 0),
        (n, 0),
        (n, n),
        (0, n),
    ];

    for _ in 0..q {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let command = split.next().unwrap();
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        match command {
            "+" => {
                for_visible_corners(n, (x, y), |i| {
                    let dist = square_distance((x,y), corners[i]);
                    match trees[i].entry(dist) {
                        Entry::Vacant(v) => {v.insert(1);},
                        Entry::Occupied(mut o) => *o.get_mut() += 1,
                    }
                })           
            },
            "-" => {
                for_visible_corners(n, (x, y), |i| {
                    let dist = square_distance((x,y), corners[i]);
                    match trees[i].entry(dist) {
                        Entry::Vacant(_) => unreachable!(),
                        Entry::Occupied(mut o) => {
                            let refcount = o.get_mut();
                            if *refcount == 1 {
                                o.remove_entry();
                            } else {
                                *refcount -= 1;
                            }
                        },
                    }
                })
            },
            "?" => {
                let corner_distance = corners.map(|c| ((c.0 - x).abs() + (c.1 - y).abs()) as u32);

                let mut max = 0f32;
                for (i, tree) in trees.iter_mut().enumerate() {
                    let last = match tree.last_entry() {
                        Some(e) => *e.key(),
                        None => continue,
                    };
                    let dist = (last as f32).sqrt() + corner_distance[i] as f32;

                    max = max.max(dist);
                }

                println!("{}", max.round() as u32);
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