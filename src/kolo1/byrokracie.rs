use std::cell::Cell;

// fn stdin_to_numbers(nums: &mut [u32]) {
//     static mut STRING_BUF: String = String::new();
//     // první úloha a už tu používám unsafe :D
//     unsafe {
//         STRING_BUF.clear();
//         let _ = std::io::stdin().read_line(&mut STRING_BUF).unwrap();

//         for (i, num) in STRING_BUF.split_whitespace().enumerate() {
//             if i < nums.len() {
//                 nums[i] = num.parse::<u32>().unwrap();
//             } else {
//                 return;
//             }
//         }
//     };
// }

// když to nejde, přidej víc maker
macro_rules! read_numbers {
    ($($var:ident),+) => {
        let mut string = String::with_capacity(6);
        let _ = std::io::stdin().read_line(&mut string).unwrap();
        let mut nums = string.split_whitespace();
        $(
            let $var: usize = nums.next().unwrap().parse::<usize>().unwrap();
        )+
    }
}

#[allow(non_snake_case)]
fn main() {
    read_numbers!(T);

    'zadani: for _ in 0..T {
        read_numbers!(P, Z, n);

        #[derive(Clone)]
        struct Povoleni {
            zavislosti: Vec<usize>,
            active: Cell<bool>,
            touched: Cell<bool>,
        }

        let mut povoleni = vec![
            Povoleni {
                zavislosti: Vec::new(),
                active: Cell::new(false),
                touched: Cell::new(false),
            };
            P
        ];

        for _ in 0..Z {
            read_numbers!(a, b);
            povoleni[a].zavislosti.push(b);
        }

        let mut stack = Vec::new();

        povoleni[n].active.set(true);
        stack.push((n, 0));

        let mut a = Vec::new();

        while !stack.is_empty() {
            let (n, i) = stack.last().unwrap().clone();
            let up = &povoleni[n];

            if i < up.zavislosti.len() {
                let push = up.zavislosti[i];
                if povoleni[push].active.get() == true {
                    println!("ajajaj");
                    continue 'zadani;
                }
                stack.last_mut().unwrap().1 += 1;
                if povoleni[push].touched.get() == true {
                    continue;
                }

                povoleni[push].active.set(true);
                stack.push((push, 0));
            } else {
                povoleni[n].active.set(false);

                if povoleni[n].touched.get() == false {
                    a.push(n);
                    povoleni[n].touched.set(true);
                }

                stack.pop();
                continue;
            }
        }

        print!("pujde to");
        for i in a {
            print!(" {}", i)
        }
        println!();
    }
}
