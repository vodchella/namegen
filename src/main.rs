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

fn get_rnd_syl() -> Vec<&'static str> {
    let template = get_rnd_num(3);
    let mut symbols: Vec<&str> = Vec::new();
    match template {
        0 => symbols.extend([get_rnd_vow(), get_rnd_con()]),
        1 => symbols.extend([get_rnd_con(), get_rnd_vow()]),
        2 => symbols.extend([get_rnd_con(), get_rnd_vow(), get_rnd_con()]),
        _ => todo!(),
    }
    symbols
}

fn get_rnd_word() -> Vec<&'static str> {
    let mut word: Vec<&str> = Vec::new();
    let syl_cnt = get_rnd_num(3) + 1;
    for _i in 0 .. syl_cnt {
        word.append(&mut get_rnd_syl());
    }
    word
}


fn main() {
    for _i in 1 .. 10 {
        println!("{}", get_rnd_word().join(""));
    }
}
