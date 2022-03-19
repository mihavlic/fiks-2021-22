use std::{collections::BTreeMap, cmp::Ordering};

type Point = (i32, i32);

fn square_distance(p: Point, c: Point) -> i64 {
    // maximum 56 bits for the length, must use i64
    ((p.0 - c.0) as i64).pow(2) + ((p.1 - c.1) as i64).pow(2)
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

#[derive(Clone, Copy, Default)]
struct Data {
    len: [u32; 3],
    corners: [u8; 3],
    point: Point,
}

// when implementing all these functions, only the first length and the point is taken into account
// the length provides ordering while the point is unique and differentiates the different Data instances
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.len[0] == other.len[0] && self.point == other.point
    }
}
impl Eq for Data {}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.len[0].partial_cmp(&other.len[0])   {
            Some(Ordering::Equal) => {},
            other => return other.map(|o| o.reverse()),
        }

        self.point.partial_cmp(&other.point)
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.len[0].cmp(&other.len[0])   {
            Ordering::Equal => {},
            other => return other.reverse(),
        }

        self.point.cmp(&other.point)
    }
}

fn prepare_data(p: Point, n: i32, corners: &[Point; 4]) -> [Option<Data>; 3] {
    let mut i = 0;
    let mut len_corner = [(0, 0); 3];

    for_visible_corners(n, p, |j| {
        let dist = (square_distance(p, corners[j]) as f64).sqrt().round() as u32;
        len_corner[i] = (dist, j as u8);
        i += 1;
    });

    let mut k = 0;
    let mut out = [None; 3];
    for j in 0..i {
        // swap so that for each visible corner we have a copy with the proper length first
        // it is apparently fine to swap the same element into itself
        // 1. [A, B, C]
        // 2. [B, A, C]
        // 3. [C, A, B]
        len_corner.swap(0, j);
        let data = Data {
            len: len_corner.map(|(len, _)| len),
            corners: len_corner.map(|(_, corner)| corner),
            point: p,
        };
        out[k] = Some(data);
        k += 1;
    }
    out
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();
    
    let n = split.next().unwrap().parse::<i32>().unwrap();
    let q = split.next().unwrap().parse::<usize>().unwrap();

    let mut trees: [BTreeMap<Data, ()>; 4] = [
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

        let p = (x, y);

        match command {
            "+" => {
                let data = prepare_data(p, n, &corners);
                for d in data {
                    if let Some(d) = d {
                        trees[d.corners[0] as usize].insert(d, ());
                    }
                }
            },
            "-" => {
                // to search the map to remove the right entry we must either do a linear search (bad idea)
                // or remake the key so that we have the same ordering as the key we are looking for
                let data = prepare_data(p, n, &corners);

                // I really appreciate the symmetry of the implementation with the "+" command
                for d in data {
                    if let Some(d) = d {
                        trees[d.corners[0] as usize].remove(&d);
                    }
                }
            },
            "?" => {
                let corner_distance = corners.map(|c| ((c.0 - p.0).abs() + (c.1 - p.1).abs()) as u32);

                let mut max = 0;
                for i in 0..4 {
                    'search_edges: for (data, _) in trees[i].iter() {

                        // find the shortest path for the given point
                        // if it's not the first one, this edge wouldn't be taken and is ignored
                        // also zero is invalid unitialized value so is ignored
                        let first = data.len[0] + corner_distance[data.corners[0] as usize];
                        for i in 1..3 {
                            if data.len[i] != 0 && (data.len[i] + corner_distance[data.corners[i] as usize]) < first {
                                continue 'search_edges;
                            }
                        }

                        // everything passed, we can continue onto the other edges 
                        max = max.max(first);
                        break;
                    }
                }

                println!("{}", max as u32);
            }
            _ => unreachable!()
        }
    }
}