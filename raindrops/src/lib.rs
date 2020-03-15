pub fn raindrops(n: u32) -> String {
    if !has_sound(n) {
        return n.to_string();
    }

    play_sound(n)
}

fn has_sound(n: u32) -> bool {
    n % 3 == 0 || n % 5 == 0 || n % 7 == 0
}

fn play_sound(n: u32) -> String {
    let mut sound = String::from("");

    sound.push_str(&pling(n));
    sound.push_str(&plang(n));
    sound.push_str(&plong(n));

    sound
}

fn pling(n: u32) -> String {
    if n % 3 != 0 {
        return "".to_string();
    }

    "Pling".to_string()
}

fn plang(n: u32) -> String {
    if n % 5 != 0 {
        return "".to_string();
    }

    "Plang".to_string()
}

fn plong(n: u32) -> String {
    if n % 7 != 0 {
        return "".to_string();
    }

    "Plong".to_string()
}
