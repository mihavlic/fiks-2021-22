#![allow(unused, non_snake_case)]
use std::{collections::VecDeque, sync::{atomic::{AtomicU32, Ordering}, Mutex, Arc}, thread::Thread, borrow::{BorrowMut, Borrow}, mem::size_of, hint::unreachable_unchecked};

use crossbeam_channel::RecvError;

struct Bitvec {
    buf: Vec<usize>
}

impl Bitvec {
    fn with_capacity(cap: usize) -> Self {
        fn div_up(a: usize, b: usize) -> usize {
            (a + b - 1) / b
        }

        let backing_size = div_up(cap, size_of::<usize>()*8);
        Self {
            buf: vec![0; backing_size]
        }
    }
    fn len(&self) -> usize {
        self.buf.len() * size_of::<usize>()*8
    }
    fn fill(&mut self, fill: bool) {
        let val = match fill {
            true => !0,
            false => 0,
        };

        self.buf.iter_mut().for_each(|int| *int = val);
    }
    fn set(&mut self, index: usize, val: bool) {
        let bits = size_of::<usize>()*8;
        let low = index / bits;
        let bit = index % bits;
    
        let val = (val as usize) << (bits - 1 - bit);
        self.buf[low] |= val;
    }
    fn get(&self, index: usize) -> bool {
        let bits = size_of::<usize>()*8;
        let low = index / bits;
        let bit = index % bits;
        
        let val = self.buf[low] >> (bits - 1 - bit);

        (val & 1) == 1
    }
    unsafe fn set_unchecked(&mut self, index: usize, val: bool) {
        let bits = size_of::<usize>()*8;
        let low = index / bits;
        let bit = index % bits;
    
        let val = (val as usize) << (bits - 1 - bit);
        *self.buf.get_unchecked_mut(low) |= val;
    }
    unsafe fn get_unchecked(&self, index: usize) -> bool {
        let bits = size_of::<usize>()*8;
        let low = index / bits;
        let bit = index % bits;
        
        let val = self.buf.get_unchecked(low) >> (bits - 1 - bit);

        (val & 1) == 1
    }
}


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
    
    let mut distance_from_czech = vec![0; N];
    {
        let mut stack = VecDeque::new();
        let mut touched = Bitvec::with_capacity(N);
        stack.push_back(0);
        touched.set(0, true);

        map_bfs(&country_neighbors, &buf, &mut touched, &mut stack, u32::MAX, |node, step| {
            distance_from_czech[node as usize] = step;
        });
    }

    // [distance, countries touched]
    
    let thread_count = std::env::args().nth(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
    
    let mut results = Arc::new((0..Q).map(|_| AtomicU32::new(0)).collect::<Vec<_>>());
    // (index, max_dist, sum_dist)
    let (send, recv) = crossbeam_channel::bounded::<(usize, u32, u32, Vec<u32>)>(64);
    
    let data = Arc::new(buf);
    let neighbors = Arc::new(country_neighbors);

    let mut threads = Vec::new();
    for _ in 0..thread_count {

        let neighbors = neighbors.clone();
        let data = data.clone();
        let results = results.clone();

        let mut stack = VecDeque::new();
        let mut touched = Vec::new();

        let recv_clone = recv.clone();
        let mut cumulative_dist = vec![(0, 0); N];

        let handle = std::thread::spawn(move || {
            loop {
                match recv_clone.recv() {
                    Ok((query_index, max_dist, sum_dist, zeme)) => {
                        let min = solve(&mut cumulative_dist, &zeme, &mut stack, &mut touched, &*neighbors, &data, max_dist);

                        let atomic: &AtomicU32 = results.get(query_index).unwrap();
<<<<<<< HEAD
=======
                        atomic.store(sum_dist - min, Ordering::SeqCst);
>>>>>>> 7cadb21 (Paralelize computation, use bitpacked structure)
                    },
                    // channel closed
                    Err(_) => break,
                }
            }
        });

        threads.push(handle);
    }
            
    let mut zeme = Vec::new();
    for i in 0..Q {
        let time = std::time::Instant::now();
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let K = split.next().unwrap().parse::<usize>().unwrap();

        let mut max_dist = 0;
        let mut sum_dist = 0;
        
        zeme.resize(K, 0);
        
        for j in 0..K {
            // indexovani v kodu od 0 oproti od 1 v zadani
            let zeme_index = split.next().unwrap().parse::<u32>().unwrap() - 1;
            zeme[j] = zeme_index;
            let dist = distance_from_czech[zeme_index as usize];
            
            max_dist = max_dist.max(dist);
            sum_dist += dist;
        }

        if K == 1 {
            results[i].store(sum_dist, Ordering::SeqCst);
        } else {
            let query = (i, max_dist, sum_dist, zeme.clone());
            send.send(query).unwrap();
        };
    }

    drop(send);

    for handle in threads {
        handle.join();
    }

    for result in &*results {
        println!("{}", result.load(Ordering::SeqCst));
    }
}

// cum (distance, countries touched), stack (country_index, origin_country_bitmap)
fn solve(cumulative_distance: &mut Vec<(u32, u32)>, zeme: &Vec<u32>, stack: &mut VecDeque<(u32, u32)>, touched: &mut Vec<Bitvec>, country_neighbors: &Vec<(u32, u32)>, neighbor_data: &Vec<u32>, max_dist: u32) -> u32 {
    let N = cumulative_distance.len();
    
    cumulative_distance.fill((0, 0));
    stack.clear();
    
    for bitvec in touched.iter_mut() {
        bitvec.fill(false);
    }

    if touched.len() < zeme.len() {
        touched.resize_with(zeme.len(), || Bitvec::with_capacity(N))
    }

    for (i, &zeme_index) in zeme.iter().enumerate() {
        stack.push_back((zeme_index, i as u32));
        touched[i].set(zeme_index as usize, true);
    }

    let len = zeme.len() as u32;
    let mut steps = 0;
    let min = unsafe {
        'min_loop: loop {
            let last = stack.len();
            for _ in 0..last {
                let (index, origin_country) = match stack.pop_front() {
                    Some(some) => some,
                    None => unreachable_unchecked(),
                };
                cumulative_distance.get_unchecked_mut(index as usize).0 += steps;
                cumulative_distance.get_unchecked_mut(index as usize).1 += 1;

                if cumulative_distance.get_unchecked(index as usize).1 == len {
                    break 'min_loop cumulative_distance.get_unchecked(index as usize).0;
                }

                let (offset, count) = *country_neighbors.get_unchecked(index as usize);
                for &x in &neighbor_data[(offset as usize)..((offset + count) as usize)] {
                    if touched.get_unchecked(origin_country as usize).get_unchecked(x as usize) == false {

                        touched.get_unchecked_mut(origin_country as usize).set_unchecked(x as usize, true);
                        stack.push_back((x, origin_country));
                    }
                }
            }
            steps += 1;
        }
    };

    min
}


//                 (node_index, generation)
fn map_bfs<F: FnMut(u32, u32)>(
    country_neighbors: &Vec<(u32, u32)>,
    neighbor_data: &Vec<u32>,
    mut nodes_touched: &mut Bitvec,
    mut stack: &mut VecDeque<u32>,
    max_steps: u32,
    mut fun: F,
) {
    let N = country_neighbors.len();
    assert!(nodes_touched.len() >= N);

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
            for &x in &neighbor_data[(offset as usize)..((offset + count) as usize)] {
                if nodes_touched.get(x as usize) == false {
                    nodes_touched.set(x as usize, true);
                    stack.push_back(x);
                }
            }
        }
        steps += 1;
    }
}
