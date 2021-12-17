use std::time::*;

fn main () {
    let seed =  SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut rng = oorandom::Rand32::new(seed);

    let max = 6;
    let count = 71;

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

        let max_groups = (round_up(sum, max) / max) as usize;

        let start = Instant::now();
        for i in 0..(max.pow(max_groups)) {
//             permute(&mut rng, max, &mut random_max_arr);



            let greedy_count = algo(&elements, &greedy_max_arr[..max_groups]);
            let rand_count = algo(&elements, &random_max_arr[..max_groups]);

            if rand_count < greedy_count {
                println!("outer: {}, i: {}, state: {:?}", outer, i, rng.state());
                println!("{:?}", elements);
                println!("{:?}", rand_count);
                println!();
            }
        }
        println!("{} Elapsed: {} ms", outer, start.elapsed().as_secs_f32() * 1000.0);
    }
}

fn algo(elements: &[u32], groups_max: &[u32]) -> u32 {
    let mut diff_acc = 0;
    let mut groups: u32 = 0;
    for element in elements {
        let max = match groups_max.get(groups as usize) {
            None => return groups,
            Some(num) => *num
        };

        if diff_acc + element > max {
            diff_acc = 0;
            groups += 1;
        }
        diff_acc += element;
    }
    groups
}

fn permute(rng: &mut oorandom::Rand32, max: u32, scratch: &mut [u32]) {
    let which = rng.rand_range(0..(scratch.len() as u32));
    let which_ref = &mut scratch[which as usize];

    if *which_ref == 0 {
        *which_ref = max
    } else {
        *which_ref -= 1;
    }
}

fn round_up(num: u32, multiple: u32) -> u32 {
    ((num + multiple - 1) / multiple) * multiple
}
