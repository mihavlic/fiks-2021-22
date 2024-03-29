fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    string
}

#[allow(non_snake_case)]
fn main() {

    let line = stdin_line();
    let mut split = line.split_whitespace();

    let N = split.next().unwrap().parse::<usize>().unwrap();
    let Q = split.next().unwrap().parse::<usize>().unwrap();

    let mut pairs = vec![(0, 0); N - 1];
    let mut child_buf = vec![0; (N - 1) * 2];
    // for each node, store offset into child_buf and the count of children/edges
    let mut nodes = vec![(0, 0); N];

    for i in 0..(N - 1) {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        // indexovani v kodu od 0 oproti od 1 v zadani
        let u = (split.next().unwrap().parse::<usize>().unwrap() - 1) as u32;
        let v = (split.next().unwrap().parse::<usize>().unwrap() - 1) as u32;

        pairs[i] = (u, v);
        // temporary use of buffer for child count of each node
        child_buf[u as usize] += 1;
        child_buf[v as usize] += 1;
    }

    // compute offset into child_buff from the temporary data stored there
    let mut offset = 0;
    for i in 0..N {
        let neighbor_count = child_buf[i];
        nodes[i] = (offset, 0);
        offset += neighbor_count as u32;
    }

    // overwrite the temporary data with actual children, use nodes[].1 as a cursor of where to write
    // at the end everything is fine
    for &(u, v) in &pairs[0..(N - 1)] {
        let a = &mut nodes[u as usize];
        child_buf[(a.0 + a.1) as usize] = v;
        a.1 += 1;

        let b = &mut nodes[v as usize];
        child_buf[(b.0 + b.1) as usize] = u;
        b.1 += 1;
    }

    drop(pairs);

    let mut order = 0;
    let mut node_interval = vec![(0, 0); N];
    let mut node_depth = vec![0; N];

    // (node index, child offset)
    let mut stack = Vec::new();
    let mut depth = 0u32;

    stack.push((0u32, 0u32.wrapping_sub(1)));
    while !stack.is_empty() {
        let (node_i, next_child) = stack.last_mut().unwrap();
        let (ch_offset, ch_count) = nodes[*node_i as usize];

        // this is a trick to get the first index to be zero without needing special flags or using signed integers
        // if this integer overflows I would like to know how much memory the computer has
        *next_child = next_child.wrapping_add(1);

        if *next_child == ch_count {
            node_interval[*node_i as usize].1 = order;
            stack.pop().unwrap();
            depth = depth.wrapping_sub(1);
        } else {
            // unpack the children/edges
            let children = &child_buf[(ch_offset as usize)..((ch_offset + ch_count) as usize)];
            let next_child = children[*next_child as usize];

            // since the edges we created earlier are undirected, one of the child's edges will point back to the parent
            // since we will never need them, we just swap the index with the last child and decrement the count, as such it can always be reconstructed
            {
                let (offset, count) = &mut nodes[next_child as usize];
                let children = &mut child_buf[(*offset as usize)..((*offset + *count) as usize)];
                *count -= 1;

                // find index of edge that points back to current node, move it to the end and shrink the edge count
                let index_of_parent = children.iter().position(|i| *i == *node_i).unwrap();

                let saved_last = *children.last().unwrap();
                let saved_parent = children[index_of_parent];

                *children.last_mut().unwrap() = saved_parent;
                children[index_of_parent] = saved_last;
            }

            depth += 1;
            order += 1;

            node_interval[next_child as usize].0 = order;
            node_depth[next_child as usize] = depth;
            stack.push((next_child, 0u32.wrapping_sub(1)));
        }
    }

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

        let depth = nearest_node(&sources, &nodes, &child_buf, &node_interval, &node_depth);
        println!("{}", sum_dist- depth)
    }
}

fn nearest_node(sources: &[u32], nodes: &[(u32, u32)], child_buf: &[u32], node_interval: &[(u32, u32)], node_depth: &[u32]) -> u32 {
    // algorithm can never terminate because it will always take the path with a single child, 
    // then runs out of nodes and tries to subtract with overflow, this case is always zero
    if sources.len() == 1 {
        return 0;
    }

    // go down into the tree and do some reasoning on each node
    // we can only move down into the tree and want to find a point closest to all the `sources`
    // as we move down we get closer to those sources that are within the interval
    // when we get to a node from whose children only ever less than a half of the sources can
    // be reached we find our best point because until then we had been moving closer to the majority
    
    // now it may seem that there is an edge case when two children split the sources precisely in half
    // however in that case any movement into one of the subtrees saves just as much distance as it generates 
    // from the other subtree, in that case we just stop where we are

    // about the distance calculation:
    // every time we take a node with n source children we are getting farther from `sources.len() - n` nodes
    // when we discard sources by going down a child we add the differences of their depths and that of the current node
    // when we finish we do the same as with orphaned nodes
    let equilibrium = sources.len() as u32 / 2; // for odd numbers we do want to round down
    let mut distance = 0;
    let mut cur_node = 0;
    let _min_node = loop {
        let children = get_node_children(cur_node, nodes, child_buf);
        
        let mut max_nodes_passed = 0u32;
        let mut next_node = None;
        
        let mut depth_of_all = 0;
        let mut depth_of_max = 0;
        for &child in children {
            let (start, end) = node_interval[child as usize];
            
            // the number of sources which are the children of this node
            let mut nodes_passed = 0;
            let mut depth = 0;
            for &source in sources {
                let (l, r) = node_interval[source as usize];
                // the source is a child
                if start <= l && r <= end {
                    nodes_passed += 1;
                    depth += node_depth[source as usize] - node_depth[cur_node as usize];
                }
            }

            depth_of_all += depth;

            if nodes_passed > max_nodes_passed {
                max_nodes_passed = nodes_passed;
                next_node = Some(child);
                depth_of_max = depth;
            }
        }
        
        // in case we have 3 sources and reach a node that splits them 1, 2
        // we will follow the node with 2 because it still saves distance,
        // eventually we reach a node that splits 1, 1 and there we stop
        if max_nodes_passed <= equilibrium {
            distance += depth_of_all;
            break cur_node;
        } else {
            // this calculates the distance of all the sources that were just left behind by going down the tree
            // of course this does't include the nodes that are still left "active"
            distance += depth_of_all - depth_of_max;

            // when moving down the child with `max_nodes_passed` sources
            // we get farther from all the othersources by 1
            // furthermore we just abandoned some other nodes and need to add their
            // depth difference
            distance += sources.len() as u32 - max_nodes_passed;
            cur_node = next_node.unwrap();
        }
        
    };

    distance
}

// currently unused Lowest Common Ancestor implementation
// it was used to try out the node_interval structure
#[allow(unused)]
fn lca(l: u32, r: u32, nodes: &[(u32, u32)], child_buf: &[u32], node_interval: &[(u32, u32)]) -> u32 {
    let mut cur_node = 0;
    loop {
        let children = get_node_children(cur_node, nodes, child_buf);
        for child in children {
            let (start, end) = node_interval[*child as usize];

            if start <= l && r <= end {
                cur_node = *child;
                continue;
            }
        }
        break;
    }
    cur_node
}

fn get_node_children<'a>(node: u32, nodes: &[(u32, u32)], child_buf: &'a [u32]) -> &'a [u32] {
    let (offset, count) = nodes[node as usize];
    &child_buf[(offset as usize)..((offset + count) as usize)]
}
