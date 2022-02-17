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

    let mut adj_matrix = vec![false; N * N];

    for _ in 0..(N - 1) {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        // indexovani v kodu od 0 oproti od 1 v zadani
        let u = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let v = split.next().unwrap().parse::<usize>().unwrap() - 1;

        adj_matrix[u + v * N] = true;
        adj_matrix[v + u * N] = true;
    }

    // let N = 5;

    // let mut adj_matrix = [
    //     0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0,
    // ]
    // .iter()
    // .map(|i| *i > 0)
    // .collect::<Vec<_>>();

    let mut touched = vec![false; N];
    let mut stack = VecDeque::new();

    stack.push_back(0);
    touched[0] = true;

    let mut distance_from_czech = vec![0; N];
    map_bfs(&mut adj_matrix, &mut touched, &mut stack, |node, gen| {
        distance_from_czech[node] = gen
    });

    touched.fill(false);

    for _ in 0..Q {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let K = split.next().unwrap().parse::<usize>().unwrap();

        let mut zeme = vec![0; K];
        for i in 0..K {
            // indexovani v kodu od 0 oproti od 1 v zadani
            let zeme_index = split.next().unwrap().parse::<usize>().unwrap() - 1;
            zeme[i] = zeme_index;
        }

        solve(&graph, &zeme, N);
    }
}

fn solve(graph: &Vec<bool>, touched: &mut Vec<bool>, zeme: &Vec<usize>, N: usize) {
    assert!(touched.len() == N);
}

//                    (node_index, generation)
fn map_bfs<F: FnMut(usize, usize)>(
    adj_matrix: &Vec<bool>,
    mut nodes_touched: &mut Vec<bool>,
    mut stack: &mut VecDeque<usize>,
    mut fun: F,
) {
    let N = nodes_touched.len();
    assert!(N * N == adj_matrix.len());

    let mut generation = 0;
    while !stack.is_empty() {
        let last = stack.len();

        for _ in 0..last {
            let pop = stack.pop_front().unwrap();
            fun(pop, generation);

            for x in 0..N {
                let connected = adj_matrix[pop * N + x];
                if nodes_touched[x] == false && connected == true {
                    nodes_touched[x] = true;
                    stack.push_back(x);
                }
            }
        }
        generation += 1;
    }
}
