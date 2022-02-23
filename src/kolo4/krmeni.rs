#![allow(non_snake_case)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static mut FILE: Option<BufReader<File>> = None;

fn stdin_line() -> String {
    let mut string = String::new();
    unsafe {
        match &mut FILE {
            Some(file) => file.read_line(&mut string).unwrap(),
            None => std::io::stdin().read_line(&mut string).unwrap(),
        }
    };

    string
}

fn main() {
    unsafe {
        FILE = Some(BufReader::new(
            File::open("./src/kolo4/krmeni.txt").unwrap(),
        ));
    }

    let line = stdin_line();
    let mut split = line.split_whitespace();

    let N = split.next().unwrap().parse::<usize>().unwrap();
    let Q = split.next().unwrap().parse::<usize>().unwrap();

    let mut pairs = vec![(0, 0); N - 1];
    let mut buf = vec![0; (N - 1) * 2];
    let mut nodes = vec![(0, 0); N];

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
        nodes[i] = (offset, 0);
        offset += neighbor_count as u32;
    }

    for &(u, v) in &pairs[0..(N - 1)] {
        // (offset, current neighbor count)
        let a = &mut nodes[u as usize];
        buf[(a.0 + a.1) as usize] = v;
        a.1 += 1;

        let b = &mut nodes[v as usize];
        buf[(b.0 + b.1) as usize] = u;
        b.1 += 1;
    }

    let mut first_occurence = vec![0; N];
    let mut node_depth = vec![0; N];
    let mut euler_index = 0;
    // (node, current_depth)
    let mut euler = vec![0; N * 2]; // is N-1 ok?
    let mut euler_depth = vec![0; N * 2];

    // (node index, child offset)
    let mut stack = Vec::new();
    let mut skip = false;
    let mut depth = 0u32;
    stack.push((0, 0u32.wrapping_sub(1)));
    while !stack.is_empty() {
        let (node_i, ch_offset) = stack.last_mut().unwrap();
        let (children_start, children_count) = nodes[*node_i as usize];

        // this is a trick to get the first index to be zero without needing special flags or using signed integers
        // if this integer overflows I would like to know how much memory the computer has
        *ch_offset = ch_offset.wrapping_add(1);

        if !skip {
            euler[euler_index as usize] = *node_i;
            euler_depth[euler_index as usize] = depth;
            euler_index += 1;
        }
        skip = false;
        
        if *ch_offset == children_count {
            stack.pop().unwrap();
            depth = depth.wrapping_sub(1);
        } else {
            // copy the value so that the reference can be dropped early by non lexical lifetimes
            let ch_offset = *ch_offset;

            let children =
                &buf[(children_start as usize)..((children_start + children_count) as usize)];
            let next_child = children[ch_offset as usize];

            // since the edges we created earlier are undirected, one of the child's edges will point back to the parent
            // this edge is skipped but a flag is needed because otherwise we would print the current node twice
            let parent = stack.len().checked_sub(2).map(|i| stack[i].0);
            if Some(next_child) == parent {
                skip = true;
                continue;
            }

            depth += 1;
            // children only ever get pushed once
            first_occurence[next_child as usize] = euler_index;
            node_depth[next_child as usize] = depth;
            stack.push((next_child, 0u32.wrapping_sub(1)));
        }
    }

    let mut segtree = vec![0; euler.len() * 4];
    build(1, 0, euler.len() as u32 - 1, &mut segtree, &euler, &mut node_depth);

    let mut sources = Vec::new();
    for _ in 0..Q {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let K = split.next().unwrap().parse::<u32>().unwrap();

        sources.resize(K as usize, 0);
        
        let mut sum_dist = 0;
        for j in 0..K {
            let index = split.next().unwrap().parse::<u32>().unwrap() - 1;
            sources[j as usize] = index;

            let dist = node_depth[index as usize];
            sum_dist += dist;
        }

        let result = sources.iter().cloned().reduce(|acc, item| {
            lca(acc, item, &segtree, &euler, &node_depth, &first_occurence).unwrap()
        }).unwrap();

        println!("{} / {}",result + 1, K*node_depth[result as usize]);
        // println!("{}",K*node_depth[result as usize])
    }
    
    // println!("{}", lca(1, 2, &segtree, &euler, &node_depth, &first_occurence).unwrap());
    // println!("{}", lca(3, 4, &segtree, &euler, &node_depth, &first_occurence).unwrap());
    // println!("{}", lca(3, 3, &segtree, &euler, &node_depth, &first_occurence).unwrap());
    // println!("{}", lca(1, 2, &segtree, &euler, &node_depth, &first_occurence).unwrap());
    // println!("{}", lca(4, 2, &segtree, &euler, &node_depth, &first_occurence).unwrap());
    


    // println!(
    //     "{:?}\n{:?}\n{:?}",
    //     first_occurence,
    //     euler.iter().map(|a| a.0).collect::<Vec<_>>(),
    //     euler.iter().map(|a| a.1).collect::<Vec<_>>()
    // );
}

// "adapted" from https://cp-algorithms.com/graph/lca.html
// the understanding is scarce and the Ctrl-C/V is abundant
fn build(node: u32, b: u32, e: u32, segtree: &mut [u32], euler: &[u32], depth: &[u32]) {
    if b == e {
        segtree[node as usize] = euler[b as usize];
    } else {
        let mid = (b + e) / 2;
        build(node << 1, b, mid, segtree, euler, depth);
        build(node << 1 | 1, mid + 1, e, segtree, euler, depth);
        let l = segtree[(node as usize) << 1];
        let r = segtree[(node as usize) << 1 | 1];
        segtree[node as usize] = if depth[l as usize] < depth[r as usize] {
            l
        } else {
            r
        };
    }
}

fn lca(u: u32, v: u32, segtree: &[u32], euler: &[u32], depth: &[u32], first: &[u32]) -> Option<u32> {
    fn query(
        node: u32,
        b: u32,
        e: u32,
        L: u32,
        R: u32,
        segtree: &[u32],
        euler: &[u32],
        depth: &[u32]
    ) -> Option<u32> {
        if b > R || e < L {
            return None;
        }
        if b >= L && e <= R {
            return Some(segtree[node as usize]);
        }
        let mid = (b + e) >> 1;

        let left = query(node << 1, b, mid, L, R, segtree, euler, depth);
        let right = query(node << 1 | 1, mid + 1, e, L, R, segtree, euler, depth);
        if left.is_none() {
            return right;
        };
        if right.is_none() {
            return left;
        };
        return if depth[left.unwrap() as usize] < depth[right.unwrap() as usize] {
            left
        } else {
            right
        };
    }

    let mut left = first[u as usize];
    let mut right = first[v as usize];
    if left > right {
        std::mem::swap(&mut left, &mut right);
    }
    return query(1, 0, euler.len() as u32 - 1, left, right, segtree, euler, depth);
}