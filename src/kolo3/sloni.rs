#![allow(unused)]

// automata evolved from 10 10 4 4 4
// in this case where no walls are touched, the count is (t+1)^2 so 25
// _ _ _ _ _ _ _ _ _ _
// _ _ _ _ X _ _ _ _ _
// _ _ _ X _ X _ _ _ _
// _ _ X _ X _ X _ _ _
// _ X _ X _ X _ X _ _
// X _ X _ X _ X _ X _   9
// _ X _ X _ X _ X _ _ 4 8
// _ _ X _ X _ X _ _ _ 3 5
// _ _ _ X _ X _ _ _ _ 2 3
// _ _ _ _ X _ _ _ _ _ 1 2

// 10 10 4 2 4
// here the patter stopped at the wall,
// l=max(t-y, 0), the loss is (l+1)l / 2
// this also is true for the opposite wall such that l = max(h-t, 0) and the x axis
// _ _ _ _ _ _ _ _ _ _
// _ _ _ _ _ _ _ _ _ _
// _ _ _ _ _ _ _ _ _ _
// _ _ _ _ X _ _ _ _ _
// _ _ _ X _ X _ _ _ _
// _ _ X _ X _ X _ _ _
// _ X _ X _ X _ X _ _
// X _ X _ X _ X _ X _
// _ X _ X _ X _ X _ _
// _ _ X _ X _ X _ _ _

// 10 10 2 1 6
// a situation can occur where the diagonal edge of the pattern is beyond the allowed area
// here the correct iterative solution yields 26 while the analytical only 24
// this causes a part of it to be subtracted twice and has to be added back
//            | _ _ _ _ _ _ _ _ __
//            | _ _ _ _ _ _ _ _ _ _
//            | _ _ X _ _ _ _ _ _ _
//            | _ X _ X _ _ _ _ _ _
//            | X _ X _ X _ _ _ _ _
//            O _ X _ X _ X _ _ _ _
//          O | X _ X _ X _ X _ _ _
//        O _ O _ X _ X _ X _ X _ _
//      O _ O | X _ X _ X _ X _ X _
//    O _ O _ O _ X * X _ X _ X _ X
//      O _ O_|_X _ X _ X _ X _ X _
//    ____O___O___O___O___O___O____
//       /  O | O _ O _ O _ O
//       | /  O _ O _ O _ O
//       |/   | O _ O _ O
//  these two |   O _ O
//            |     O

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

fn main() {
    let n = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..n {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let w = split.next().unwrap().parse::<i64>().unwrap();
        let h = split.next().unwrap().parse::<i64>().unwrap();

        let x = split.next().unwrap().parse::<i64>().unwrap();
        let y = split.next().unwrap().parse::<i64>().unwrap();

        let t = split.next().unwrap().parse::<i64>().unwrap();

        let analytical = analytical(w, h, x, y, t);
        // let iterative = iterative(w, h, x, y, t);

        println!("{}", analytical /* , iterative */);
    }

    // let w = 10;
    // let h = 10;

    // let x = 3;
    // let y = 2;

    // let t = 5;

    // println!("{}", iterative(w, h, x, y, t));
    // println!("{}", analytical(w, h, x, y, t));
}

fn analytical(w: i64, h: i64, x: i64, y: i64, t: i64) -> i64 {
    // the size of the patter if there are no walls
    let mut s = (t + 1) * (t + 1);

    // subtract the parts of the pattern beyond the walls
    let mut oob = |l: i64| {
        let l = l.max(0);
        s -= (l + 1) * l / 2;
    };

    oob(t - x);
    oob(-w + x + t + 1);
    oob(t - y);
    oob(-h + y + t + 1);

    // fix when the oob parts overlap and are subtracted twice
    let mut fix = |m: i64, n: i64| {
        let val = (m + n - t).min(0);
        s += val * val / 4;
    };

    fix(x, y);
    fix(w - x - 1, y);
    fix(w - x - 1, h - y - 1);
    fix(x, h - y - 1);

    s
}

fn print(field: &[bool], w: i64, h: i64) {
    for y in (0..h).rev() {
        for x in 0..w {
            let ch = match field[(x + y * w) as usize] {
                true => 'X',
                false => '_',
            };

            print!("{} ", ch);
        }
        println!();
    }
}

fn iterative(w: i64, h: i64, x: i64, y: i64, t: i64) -> i64 {
    let mut a = vec![false; w as usize * h as usize];
    a[(x + y * w) as usize] = true;
    let mut b = vec![false; w as usize * h as usize];

    print(&a, w, h);
    println!();

    let get = |field: &mut [bool], x: i64, y: i64| {
        if (x != -1 && x != w) && (y != -1 && y != h) {
            return field[(x + y * w) as usize];
        } else {
            return false;
        }
    };

    for _ in 0..t {
        for y in 0..h {
            for x in 0..w {
                let this = &mut b[(x + y * w) as usize];

                let neighbors = get(&mut a, x, y - 1)
                    || get(&mut a, x - 1, y)
                    || get(&mut a, x + 1, y)
                    || get(&mut a, x, y + 1);

                *this = neighbors;

                // truth table for the automata
                // current neighbors -> next
                // 0 0 0
                // 0 1 1
                // 1 0 0
                // 1 1 1
            }
        }

        std::mem::swap(&mut a, &mut b);

        // print(&a, w, h);
        // println!();
    }

    print(&a, w, h);
    println!();

    let mut count = 0;
    for cell in a {
        count += cell as i64;
    }

    count
}
