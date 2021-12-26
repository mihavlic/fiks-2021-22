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
// a situation can occur where one edge is completely out of bounds and the equations break down
// here the iterative solution yields 26 while the analytical only 24
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
//   ___O___O_|_O___O___O___O___O___
//        O _ O _ O _ O _ O _ O
//       /  O | O _ O _ O _ O
//       | /  O _ O _ O _ O
//       |/   | O _ O _ O
//  these two |   O _ O
//            |     O
// this happens when [x, y] are both closer to the walls than [t-2, t-2] (dunno why, trial and error)
// the area of the created triangle is
//  if (x - t + 2) < 0 && (y - t + 2) < 0 {
//      s -= min(x - t + 2, y - t + 2)^2;
//  }
// origin +- t/2 > 0 && < {w, h}

fn main() {

    let w = 10;
    let h = 10;

    let x = 7;
    let y = 8;

    let t = 18;

    println!("{}", iterative(w, h, x, y, t));
    println!("{}", analytical(w, h, x, y, t));
}

fn print(field: &[bool], w: i32, h: i32) {
    for y in (0..h).rev() {
        for x in 0..w {
            let ch = match field[(x+y*w) as usize] {
                true => 'X',
                false => '_',
            };

            print!("{} ", ch);
        }
        println!();
    }
}

fn iterative(w: i32, h: i32, x: i32, y: i32, t: i32) -> i32 {
    let mut a = vec![false; w as usize * h as usize];
    a[(x+y*w) as usize] = true;
    let mut b = vec![false; w as usize * h as usize];

    print(&a, w, h);
    println!();

    let get = |field: &mut [bool], x: i32, y: i32| {
        if (x != -1 && x != w) && (y != -1 && y != h) {
            return field[(x+y*w) as usize];
        } else {
            return false;
        }
    };

    for _ in 0..t {
        for y in 0..h {
            for x in 0..w {
                let this = &mut b[(x+y*w) as usize];

                let neighbors = 
                get(&mut a, x, y - 1) ||  
                get(&mut a, x - 1, y) ||  
                get(&mut a, x + 1, y) ||  
                get(&mut a, x, y + 1);  

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
        count += cell as i32;
    }

    count
}

fn analytical(w: i32, h: i32, x: i32, y: i32, t: i32) -> i32 {

    let s = (t+1)*(t+1);

    // let wx1 = dbg!(x - t + 2);
    // let wx2 = dbg!(w - x - t + 4);
    // let wy1 = dbg!(y - t + 2);
    // let wy2 = dbg!(h - y - t + 4);

    
    // if wx1 < 0 && wy1 < 0 {
    //     let min = wx1.min(wy1);
    //     s += (min * dbg!(min)) / 4;
    // }
    
    // origin +- t/2 > 0 && < {w, h}
    
    //   ^    d
    //   |
    // a |             c
    //   |---------->
    //        b

    let a = - x + t/2;
    let b = - y + t/2;
    
    let c = x + t/2 - w;
    let d = y + t/2 - h;
    
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
    dbg!(&d);

    let mut i = 0;

    if a >= 0 && b >= 0 {
        println!("Heck1");
        let o = a.max(b)+1;
        i += o*o / 4;
    }

    if b >= 0 && c >= 0 {
        println!("Heck2");
        let o = b.max(c)+1;
        i += o*o / 4;
    }

    if c >= 0 && d >= 0 {
        println!("Heck3");
        let o = c.max(d)+1;
        i += o*o / 4;
    }

    if d >= 0 && a >= 0 {
        println!("Heck4");
        let o = d.max(a)+1;
        i += o*o / 4;
    }


    let x1 = {
        let l = (t-x).max(0);
        (l+1)*l / 2
    };
    
    let x2 = {
        let l = (-w+x+t+1).max(0);
        (l+1)*l / 2
    };

    let y1 = {
        let l = (t-y).max(0);
        (l+1)*l / 2
    };
    
    let y2 = {
        let l = (-h+y+t+1).max(0);
        (l+1)*l / 2
    };

    dbg!(s) - dbg!(x1) - dbg!(x2) - dbg!(y1) - dbg!(y2) + dbg!(i)
}