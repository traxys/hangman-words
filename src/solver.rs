pub fn match_regex(used_letters: &str, pattern: &str) -> regex::Regex {
    let mut reg = String::from("(?m)^");
    let mut any_letter = String::from("[a-z&&[^");
    for letter_in_patern in used_letters.chars().filter(|c| *c != '_') {
        any_letter.push(letter_in_patern)
    }

    for used_letter in used_letters.chars() {
        if !any_letter.contains(used_letter) {
            any_letter.push(used_letter)
        }
    }
    any_letter.push_str("]]");
    for letter in pattern.chars() {
        if letter == '_' {
            reg.push_str(&any_letter);
        } else {
            reg.push(letter);
        }
    }

    reg.push_str(r"\b");
    regex::Regex::new(&reg).unwrap()
}

pub fn get_matches<'a>(
    word_list: &'a str,
    match_regex: &'a regex::Regex,
) -> regex::Matches<'a, 'a> {
    match_regex.find_iter(word_list)
}
