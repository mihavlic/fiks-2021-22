#![allow(non_snake_case)]

use jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use std::{ops::DerefMut, alloc::Layout, collections::BinaryHeap};

type Point = (i32, i32);
// this is technically in the bottom left corner, however the points are guaranteed to be outside of the walls
const EMPTY: Point = (0, 0);
const LEAF_CAPACITY: usize = 16;
const NODE_MERGE_THRESHOLD: usize = 11;

#[derive(Debug)]
enum QuadNode {
    Nodes(Box<[QuadNode; 4]>),
    Leaf(Box<[Point; LEAF_CAPACITY]>),
}

struct QuadTree {
    root: QuadNode,
    half_extent: i32,
}

impl QuadTree {
    fn new(depth: u32) -> Self {
        Self {
            root: QuadNode::Leaf(Box::new([EMPTY; LEAF_CAPACITY])),
            half_extent: 2i32.pow(depth - 1)
        }
    }
    fn find_leaf(&mut self, p: Point) -> (*mut QuadNode, Option<*mut QuadNode>, Point, i32) {
        let mut parent: Option<*mut QuadNode> = None;
        let mut node: *mut QuadNode = &mut self.root;
        let mut center_x = 0;
        let mut center_y = 0;
        let mut half_extent = self.half_extent;
        
        loop {
            unsafe {
            match &mut *node {
                QuadNode::Nodes(nodes) => {
                    let index = Self::get_subtree_index(p, center_x, center_y);
                    Self::step_into_subtree(index, &mut half_extent, &mut center_x, &mut center_y);

                    parent = Some(node);
                    node = &mut nodes[index];
                }
                QuadNode::Leaf(_) => {
                    return (node, parent, (center_x, center_y), half_extent);
                }
            }}
        }
    }
    fn get_bucket_len(bucket: &Box<[Point; LEAF_CAPACITY]>) -> usize {
        bucket.iter().position(|p| *p == EMPTY).unwrap_or(LEAF_CAPACITY)
    }
    fn get_subtree_index(p: Point, center_x: i32, center_y: i32) -> usize {
        let x_half = p.0 <= center_x;
        let y_half = p.1 <= center_y;
        // 2 | 3
        // -----
        // 0 | 1
        ((y_half as usize) << 1) | (x_half as usize)
    }
    fn step_into_subtree(subtree_index: usize, half_extent: &mut i32, center_x: &mut i32, center_y: &mut i32) {
        let x_half = (subtree_index & 1) == 1;
        let y_half = (subtree_index & 2) == 2;

        *half_extent = *half_extent / 2;
                    
        // this really sucks but should be easy for the compiler to make it a lot better
        if x_half {
            *center_x -= *half_extent;
        } else {
            *center_x += *half_extent;
        }
        
        if y_half {
            *center_y -= *half_extent;
        } else {
            *center_y += *half_extent;
        }
    }
    fn insert(&mut self, p: Point) {
        debug_assert!(p != EMPTY);

        let (leaf, _, (mut center_x, mut center_y), mut half_extent) = self.find_leaf(p);
        
        // this is safe because we don't use the other pointer
        let mut leaf = unsafe {
            &mut *leaf
        };
        let bucket =
            match leaf {
            QuadNode::Nodes(_) => unreachable!(),
            QuadNode::Leaf(bucket) => bucket,
        };
        let len = Self::get_bucket_len(&bucket);
        
        if len == LEAF_CAPACITY {
            loop {
                //  gotta reuse the last bucket, just make sure to ptr::write later
                let owned_bucket_leaf = unsafe {
                    std::ptr::read(leaf)
                };

                let new_buckets = Box::new([
                    owned_bucket_leaf,
                    QuadNode::Leaf(Box::new([EMPTY; LEAF_CAPACITY])),
                    QuadNode::Leaf(Box::new([EMPTY; LEAF_CAPACITY])),
                    QuadNode::Leaf(Box::new([EMPTY; LEAF_CAPACITY])),
                ]);
    
                unsafe {
                    // must use ptr::write because we are reusing the old bucket so we don't want the destructor to run
                    std::ptr::write(leaf, QuadNode::Nodes(new_buckets));
                }
    
                let bucket_refs = {
                    // oh my god this is horrible
                    match leaf {
                        QuadNode::Nodes(nodes) => match nodes.deref_mut() {
                            [QuadNode::Leaf(b1), QuadNode::Leaf(b2), QuadNode::Leaf(b3), QuadNode::Leaf(b4)] => [b1, b2, b3, b4],
                            _ => unreachable!(),
                        },
                        QuadNode::Leaf(_) => unreachable!(),
                    }
                };
                let mut bucket_sizes = [0; 4];
    
                for p_i in 0..len {
                    // we have just reused the old bucket into one of the new ones, the points there are still there in the 0th bucket
                    // there is no danger of overwriting the old point as in the worst case we loop and subdivide the tree again
                    let p = bucket_refs[0][p_i];

                    let index = Self::get_subtree_index(p, center_x, center_y);

                    bucket_refs[index][bucket_sizes[index]] = p;
                    bucket_sizes[index] += 1;
                } 

                // insert the point that we wanted to insert in the first place
                let index = Self::get_subtree_index(p, center_x, center_y);
                
                // oh no, we never actually split the points into subtrees, instead, they are all still in one bucket
                // let's just split the tree one more time, just like in the find_leaf function 
                if bucket_sizes[index] == LEAF_CAPACITY {
                    Self::step_into_subtree(index, &mut half_extent, &mut center_x, &mut center_y);

                    leaf = match leaf {
                        QuadNode::Nodes(nodes) => &mut nodes[index],
                        _ => unreachable!()
                    };

                    continue;
                } else {
                    bucket_refs[index][bucket_sizes[index]] = p;
                    break;
                }
            
            }
        } else {
            bucket[len] = p;
        }
    }
    fn remove(&mut self, p: Point) {
        debug_assert!(p != EMPTY);
        
        let (leaf, parent, _, _) = self.find_leaf(p);
        let leaf = unsafe {
            &mut *leaf
        };
        let bucket = match leaf {
            QuadNode::Nodes(_) => unreachable!(),
            QuadNode::Leaf(bucket) => bucket,
        };
        
        let mut i = 0;
        // index of point to be removed
        let found = loop {
            if i == LEAF_CAPACITY {
                panic!("Attempting to remove point that doesn't exist");
            }
            if bucket[i] == p {
                break i;
            }
            i += 1;
        };
        let len = loop {
            if i == LEAF_CAPACITY {
                break LEAF_CAPACITY;
            }
            if bucket[i] == EMPTY {
                break i;
            }
            i += 1;
        };

        // swap remove the point, this is consistent even when len = 1
        bucket[found] = bucket[len - 1];
        bucket[len - 1] = EMPTY;

        if let Some(parent) = parent {
            // this is fine because miri said so
            let parent = unsafe {
                &mut *parent
            };
            match parent {
                QuadNode::Nodes(nodes) => {
                    match nodes.deref_mut() {
                        [QuadNode::Leaf(b1), QuadNode::Leaf(b2), QuadNode::Leaf(b3), QuadNode::Leaf(b4)] => {
                            let sizes = [&b1, &b2, &b3, &b4].map(|b| Self::get_bucket_len(b));
                            let total_points = sizes.iter().sum::<usize>();

                            if total_points <= NODE_MERGE_THRESHOLD {
                                let mut owned_buckets = unsafe {[
                                    std::ptr::read(b1),
                                    std::ptr::read(b2),
                                    std::ptr::read(b3),
                                    std::ptr::read(b4),
                                ]};
                                
                                // drop the Box holding the 4 nodes without dropping the contained boxes
                                // code taken from https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw
                                unsafe {
                                    let owned_nodes = std::ptr::read(nodes);
                                    let p = Box::into_raw(owned_nodes);
                                    std::alloc::dealloc(p as *mut u8, Layout::new::<[QuadNode; 4]>());
                                }

                                // use one of the buckets to compact the points into, drop the rest
                                let mut i = 0;
                                for bucket_i in 0..4 {
                                    for j in 0..sizes[bucket_i] {
                                        owned_buckets[0][i] = owned_buckets[bucket_i][j];
                                        i += 1;
                                    }
                                }

                                unsafe {
                                    // drop the other three boxes
                                    let [b1, ..] = owned_buckets;
                                    std::ptr::write(parent, QuadNode::Leaf(b1));
                                }
                            }
                        },
                        // not all of the 
                        _ => (),
                    }
                },
                QuadNode::Leaf(_) => unreachable!(),
            }
        }
    }
    fn distance(p: Point, c: Point, corner_door_distance: i32) -> i32 {
        (((p.0 - c.0) as f64).powf(2.0) + ((p.1 - c.1) as f64).powf(2.0)).sqrt().round() as i32 + corner_door_distance
    }
    // check a 'house' against all corners of the zoo combined with the path to the door, return the minimum distance
    // since the tourists always take the optimal path
    fn point_distance(p: Point, corners: &[Point; 4], corner_to_door: &[i32; 4]) -> i32 {
        //     |        |               
        //   --3--------2--             
        //     |        |              
        //   --0--------1--             
        //     |        |     

        let mut mask = [false; 4];
        if p.0 < 0 {
            mask[0] = true;
            mask[3] = true;
        } else {
            mask[1] = true;
            mask[2] = true;
        }
        if p.1 < 0 {
            mask[0] = true;
            mask[1] = true;
        } else {
            mask[2] = true;
            mask[3] = true;
        }
        
        let mut min = i32::MAX;
        for i in 0..4 {
            if mask[i] == true {
                let dist = Self::distance(p, corners[i], corner_to_door[i]);
                min = min.min(dist);
            }
        }

        min
    }
    // get the bounds of the quadtree region, essentially min/max of Self::distance() of the region corners
    fn quadtree_quadrant_distance_bounds(c: Point, h_e: i32, corners: &[Point; 4], corner_to_door: &[i32; 4]) -> (i32, i32) {
        let points = [
            (c.0 - h_e, c.1 - h_e),
            (c.0 + h_e, c.1 - h_e),
            (c.0 + h_e, c.1 + h_e),
            (c.0 - h_e, c.1 + h_e),
        ];

        let mut min = i32::MAX;
        let mut max = 0;
        for i in 0..4 {
            for j in 0..4 {
                let dist = Self::distance(points[i], corners[j], corner_to_door[j]);
                min = min.min(dist);
                max = max.max(dist);
            }  
        }
        (min, max)
    }
    fn query_farthest<'a>(&'a self, d: Point, N: i32, heap: &mut BinaryHeap<Comparison<'_>>) -> i32 {
        assert!(N > 0);
        debug_assert!((d.0 == 0 || d.0 == N) || (d.1 == 0 || d.1 == N));

        let corners = [
            (0, 0),
            (N, 0),
            (N, N),
            (0, N),
        ];
        // manhattan distance of corner to door
        let corner_to_door = corners.map(|c| (c.0 - d.0).abs() + (c.1 - d.1).abs());

        // rust is complaining here, because we're pushing objects that hold a reference to self
        // so it thinks that the references outlive this function in the heap, even though we clear it at the end
        // so we just tell rustc that suddenly this heap is ok to use for the references even though the one that was
        // given to us as a parameter is not
        let heap = unsafe {
            std::mem::transmute::<_, &mut BinaryHeap<Comparison<'a>>>(heap)
        };
        
        let center = (0, 0);
        let bounds = Self::quadtree_quadrant_distance_bounds(center, self.half_extent, &corners, &corner_to_door);
        // reverse the bounds to (max, min) to first get the farthest quadrants
        heap.push(Comparison((bounds.1, bounds.0), (center, self.half_extent), &self.root));
        
        let mut max_distance = 0;
        loop {
            let Comparison((max, _), (c, h_e), node) = match heap.pop() {
                Some(comp) => comp,
                // there are no more nodes to search
                None => break,
            };
            
            // we have exhausted quadrants that could provide a more distant point
            if max_distance > max {
                break;
            }

            match node {
                QuadNode::Nodes(nodes) => {
                    for (i, node) in nodes.iter().enumerate() {
                        let mut c = c;
                        let mut h_e = h_e;
                        
                        Self::step_into_subtree(i, &mut h_e, &mut c.0, &mut c.1);
                        let bounds = Self::quadtree_quadrant_distance_bounds(c, h_e, &corners, &corner_to_door);

                        heap.push(Comparison((bounds.1, bounds.0), (c, h_e), &node));
                    }
                },
                QuadNode::Leaf(bucket) => {
                    let len = Self::get_bucket_len(bucket);
                    for i in 0..len {
                        let p = bucket[i];
                        let dist = Self::point_distance(p, &corners, &corner_to_door);
                        max_distance = max_distance.max(dist);
                    }
                },
            }
        }
        
        heap.clear();
        max_distance
    }
}

struct Comparison<'a>((i32, i32), (Point, i32), &'a QuadNode);

impl PartialOrd for Comparison<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Comparison<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<'a> PartialEq for Comparison<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Comparison<'_> {}

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    string
}

fn main() {
    let mut tree = QuadTree::new(28);
    
    let line = stdin_line();
    let mut split = line.split_whitespace();

    let N = split.next().unwrap().parse::<i32>().unwrap();
    let Q = split.next().unwrap().parse::<usize>().unwrap();

    let mut heap = BinaryHeap::new();
    println!();
    for o in 0..Q {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let command = split.next().unwrap();
        let x = split.next().unwrap().parse::<i32>().unwrap();
        let y = split.next().unwrap().parse::<i32>().unwrap();

        match command {
            "+" => tree.insert((x, y)),
            "-" => tree.remove((x, y)),
            "?" => {
                let dist = tree.query_farthest((x, y), N, &mut heap);
                print!("\r{} {}", o, dist);
            }
            _ => unreachable!()
        }
    }
    println!()
}
