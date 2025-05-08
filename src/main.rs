use rand::Rng;

const VOW: [&str; 5] = ["a", "e", "i", "o", "u"];
const CON: [&str; 22] = [
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n",
    "p", "q", "r", "s", "t", "v", "w", "x", "y", "z", "th"
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

fn get_rnd_syl() -> String {
    let template = get_rnd_num(3);
    match template {
        0 => format!("{}{}", get_rnd_vow(), get_rnd_con()),
        1 => format!("{}{}", get_rnd_con(), get_rnd_vow()),
        2 => format!("{}{}{}", get_rnd_con(), get_rnd_vow(), get_rnd_con()),
        _ => todo!(),
    }
}

fn get_rnd_word() -> String {
    let mut result: String = "".to_string();
    let syl_cnt = get_rnd_num(3) + 1;
    for _i in 0 .. syl_cnt {
        result += &get_rnd_syl();
    }
    result
}


fn main() {
    for _i in 1 .. 10 {
        println!("{}", get_rnd_word());
    }
}
