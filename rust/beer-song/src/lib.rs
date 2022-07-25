use std::ops::Add;

pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }

    if n == 1 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    }

    let text: &str = &"[x] bottles of beer on the wall, [x] bottles of beer.\nTake one down and pass it around, [x-1] bottle[s-1] of beer on the wall.\n"
                        .replace("[x]", &n.to_string()).replace("[x-1]", &(n-1).to_string());

    if n-1 == 1 {
        return String::new().add(text).replace("[s-1]", "")
    }

    String::new().add(text).replace("[s-1]", "s")
}

pub fn sing(start: u32, end: u32) -> String {
    let mut lyrics = String::new();

    for n in (end..=start).rev() {
        lyrics.push_str(&verse(n));

        if n > end {
            lyrics.push_str("\n");
        }
    }

    lyrics
}
