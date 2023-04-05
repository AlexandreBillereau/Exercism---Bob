pub fn reply(message: &str) -> &str {
    let mut vec_chars: Vec<char> = message.trim().chars().collect();

    let only_letter  = |vec: &mut Vec<char>| vec.retain(|e| e.is_alphabetic());
    let is_yell      = |vec: &Vec<char>| vec.iter().all(|e| e.is_uppercase()) && !vec.is_empty();

    if is_question(&vec_chars) {
        only_letter(&mut vec_chars);
        if !is_yell(&vec_chars) {
            return "Sure.";
        }
        return "Calm down, I know what I'm doing!";
    }

    if vec_chars.is_empty() {
        return "Fine. Be that way!";
    }
    
    only_letter(&mut vec_chars);
    if is_yell(&vec_chars) {
        return "Whoa, chill out!";
    }

    return "Whatever.";
}

fn is_question(vec: &Vec<char>) -> bool {
    match vec.last() {
        None => false,
        Some(n) => *n == '?',
    }
}
