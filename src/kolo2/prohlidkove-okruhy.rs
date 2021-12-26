// Poznámky k odevzdání:
//  toto je napsáno v Rustu, koncovka je .txt protože odevzdávací systém nepřijímá .rs soubory
//  pro kompilaci programu použijte 'rustc ./prohlidkove-okruhy_rust.txt'
//  užitečný one-liner pro běžení se vstupními daty je 'rustc ./prohlidkove-okruhy_rust.txt -o sponzori && ./sponzori < $VSTUPNI_DATA'

use std::cell::Cell;

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[derive(Clone)]
struct Crossroad {
    outgoing_edges: Vec<usize>,
    touched: Cell<bool>,
}

impl Default for Crossroad {
    fn default() -> Self {
        Self {
            outgoing_edges: Vec::new(),
            touched: Cell::new(false),
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    ///// INPUT PARSING /////

    // line parsing boilerplate
    let line = stdin_line();
    let mut split = line.split_whitespace();

    // crossroad count, edge count, query count
    let M = split.next().unwrap().parse::<usize>().unwrap();
    let N = split.next().unwrap().parse::<usize>().unwrap();
    let O = split.next().unwrap().parse::<usize>().unwrap();

    let mut crossroads = vec![Crossroad::default(); M];

    for _ in 0..N {
        // line parsing boilerplate
        let line = stdin_line();
        let mut split = line.split_whitespace();

        // directed P -> Q, crossorads indexed from 1
        let P = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let Q = split.next().unwrap().parse::<usize>().unwrap() - 1;

        crossroads[P].outgoing_edges.push(Q);
    }

    // stack for the graph search is allocated here
    let mut stack = Vec::new();

    ///// QUERY PARSING AND SOLVING /////

    for i in 0..O {
        // line parsing boilerplate
        let line = stdin_line();
        let mut split = line.split_whitespace();

        // indexed from 1
        let A = split.next().unwrap().parse::<usize>().unwrap() - 1;
        let B = split.next().unwrap().parse::<usize>().unwrap() - 1;

        // if not on the first iteration reset the touched flag
        if i != 0 {
            for crossroad in &mut crossroads {
                crossroad.touched.set(false);
            }
        }

        // the stack is empty at first, only containing the root of the search
        stack.clear();
        stack.push(A);
        crossroads[A].touched.set(true);

        let reachable = loop {
            // we've exhausted all the reachable nodes, signal failure
            if stack.is_empty() {
                break false;
            }

            let current_i = stack.pop().unwrap();

            // the target node has been found
            if current_i == B {
                break true;
            }

            // iterate through all the nodes connected to the one we just popped and try to push them onto the stack
            //
            // interior mutability with Cell is needed here because the for-loop holds a reference to crossroads while we also want to mutate the Crossroad.touched flag
            // this could also be solved by iterating through integer ranges and indexing everything all the time
            for &connected_i in &crossroads[current_i].outgoing_edges {
                let connected = &crossroads[connected_i];

                // if the node hasn't yet been touched, add it and touch it
                // otherwise it is either already on the stack or has been popped and there is no use in pushing it again
                if connected.touched.get() == false {
                    stack.push(connected_i);
                    connected.touched.set(false);
                }
            }
        };

        match reachable {
            true => println!("Cesta existuje"),
            false => println!("Cesta neexistuje"),
        }
    }
}
