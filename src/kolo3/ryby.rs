// Poznámky k odevzdání:
//  toto je napsáno v Rustu, koncovka je .txt protože odevzdávací systém nepřijímá .rs soubory
//  pro kompilaci programu použijte 'rustc ./prohlidkove-okruhy_rust.txt'
//  užitečný one-liner pro běžení se vstupními daty je 'rustc ./prohlidkove-okruhy_rust.txt -o sponzori && ./sponzori < $VSTUPNI_DATA'

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();

    // tady se krásně parsuje, prý čím víc .unwrap(), tím líp
    let n = split.next().unwrap().parse::<usize>().unwrap();
    let sx = split.next().unwrap().parse::<i32>().unwrap();
    let sy = split.next().unwrap().parse::<i32>().unwrap();

    let mut polygons = Vec::with_capacity(n);
    for _ in 0..n {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let p = split.next().unwrap().parse::<usize>().unwrap();

        let mut vertices = Vec::with_capacity(p);
        for _ in 0..p {
            let x = split.next().unwrap().parse::<i32>().unwrap();
            let y = split.next().unwrap().parse::<i32>().unwrap();

            vertices.push((x, y));
        }

        polygons.push(vertices);
    }

    let mut projected_extremes = Vec::with_capacity(polygons.len() * 2);

    // spočítá "nějakou" vzdálenost od přímky rovnoběžné s vektorem (sx, sy) procházející nulou
    // její velikost není důležitá, jen slouží k porovnání
    for vertices in &polygons {
        let mut min = i32::MAX;
        let mut max = 0;
        for &(m, n) in vertices {
            // toto je ta magická omáčka co počítá vzdálenost
            let v = sy * m - sx * n;
            min = min.min(v);
            max = max.max(v);
        }

        // true znamená, že interval uvádí, false ho uzavírá
        projected_extremes.push((min, false));
        projected_extremes.push((max, true));
    }

    // rustová implementace sortu pro touply, tedy (i32, bool) nejdříve řadí podle prvního elementu a až poté se rozhoduje podle druhého
    // tedy ve výsledku budou touply se stejným číslem a true první, tím se jakoby docílí aby byly intervaly uzavřené
    projected_extremes.sort_unstable();

    let mut max_overlap = 0;
    let mut cur_overlap_count = 0;
    for (_, is_end) in projected_extremes {
        match is_end {
            false => cur_overlap_count += 1,
            true => cur_overlap_count -= 1,
        }
        max_overlap = max_overlap.max(cur_overlap_count);
    }

    println!("{}", max_overlap);

    // rasterizuje polygony a vypíše je do stdout
    // rasterize(polygons);
}

// tohle je scanline rasterizer na ilustraci vstupu, není součástní řešení ale jen tu tak je abych se mohl vytahovat
fn rasterize(polygons: Vec<Vec<(i32, i32)>>) {
    // scale oddělený pro osy aby se vykompenzoval nerovný poměr stran políček terminálu
    let scale_x = std::env::args()
        .nth(1)
        .map(|s| s.parse::<i32>().unwrap())
        .unwrap_or(8);
    let scale_y = std::env::args()
        .nth(2)
        .map(|s| s.parse::<i32>().unwrap())
        .unwrap_or(scale_x / 2);

    let mut total_min_x = i32::MAX;
    let mut total_min_y = i32::MAX;
    let mut total_max_x = 0;
    let mut total_max_y = 0;

    // scale se aplikuje až tady místo ve čtení výstupu jen aby to bylo přehledné
    let mut rescaled_polygons = Vec::with_capacity(polygons.len());
    for mut vertices in polygons {
        assert!(!vertices.is_empty());

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

    // do teď min a max řešily pozici samotných vrcholů, teď je potřeba určit kolik "pixelů" reálně bude
    let w = (total_max_x - total_min_x + 1) as usize;
    let h = (total_max_y - total_min_y + 1) as usize;

    let mut canvas = vec![0u8; w * h];

    // i když jsou polygony konvexní, algoritmus v nekterych pripadech vyplivne jeden průsečík dvakrát
    // nevidím jak to jednoduše spravit jinak než všude dat vyjímky takže tu prostě bude místo na tři
    let mut row_pts = [0; 3];

    for (polygon_i, (vertices, min, max)) in rescaled_polygons.iter().enumerate() {
        let polygon_id = (polygon_i % 255 + 1) as u8;
        let mut buffer_y = (min.1 - total_min_y) as usize;

        // na jádro tohoto algoritmu jsem přepsal nějaký kod z C který ani nefungoval
        for row_y in min.1..=max.1 {
            let mut row_pts_count = 0;
            let mut prev_dir = 0;

            let mut j = vertices.len() - 1;

            for i in 0..vertices.len() {
                let pi = vertices[i];
                let pj = vertices[j];

                j = i;

                let side_i = pi.1 > row_y;
                let side_j = pj.1 > row_y;

                if side_i != side_j || pi.1 == row_y || pj.1 == row_y {
                    let s = (pi.0 - pj.0, pi.1 - pj.1);
                    let cur_dir = s.1.signum();

                    // pokuď je hrana horizontální, nebo byly dvě za sebou ve stejném směru
                    // /  to je aby když nastane ta situace, že dvě hrany tvoří vrchol na řádku, protínáme jen jednu aby nevznikly dva stejné průsečíky
                    // |
                    //
                    // ^ v případě, že máme takto ostrý vrchol, chceme jeden průsečík dvakrát aby obarvil jen ten pixel přímo pod vrcholem
                    if s.1 == 0 || prev_dir == cur_dir {
                        // hodnota 0 se normálně nevyskytuje mimo horizontální hrany, znamená to tedy, že příští nehorizontální hrana přes tohle přejde
                        prev_dir = 0;
                        continue;
                    }

                    prev_dir = cur_dir;

                    // vyrobeno z obecné rovnice přímmky ax+by+c=0
                    let x = (s.1 * pi.0 + -s.0 * pi.1 + s.0 * row_y) / s.1;

                    row_pts[row_pts_count] = x - total_min_x;
                    row_pts_count += 1;
                }
            }

            let a = row_pts[0];
            let b = row_pts[1];
            if a > b {
                row_pts[0] = b;
                row_pts[1] = a;
            }

            // vyplňuje pole mezi dvojicemi prusečíků, zapisuje číslo polygonu, to je přeměněno na znak později
            let mut i = 0;
            while i + 1 < row_pts_count {
                for pixel_x in row_pts[i]..=row_pts[i + 1] {
                    canvas[pixel_x as usize + buffer_y * w] = polygon_id;
                }
                i += 2;
            }

            buffer_y += 1;
        }
    }

    // ošklivé printítko výsledného pole, pro různé polygony printuje různé znaky a na místech, kde by bez scalovani byly celé souřadnice udělá puntíky
    // printuje do stderr aby v tom nebyl bordel
    // třeba takto
    // ●▓▓▓●...●░░░●
    // ▓▓▓▓▓...░░░░░
    // ●▓▓▓●...●░░░●
    // .............
    // ●░░░●...●▒▒▒●
    // ░░░░░...▒▒▒▒▒
    // ●░░░●...●▒▒▒●
    let scale_x = scale_x as usize;
    let scale_y = scale_y as usize;
    for y in (0..h).rev() {
        for x in 0..w {
            let chars = ['░', '▒', '▓'];

            let val = match canvas[(x + y * w) as usize] {
                _ if x % scale_x == 0 && y % scale_y == 0 && scale_x != 1 && scale_y != 1 => '●',
                // prázdný pixel
                0 => '.',
                n => chars[(n - 1) as usize % chars.len()],
            };
            eprint!("{}", val);
        }
        eprintln!();
    }
}
