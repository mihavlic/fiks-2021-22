use std::time::*;

use oorandom::Rand32;
use signal::Signal;

fn main() {
    let max = 3;
    let count = 16;

    let threadcount = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();

    let seed = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut rng = oorandom::Rand64::new(seed);

    let mut threads = Vec::new();

    for i in 0..threadcount {
        let thread_rng = Rand32::new(rng.rand_u64());
        let handle = std::thread::spawn(move || verify(thread_rng, max, count, i));
        threads.push(handle);
    }

    let trap = signal::trap::Trap::trap(&[Signal::SIGINT]);
    trap.wait(
        Instant::now()
            .checked_add(Duration::from_secs(100000))
            .unwrap(),
    );
}

fn verify(mut rng: Rand32, max: u32, count: usize, id: usize) {
    let mut elements = vec![0; count];

    let greedy_max_arr = vec![max; count];
    let mut random_max_arr = vec![0; count];

    for outer in 0..2048 {
        let mut sum = 0;

        for i in 0..count {
            let rand = rng.rand_range(0..(max + 3));

            elements[i] = rand;
            sum += rand;
        }

        let max_groups = (round_up(sum, max) / max).min(count as u32) as usize;

        let greedy_count = algo(&elements, &greedy_max_arr[..max_groups]);

        random_max_arr.fill(0);

        if id == 0 && outer == 0 {
            println!("Started bruteforce");
        }

        let start = Instant::now();
        for i in 0..((max as usize).pow(max_groups as u32)) {
            for group in 0..max_groups {
                let count = &mut random_max_arr[group];
                if *count == max {
                    *count = 0;
                } else {
                    *count += 1;
                    break;
                }
            }

            let rand_count = algo(&elements, &random_max_arr[..max_groups]);

            if rand_count < greedy_count {
                eprintln!(
                    "id: {}, outer: {}, i: {}, state: {:?}",
                    id,
                    outer,
                    i,
                    rng.state()
                );
                eprintln!("{:?}", elements);
                eprintln!("{:?}", rand_count);
                eprintln!();
            }
        }
        if id == 0 {
            println!("{}: {} ms", outer, start.elapsed().as_secs_f32() * 1000.0);
        }
    }
}

fn algo(elements: &[u32], groups_max: &[u32]) -> u32 {
    let mut diff_acc = 0;
    let mut groups: u32 = 0;
    for element in elements {
        let max = match groups_max.get(groups as usize) {
            None => return groups,
            Some(num) => *num,
        };

        if diff_acc + element > max {
            diff_acc = 0;
            groups += 1;
        }
        diff_acc += element;
    }
    groups
}

fn round_up(num: u32, multiple: u32) -> u32 {
    ((num + multiple - 1) / multiple) * multiple
}
