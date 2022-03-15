#![allow(unused)]

use core::panic;
use std::io::Read;

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

struct DnaBases {
    data: Vec<u8>,
    bases_count: usize,
}

const A: u8 = 0b00;
const C: u8 = 0b01;
const G: u8 = 0b10;
const T: u8 = 0b11;

impl DnaBases {
    fn from_bytes(bytes: &[u8], count: usize) -> Self {
        Self {
            data: bytes.to_vec(),
            bases_count: count,
        }
    }
    fn from_str(string: &str) -> Self {
        let mut data = Vec::new();
        data.resize(string.len() / 4 + 1, 0);

        for (i, ch) in string.bytes().enumerate() {
            let value = match ch as char {
                'A' => A,
                'C' => C,
                'G' => G,
                'T' => T,
                _ => unreachable!(),
            };

            let shifted = value << 6 - 2 * (i % 4);

            data[i / 4] |= shifted;
        }

        Self {
            data,
            bases_count: string.len(),
        }
    }
    fn get(&self, index: usize) -> u8 {
        let byte = self.data[index / 4];

        let value = byte >> 6 - 2 * (index % 4);

        value & 0b00000011
    }
    fn len(&self) -> usize {
        self.bases_count
    }
}

// len_a > len_b
fn edit_distance<'a>(
    mut a: &'a DnaBases,
    mut b: &'a DnaBases,
    max_distance: u8,
    matrix: &mut Vec<u8>,
) -> u8 {
    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }

    let len_a = a.len();
    let len_b = b.len();

    let len_diff = len_a.abs_diff(len_b);
    if len_diff > max_distance as usize {
        return len_diff as u8;
    }

    // handle special case of 0 length
    if len_a == 0 {
        return len_b as u8;
    } else if len_b == 0 {
        return len_a as u8;
    }

    // let len_b = b.len().min(a.len() - 1 + max_distance as usize) + 1;
    let len_b = len_b + 1;

    matrix.resize(len_b, 0);

    // initialize string b
    for i in 0..len_b {
        matrix[i] = i as u8;
    }

    let mut pre;
    let mut tmp;

    // calculate edit distance
    for i in 0..a.len() {
        let ca = a.get(i);
        // get first column for this row
        pre = matrix[0];
        matrix[0] = (i + 1) as u8;

        // https://stackoverflow.com/questions/48901351/levenstein-distance-limit
        // for j in i.saturating_sub(max_distance as usize + 1)..b.len().min(i + max_distance as usize) {
        for j in 0..b.len() {
            let cb = b.get(j);

            tmp = matrix[j + 1];
            matrix[j + 1] = std::cmp::min(
                // deletion
                tmp + 1,
                std::cmp::min(
                    // insertion
                    matrix[j] + 1,
                    // match or substitution
                    pre + if ca == cb { 0 } else { 1 },
                ),
            );
            pre = tmp;
        }
    }

    matrix[len_b - 1]
}

fn main() {
    let t = stdin_line().trim().parse::<usize>().unwrap();

    let mut scratch = Vec::<u8>::new();
    let mut adjacency_matrix = Vec::<u8>::new();

    let stdin_ = std::io::stdin();
    let mut stdin = stdin_.lock().bytes().peekable();

    let mut buf = Vec::new();

    for _ in 0..t {
        buf.clear();

        const LF: u8 = '\n' as u8;
        const CR: u8 = '\r' as u8;

        loop {
            match stdin.next() {
                Some(Ok(LF | CR)) => break,
                Some(Ok(ch)) => buf.push(ch),
                _ => break,
            };
        }

        loop {
            match stdin.peek() {
                Some(Ok(LF | CR)) => stdin.next(),
                _ => break,
            };
        }

        let mut split = std::str::from_utf8(&buf).unwrap().split_whitespace();

        // dna sample count
        let n = split.next().unwrap().parse::<usize>().unwrap();
        // max distance
        let k = split.next().unwrap().parse::<usize>().unwrap();

        let mut samples = Vec::with_capacity(n);
        for _i in 0..n {
            buf.clear();

            loop {
                let next = stdin.next().unwrap().unwrap();
                next as char;

                if next == ' ' as u8 {
                    break;
                }

                buf.push(next);
            }

            let count = std::str::from_utf8(&buf).unwrap().parse::<usize>().unwrap();
            // 4 dna bases per byte
            let size = div_up(count, 4);

            buf.clear();
            buf.extend((&mut stdin).take(size).map(|r| r.unwrap()));
            samples.push(DnaBases::from_bytes(&buf, count));

            loop {
                match stdin.peek() {
                    Some(Ok(LF | CR)) => stdin.next(),
                    _ => break,
                };
            }
        }

        // for (a, sample) in samples.iter().enumerate() {
        //     for s in 0..sample.len() {
        //         let chars = ['A', 'C', 'G' ,'T'];

        //         let val = sample.get(s);

        //         print!("{}", chars[val as usize]);
        //     }
        //     println!();
        // }

        //     0 1 2 3 4
        //
        // 0   0 2 2 4 3
        // 1   2 0 2 2 4
        // 2   2 2 0 2 4
        // 3   4 2 2 0 5
        // 4   3 4 4 5 0
        let w = n;
        let h = n;

        adjacency_matrix.resize(w * h, 0);
        adjacency_matrix.fill(k as u8 + 1);

        for (y, sample_1) in samples.iter().enumerate() {
            for (x, sample_2) in samples[..y].iter().enumerate() {
                if y == x {
                    continue;
                }

                let distance = edit_distance(sample_1, sample_2, k as u8, &mut scratch);

                adjacency_matrix[x + y * w] = distance;
                adjacency_matrix[y + x * w] = distance;
            }
        }

        // println!("\n{}, {}", n, k);
        // print!("   ");
        // for x in 0..w {
        //     print!("{:2} ", x);
        // }
        // println!();
        // for y in 0..h {
        //     for x in 0..w {

        //         if x == 0 {
        //             print!("{:2} ", y);
        //         }

        //         // if x == y  { continue; }

        //         let a = adjacency_matrix[x + y * w];

        //         print!("{:2} ", a);
        //     }
        //     println!();
        // }
        // println!();

        for cell in &mut adjacency_matrix {
            if *cell <= k as u8 {
                *cell = 1;
            } else {
                *cell = 0;
            }
        }

        let mut triplets = Vec::new();

        for y in 0..h {
            for x1 in 0..w {
                if adjacency_matrix[x1 + y * w] == 0 {
                    continue;
                }
                if x1 == y {
                    continue;
                }

                for x2 in (x1 + 1)..w {
                    if x2 == y {
                        continue;
                    }

                    if adjacency_matrix[x2 + y * w] == 0 {
                        continue;
                    }

                    if adjacency_matrix[x1 + x2 * w] == 0 {
                        triplets.push((x1, y, x2));
                    }
                }
            }
        }

        for (i, triplet1) in triplets.iter().enumerate() {
            for (j, triplet2) in triplets.iter().enumerate() {
                if *triplet1 == *triplet2 && i != j {
                    println!("AAAAAAaaaaaaa");
                    panic!();
                }
            }
        }

        println!("{}", triplets.len());
        for (x1, y, x2) in triplets {
            println!("{} {} {}", x1, y, x2);
        }
    }

    // for triplets in &total_triplets {
    //     print!("{} ", triplets.len());
    // }
    // println!();

    // for triplets in &total_triplets {
    //     for (x1, y, x2) in triplets {
    //         println!("{} {} {}", x1, y, x2);
    //     }
    // }
}

pub fn div_up(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}
