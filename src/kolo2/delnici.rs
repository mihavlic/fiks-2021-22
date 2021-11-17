use std::{collections::BinaryHeap, rc::Rc};

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[derive(Debug)]
struct LanguageVertex {
    name: String,
    explored: bool,
    translation_edges: Vec<(u32, usize)>, // (cost, opposing LanguageVertex) it is not needed which translator provides the translation
}

fn main() {
    let t = stdin_line().trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let n = stdin_line().trim().parse::<usize>().unwrap();

        let mut languages = Vec::with_capacity(n);
        for _ in 0..n {
            let name = stdin_line().trim().to_owned();
            languages.push(LanguageVertex {
                name,
                explored: false,
                translation_edges: Vec::new(),
            });
        }

        languages.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        let p = stdin_line().trim().parse::<usize>().unwrap();
        for _ in 0..p {
            let line = stdin_line();
            let mut split = line.split_whitespace();

            let m = split.next().unwrap().parse::<usize>().unwrap();
            let c = split.next().unwrap().parse::<u32>().unwrap();

            let mut langs = Vec::with_capacity(m);
            for _ in 0..m {
                let language = split.next().unwrap();
                let index = languages
                    .binary_search_by_key(&language, |lang| lang.name.as_str())
                    .unwrap();
                langs.push(index);
            }

            for lang_i in &langs {
                for lang_j in &langs {
                    // do not make an edge from itself to itself
                    if lang_i == lang_j {
                        continue;
                    }

                    let mut push_translation_dedup =
                        |lang_index: usize, (cost, translation_lang_index): (u32, usize)| {
                            for (prev_cost, prev_translation_lang) in
                                &mut languages[lang_index].translation_edges
                            {
                                // a translation already exists, set the minimum cost
                                if *prev_translation_lang == translation_lang_index {
                                    *prev_cost = (*prev_cost).min(cost);
                                    return;
                                }
                            }
                            // the translation is not here, push it
                            languages[lang_index]
                                .translation_edges
                                .push((cost, translation_lang_index));
                        };

                    // make edges between the two languages in both directions
                    push_translation_dedup(*lang_i, (c, *lang_j));
                    push_translation_dedup(*lang_j, (c, *lang_i));
                }
            }
        }

        let line = stdin_line();
        let mut split = line.split_whitespace();

        let start = {
            let language = split.next().unwrap();
            let index = languages
                .binary_search_by_key(&language, |lang| lang.name.as_str())
                .unwrap();
            index
        };
        let target = {
            let language = split.next().unwrap();
            let index = languages
                .binary_search_by_key(&language, |lang| lang.name.as_str())
                .unwrap();
            index
        };

        #[derive(Clone)]
        struct PathLink(usize, Option<Rc<Self>>);

        struct Node {
            cost: u32,
            lang_index: usize,
            prev: PathLink,
        }

        // a bunch of custom derives so that the BinaryHeap works correctly since we want to sort only by the cost and the smallest cost must be at the top of the heap
        impl PartialEq for Node {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost
            }
        }

        impl Eq for Node {}

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.cost.partial_cmp(&other.cost).map(|ord| ord.reverse())
            }
        }

        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.cost.cmp(&other.cost).reverse()
            }
        }

        let mut heap = BinaryHeap::new();
        heap.push(Node {
            cost: 0,
            lang_index: start,
            prev: PathLink(start, None),
        });

        let mut last_node = None;
        while !heap.is_empty() {
            let pop = heap.pop().unwrap();

            languages[pop.lang_index].explored = true;

            if pop.lang_index == target {
                last_node = Some(pop);
                break;
            }

            for &(edge_cost, translation_index) in &languages[pop.lang_index].translation_edges {
                let translation = &languages[translation_index];

                // node was already explored
                if translation.explored == true {
                    continue;
                }

                heap.push(Node {
                    cost: pop.cost + edge_cost,
                    lang_index: translation_index,
                    prev: PathLink(pop.lang_index, Some(Rc::new(pop.prev.clone()))),
                })
            }
        }

        if let Some(node) = last_node {
            let mut final_translations = vec![target, node.prev.0];
            let mut path = &node.prev.1;

            while let Some(prev) = path {
                if prev.1.is_some() {
                    final_translations.push(prev.0);
                }
                path = &prev.1;
            }

            println!("To nas bude stat {},-.", node.cost);
            println!("Pocet prekladu: {}.", final_translations.len() - 1);
            for &translation in final_translations.iter().rev() {
                println!("{}", languages[translation].name);
            }
        } else {
            println!("Takove prekladatele nemame.");
        }
    }
}
