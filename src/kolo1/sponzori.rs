// Poznámky k odevzdání:
//  *omlouvám se jestli toto není vhodné místo pro poznámky, nic vhodnějšího jsem nenašel/nevymyslel
//  toto je napsáno v Rustu, koncovka je .txt protože odevzdávací systém nepřijímá .rs soubory, ale podle 'https://fiks.fit.cvut.cz/p/faq#v-jak%C3%A9m-programovac%C3%ADm-jazyku-m%C5%AF%C5%BEu-fiks-d%C4%9Blat' je to ok, smutný Ferris
//  pro kompilaci programu použijte 'rustc ./sponzori_rust.txt'
//  užitečný one-liner pro běžení se vstupními daty je 'rustc ./sponzori_rust.txt -o sponzori && cat $VSTUPNI_DATA | ./sponzori'

fn stdin_line() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string).unwrap();

    string
}

#[derive(Clone, Debug)]
struct Zvire {
    jmeno: String,
    pocet_sponzoru: usize,
    zabrano: bool,
}

#[derive(Clone)]
struct Majitel {
    jmeno: String,
    platna_zvirata: Vec<usize>,
}

#[allow(non_snake_case)]
fn main() {
    // čtení vstupu a příprava struktur
    let line = stdin_line();
    let mut split = line.split_whitespace();

    let N = split.next().unwrap().parse::<usize>().unwrap();
    let M = split.next().unwrap().parse::<usize>().unwrap();

    let mut zvirata = vec![
        Zvire {
            jmeno: String::new(),
            pocet_sponzoru: 0,
            zabrano: false
        };
        N
    ];

    let mut sponzori = vec![
        Majitel {
            jmeno: String::new(),
            platna_zvirata: Vec::new(),
        };
        M
    ];

    // O(N)
    for _ in 0..N {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let I = split.next().unwrap().parse::<usize>().unwrap();
        let jmeno = split.next().unwrap();

        zvirata[I].jmeno = jmeno.to_owned();
    }

    // O(N * Nz) - Nz je počet zvířat, předpokládejme že má zanedbatelně nízké maximum
    for i in 0..M {
        let line = stdin_line();
        let mut split = line.split_whitespace();

        let jmeno = split.next().unwrap();
        let zvirata_n = split.next().unwrap().parse::<usize>().unwrap();

        let sponzor = &mut sponzori[i];
        sponzor.platna_zvirata.reserve_exact(zvirata_n);

        for _ in 0..zvirata_n {
            let zvire_n = split.next().unwrap().parse::<usize>().unwrap();
            zvirata[zvire_n].pocet_sponzoru += 1;
            sponzor.platna_zvirata.push(zvire_n);
        }

        sponzor.jmeno = jmeno.to_owned();
    }

    // přidělování sponzorů

    // O(N * log(N))
    sponzori.sort_unstable_by_key(|k| k.platna_zvirata.len());

    let mut prirazene: Vec<(usize, usize)> /* (zvire, sponzor) */ = Vec::with_capacity(M);

    // O(N * (ZN * log(Nz) + Nz)
    for (sponzor_i, sponzor) in sponzori.iter_mut().enumerate() {
        // O(ZN * log(Nz))
        sponzor
            .platna_zvirata
            .sort_unstable_by_key(|k| zvirata[*k].pocet_sponzoru);

        // O(Nz)
        for zvire_i in &sponzor.platna_zvirata {
            let zvire = &mut zvirata[*zvire_i];

            if zvire.zabrano == false {
                prirazene.push((*zvire_i, sponzor_i));
                zvire.zabrano = true;

                break;
            }
        }
    }

    // O(N * log(N))
    prirazene.sort_unstable_by_key(|k| &zvirata[k.0].jmeno);

    if prirazene.len() < N {
        println!("Ne");
    } else {
        println!("Ano");
    }

    // O(N)
    for (zvire_i, majitel_i) in prirazene {
        println!("{} {}", zvirata[zvire_i].jmeno, sponzori[majitel_i].jmeno);
    }
}
