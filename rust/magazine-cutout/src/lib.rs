use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words_count: HashMap<&str, i32> = HashMap::new();
    let mut possible: bool = true;

    for word in magazine{
        let w = words_count.entry(word).or_insert(0);
        *w += 1;
    }

    for word in note{
        let w = words_count.entry(word).or_insert(0);
        *w -= 1;

        if *w < 0 {
            possible = false;
            break
        }
    }

    possible
}
