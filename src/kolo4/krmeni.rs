#![allow(unused, non_snake_case)]
use std::collections::VecDeque;

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();

    let N = split.next().unwrap().parse::<usize>().unwrap();
    let Q = split.next().unwrap().parse::<usize>().unwrap();

    let mut pairs = vec![(0, 0); N];
    let mut buf = vec![0; (N - 1)*2];
    let mut country_neighbors = vec![(0, 0); N];

    for i in 0..(N - 1) {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        // indexovani v kodu od 0 oproti od 1 v zadani
        let u = (split.next().unwrap().parse::<usize>().unwrap() - 1) as u32;
        let v = (split.next().unwrap().parse::<usize>().unwrap() - 1) as u32;

        pairs[i] = (u, v);
        buf[u as usize] += 1;
        buf[v as usize] += 1;
    }

    let mut offset = 0;
    for i in 0..N {
        let neighbor_count = buf[i];
        country_neighbors[i] = (offset, 0);
        offset += neighbor_count as u32;
    }

    for &(u, v) in &pairs[0..(N - 1)] {
        // (offset, current neighbor count)
        let a = &mut country_neighbors[u as usize];
        buf[(a.0 + a.1) as usize] = v;
        a.1 += 1;
        
        let b = &mut country_neighbors[v as usize];
        buf[(b.0 + b.1) as usize] = u;
        b.1 += 1;
    }

    // for (i, &(offset, count)) in country_neighbors.iter().enumerate() {
    //     println!("{}: {:?}", i, &buf[(offset as usize)..((offset + count) as usize)]);
    // }
    
    let mut stack = VecDeque::new();
    let mut touched = vec![false; N];
    stack.push_back(0);
    touched[0] = true;

    let mut distance_from_czech = vec![0; N];
    map_bfs(&country_neighbors, &buf, &mut touched, &mut stack, u32::MAX, |node, step| {
        distance_from_czech[node as usize] = step;
    });

    // [distance, countries touched]
    let mut cumulative_distance = pairs;
    let mut zeme = Vec::new();

    for _ in 0..Q {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let K = split.next().unwrap().parse::<usize>().unwrap();

        let mut max_dist = 0;
        let mut sum_dist = 0;

        zeme.resize(K, 0);

        for i in 0..K {
            // indexovani v kodu od 0 oproti od 1 v zadani
            let zeme_index = split.next().unwrap().parse::<u32>().unwrap() - 1;
            zeme[i] = zeme_index;
            let dist = distance_from_czech[zeme_index as usize];
            
            max_dist = max_dist.max(dist);
            sum_dist += dist;
        }

        cumulative_distance.fill((0, 0));

        for &zeme_index in &zeme {
            stack.clear();
            touched.fill(false);
            stack.push_back(zeme_index);
            touched[zeme_index as usize] = true;

            map_bfs(&country_neighbors, &buf, &mut touched, &mut stack, max_dist, |node, step| {
                let dist = &mut cumulative_distance[node as usize];
                dist.0 += step;
                dist.1 += 1;
            });
        }

        let mut len = zeme.len() as u32;
        let mut min = u32::MAX;
        for &(dist, touch_count) in &cumulative_distance {
            if touch_count == len {
                min = min.min(dist);
            }
        }

        println!("{}", sum_dist as u32 - min);
    }
}

//                 (node_index, generation)
fn map_bfs<F: FnMut(u32, u32)>(
    country_neighbors: &Vec<(u32, u32)>,
    buf: &Vec<u32>,
    mut nodes_touched: &mut Vec<bool>,
    mut stack: &mut VecDeque<u32>,
    max_steps: u32,
    mut fun: F,
) {
    let N = nodes_touched.len();
    assert!(N == country_neighbors.len());

    let mut steps = 0;
    while !stack.is_empty() {
        if steps > max_steps {
            break;
        }

        let last = stack.len();
        for _ in 0..last {
            let pop = stack.pop_front().unwrap();
            fun(pop, steps);

            let (offset, count) = country_neighbors[pop as usize];
            for &x in &buf[(offset as usize)..((offset + count) as usize)] {
                if nodes_touched[x as usize] == false {
                    nodes_touched[x as usize] = true;
                    stack.push_back(x);
                }
            }
        }
        steps += 1;
    }
}
