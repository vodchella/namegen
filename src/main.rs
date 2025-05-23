use rand::Rng;

const VOW: [&str; 5] = ["a", "e", "i", "o", "u"];
const CON: [&str; 28] = [
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n",
    "p", "q", "r", "s", "t", "v", "w", "x", "y", "z",
    "th", "ch", "sh", "ph", "wh", "gh", "qu"
];


fn get_rnd_num(len: usize) -> usize {
    rand::thread_rng().gen_range(0..len)
}

fn get_rnd_vow() -> &'static str {
    VOW[get_rnd_num(VOW.len())]
}

fn get_rnd_con() -> &'static str {
    CON[get_rnd_num(CON.len())]
}

fn is_vow(sym: &str) -> bool {
    VOW.contains(&sym)
}

fn get_rnd_syl() -> Vec<&'static str> {
    /*
     *  0: V
     *  1: V C
     *  2: C V
     *  3: C V C
     */
    let template = get_rnd_num(3);
    let mut symbols: Vec<&str> = Vec::new();
    match template {
        0 => symbols.extend([get_rnd_vow()]),
        1 => symbols.extend([get_rnd_vow(), get_rnd_con()]),
        2 => symbols.extend([get_rnd_con(), get_rnd_vow()]),
        3 => symbols.extend([get_rnd_con(), get_rnd_vow(), get_rnd_con()]),
        _ => (),
    }
    symbols
}

fn get_rnd_word() -> Vec<&'static str> {
    let mut word: Vec<&str> = Vec::new();
    let syl_cnt = get_rnd_num(3) + 2;
    for _i in 0 .. syl_cnt {
        word.append(&mut get_rnd_syl());
    }
    word
}

fn get_mutated_word<'a>(word: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut cloned = (*word).clone();
    let i = get_rnd_num(cloned.len());
    loop {
        let new_sym = match is_vow(cloned[i]) {
            true  => get_rnd_vow(),
            false => get_rnd_con(),
        };
        if new_sym != cloned[i] {
            cloned[i] = new_sym;
            break;
        }
    }
    cloned
}


fn main() {
    for _i in 0 .. 6 {
        let word = get_rnd_word();
        println!("{}", word.join(""));
        for _j in 0 .. 3 {
            println!("    {}", get_mutated_word(&word).join(""));
        }
    }
}
