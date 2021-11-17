fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

struct Language {
    name: String,
    translations: Vec<(usize, usize)> // (cost, translation)
}

struct Translator {
    c: usize,
    langs: Vec<usize>
}

fn main() {
    let t = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let n = stdin_line().trim().parse::<usize>().unwrap();

        let mut languages = Vec::new();
        for _ in 0..n {
            let name = stdin_line().trim().to_owned();
            languages.push(Language {
                name,
                translations: Vec::new()
            });
        }

        languages.sort_unstable_by(|a,b| a.name.cmp(&b.name));

        let p = stdin_line().trim().parse::<usize>().unwrap();
        let mut translators = Vec::new();
        for _ in 0..p {
            let line = stdin_line();
            let mut split = line.split_whitespace();

            let m = split.next().unwrap().parse::<usize>().unwrap();
            let c = split.next().unwrap().parse::<usize>().unwrap();

            let mut langs = Vec::with_capacity(m);
            for _ in 0..m {
                let language = split.next().unwrap();
                let index = languages.binary_search_by_key(&language, |lang| lang.name.as_str()).unwrap();
                langs.push(index);
            }

            for lang_i in &langs {
                for lang_j in &langs {
                    if lang_i == lang_j { continue; }

                    languages[*lang_i].translations.push((c, *lang_j));
                    languages[*lang_j].translations.push((c, *lang_i));
                }
            }

            translators.push(Translator {
                c,
                langs
            });
        }

        let line = stdin_line();
        let mut split = line.split_whitespace();

        let lang1 = {
            let language = split.next().unwrap();
            let index = languages.binary_search_by_key(&language, |lang| lang.name.as_str()).unwrap();
            index
        };
        let lang2 = {
            let language = split.next().unwrap();
            let index = languages.binary_search_by_key(&language, |lang| lang.name.as_str()).unwrap();
            index
        };

        let mut stack: Vec<(usize, usize)> = Vec::new(); // (lang_index, inner_index)
        stack.push((lang1, 0));

        let mut best = (0, Vec::new()); // (cost, sequence)
        let mut cost = 0;

        while stack.is_empty() {

            let last_lang = stack.last().unwrap().0;

            if last_lang == lang2 && cost > best.0 {
                let sequence = stack.iter().map(|(lang_index, _)| lang_index).collect();
                best = (cost, sequence);
            }

            let last = stack.last_mut().unwrap();
            let lang = &languages[last.0];

            if last.1 < lang.translations.len() {
                last.1 += 1;
                let next = &lang.translations[last.1];

                drop(last);

                if stack.iter().position(|k| k.0 == next.1).is_some() {
                    continue;
                }

                cost += next.0;
                stack.push((next.1, 0));
            } else {

                drop(last);

                let pop = stack.pop().unwrap();
                cost -= languages[pop.0].translations[pop.1 - 1].0;
            }
        }
/*
        for a in best.1 {
            print!("{} ", languages[*a].name);
        }
        println!()*/
    }
}
