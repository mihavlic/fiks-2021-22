fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();

    let n = split.next().unwrap().parse::<usize>().unwrap();
    let _sx = split.next().unwrap().parse::<i32>().unwrap();
    let _sy = split.next().unwrap().parse::<i32>().unwrap();

    let mut polygons = Vec::with_capacity(n);

    let mut total_min_x = i16::MAX;
    let mut total_min_y = i16::MAX;
    let mut total_max_x = 0;
    let mut total_max_y = 0;

    for _ in 0..n {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let p = split.next().unwrap().parse::<usize>().unwrap();

        let mut min_x = i16::MAX;
        let mut min_y = i16::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        let mut polygon_points = vec![(0, 0); p];

        for i in 0..p {
            let x = 8 * TryInto::<i16>::try_into(split.next().unwrap().parse::<u32>().unwrap())
                .unwrap();
            let y = 8 * TryInto::<i16>::try_into(split.next().unwrap().parse::<u32>().unwrap())
                .unwrap();

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);

            polygon_points[i] = (x, y);
        }

        total_min_x = total_min_x.min(min_x);
        total_min_y = total_min_y.min(min_y);
        total_max_x = total_max_x.max(max_x);
        total_max_y = total_max_y.max(max_y);

        polygons.push((polygon_points, (min_x, min_y), (max_x, max_y)));
    }

    let w = (total_min_x..=total_max_x).len() as i16;
    let h = (total_min_y..=total_max_y).len() as i16;

    println!("{} {}", w, h);

    let mut canvas = vec![0u8; (w * (h + 1)) as usize];

    let mut row_pts = vec![0; w as usize];

    for (polygon_i, (vertices, min, max)) in polygons.iter().enumerate() {
        let polygon_id = (polygon_i % 255 + 1) as u8;

        let mut buffer_y = min.1 - total_min_y;
        for row_y in min.1..=max.1 {
            let mut row_pts_count = 0;

            if vertices.len() > row_pts.len() {
                row_pts.resize(vertices.len(), 0);
            }

            let mut j = vertices.len() - 1;
            let mut prev_dir = 0; //(vertices[j-1].1-vertices[j].1).signum();
            for i in 0..vertices.len() {
                let pi = vertices[i];
                let pj = vertices[j];

                // 6 0
                // 0 0
                // 0 6

                // y = 0, 6
                j = i;

                let side_i = pi.1 > row_y;
                let side_j = pj.1 > row_y;

                if side_i != side_j || pi.1 == row_y || pj.1 == row_y
                // || pi.1 <  row_y && pj.1 >= row_y
                // || pj.1 <  row_y && pi.1 >= row_y
                {
                    // let s = P[i] - P[j]
                    // let n = [s.1, -s.0]
                    // p: n.0*x + n.1*y - (n.0*p.0 + n.1*p.1) = 0
                    // y: y - row_y = 0
                    //
                    // x = (n.0*p.0 + n.1*p.1 - n.1*y) / n.0

                    let s = (pi.0 - pj.0, pi.1 - pj.1);

                    let cur_dir = s.1.signum();

                    if s.1 == 0 || prev_dir == cur_dir {
                        continue;
                    }

                    prev_dir = cur_dir;

                    let n = (s.1, -s.0);
                    let y = row_y;
                    let x = (n.0 * pi.0 + n.1 * pi.1 - n.1 * y) / n.0;

                    row_pts[row_pts_count] = x;
                    row_pts_count += 1;
                }
            }

            row_pts[0..row_pts_count].sort_unstable();

            if row_y == total_max_y {
                println!("{} {} {:?}", polygon_i, row_y, &row_pts[..row_pts_count]);
                println!();
            }

            for i in (0..row_pts_count).step_by(2) {
                for pixel_x in row_pts[i]..=row_pts[(i + 1).min(row_pts_count - 1)] {
                    canvas[(pixel_x + buffer_y as i16 * w) as usize] = polygon_id;
                }
            }

            buffer_y += 1;
        }
    }

    for y in (0..h).rev() {
        for x in 0..w {
            let chars = ['■', '░', '▒', '▓'];
            let val = match canvas[(x + y * w) as usize] {
                0 => '.',
                n => chars[(n - 1) as usize % chars.len()],
            };
            print!("{}", val);
        }
        // print!("{}", '|');
        println!();
    }
}
