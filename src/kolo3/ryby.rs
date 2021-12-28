fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

fn main() {

    let line = stdin_line();
    let mut split = line.split_whitespace();

    let n = split.next().unwrap().parse::<usize>().unwrap();
    let sx = split.next().unwrap().parse::<i32>().unwrap();
    let sy = split.next().unwrap().parse::<i32>().unwrap();

    let mut polygons = Vec::with_capacity(n);
    for _ in 0..n {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let p = split.next().unwrap().parse::<usize>().unwrap();

        let mut vertices = vec![(0, 0); p];
        for i in 0..p {
            let x = split.next().unwrap().parse::<i32>().unwrap();
            let y = split.next().unwrap().parse::<i32>().unwrap();

            vertices[i] = (x, y);
        }

        polygons.push(vertices);
    }

    
    // s = (sx, sy)
    // n = (sy, -sx)
    // p: sy⋅x + -sx⋅y = 0
    //
    // Q: [m, n]
    // v(Q, p) = (sy⋅m - sx⋅n) / |s|  ; zajima nas jen relativni vzdalenost, deleni tedy muzeme vypustit
    // v(Q, p) = (sy⋅m - sx⋅n)
    
    let mut projected_extremes = Vec::new();

    // let mut total_distance_max = 0;
    for vertices in &polygons {
        let mut min = i32::MAX;
        let mut max = 0;
        for &(m, n) in vertices {
            let v = sy*m - sx*n;
            min = min.min(v);
            max = max.max(v);
        }
        // total_distance_max = total_distance_max.max(max);

        // projected_shapes.push((min, max));
        projected_extremes.push((min, true));
        projected_extremes.push((max, false));
    }

    projected_extremes.sort_unstable();

    let mut max_overlap = 0;
    let mut overlapping_count = 0;
    for (_, is_start) in projected_extremes {
        match is_start {
            true => overlapping_count += 1,
            false => overlapping_count -= 1,
        }
        max_overlap = max_overlap.max(overlapping_count);
    }
    
    println!("{}", max_overlap);

    // for (min, max) in projected_shapes {
    //     println!("{}..{}", min, max);
    // }
    
    rasterize(polygons);
}

// tohle je scanline rasterizer na ilustraci vstupu, neni soucasti reseni ale jen tu tak je
fn rasterize(polygons: Vec<Vec<(i32, i32)>>) {
    
    // scale oddeleny pro osy aby to v terminalu nebylo protahle
    let scale_x = std::env::args().nth(1).map(|s| s.parse::<i32>().unwrap()).unwrap_or(8);
    let scale_y = std::env::args().nth(2).map(|s| s.parse::<i32>().unwrap()).unwrap_or(scale_x/2);

    let mut total_min_x = i32::MAX;
    let mut total_min_y = i32::MAX;
    let mut total_max_x = 0;
    let mut total_max_y = 0;

    // ja vim ze jsem tohle vsechno mohl delat pri cteni vstupu, ale ten jsem chtel nechat bez tohohle sumu
    let mut rescaled_polygons = Vec::with_capacity(polygons.len()); 
    for mut vertices in polygons {

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for (x, y) in &mut vertices {
            *x *= scale_x;
            *y *= scale_y;

            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }

        total_min_x = total_min_x.min(min_x);
        total_min_y = total_min_y.min(min_y);
        total_max_x = total_max_x.max(max_x);
        total_max_y = total_max_y.max(max_y);

        rescaled_polygons.push((vertices, (min_x, min_y), (max_x, max_y)))
    }

    let w = (total_max_x - total_min_x + 1) as usize;
    let h = (total_max_y - total_min_y + 1) as usize;

    let mut canvas = vec![0u8; w * h];

    // even though the polygon is convex, this algorithm emits 3 intersections in certain edge cases, the output is not affected
    let mut row_pts = [0; 3];

    for (polygon_i, (vertices, min, max)) in rescaled_polygons.iter().enumerate() {
        let polygon_id = (polygon_i % 255 + 1) as u8;

        let mut buffer_y = (min.1 - total_min_y) as usize;

        for row_y in min.1..=max.1 {
            let mut row_pts_count = 0;

            // if vertices.len() > row_pts.len() {
            //     row_pts.resize(vertices.len(), 0);
            // }

            let mut j = vertices.len() - 1;
            let mut prev_dir = 0;
            for i in 0..vertices.len() {
                let pi = vertices[i];
                let pj = vertices[j];
                
                j = i;
                
                let side_i = pi.1 > row_y;
                let side_j = pj.1 > row_y;

                // if row_y == 15 && polygon_i == 2 {
                //     println!("{} ({}, {}) ({}, {})", prev_dir, pi.0 / 16, pi.1 / 16, pj.0 / 16, pj.1 / 16);
                // }
                
                if side_i != side_j || pi.1 == row_y || pj.1 == row_y {
                    let s = (pi.0 - pj.0, pi.1 - pj.1);
                    
                    let cur_dir = s.1.signum();

                    if s.1 == 0 || prev_dir == cur_dir {
                        continue;
                    }

                    prev_dir = cur_dir;

                    let n = (s.1, -s.0);
                    let x = (n.0 * pi.0 + n.1 * pi.1 - n.1 * row_y) / n.0;

                    row_pts[row_pts_count] = x - total_min_x;
                    row_pts_count += 1;

                    // if row_pts_count == 3 {
                    //     dbg!(&row_pts[0..row_pts_count]);
                    //     dbg!(polygon_i);
                    //     dbg!(pi);
                    //     dbg!(pj);
                    //     dbg!(row_y);
                    //     canvas[row_y as usize*w + 5] = 1;
                    // }
                }
            }

            let a = row_pts[0];
            let b = row_pts[1];
            if a > b {
                row_pts[0] = b;
                row_pts[1] = a;
            }
            // row_pts[0..row_pts_count].sort_unstable();

            let mut i = 0;
            while i < row_pts_count - 1 {
                for pixel_x in row_pts[i]..=row_pts[i + 1] {
                    canvas[pixel_x as usize + buffer_y * w] = polygon_id;
                }
                i += 2;
            }

            buffer_y += 1;
        }
    }

    let scale_x = scale_x as usize;
    let scale_y = scale_y as usize;
    for y in (0..h).rev() {
        for x in 0..w {
            let chars = ['■', '░', '▒', '▓'];

            let val = match canvas[(x + y * w) as usize] {
                _ if x % scale_x == 0 && y % scale_y == 0 => '●',
                0 => '.',
                n => chars[(n - 1) as usize % chars.len()],
            };
            print!("{}", val);
        }
        // print!("{}", '|');
        println!();
    }
}