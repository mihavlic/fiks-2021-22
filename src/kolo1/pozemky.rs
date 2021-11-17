fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

#[allow(non_snake_case)]
fn main() {
    let line = stdin_line();

    let T = line.trim().parse::<usize>().unwrap();

    for _ in 0..T {
        let line = stdin_line();
        let T = line.trim().parse::<usize>().unwrap();

        let mut celkova_plocha = 0;

        for _ in 0..T {
            let line = stdin_line();
            let mut split = line.split_whitespace();

            let _jmeno = split.next().unwrap();
            let n = split.next().unwrap().parse::<usize>().unwrap();
            let plocha = split.next().unwrap().parse::<usize>().unwrap();

            celkova_plocha += plocha * n;
        }

        println!("{}", celkova_plocha);
    }
}
