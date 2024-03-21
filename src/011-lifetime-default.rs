fn first_word_in_string(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}

fn first_word_v2(s: &str) -> &str {
    let space_position = s.bytes().position(|byte| byte == b' ');

    match space_position {
        Some(pos) => &s[0..pos],
        None => s,
    }
}

fn first_word_v3(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn first_word_v4(s: &str) -> &str {
    match s.find(' ') {
        Some(pos) => &s[0..pos],
        None => s,
    }
}

fn first_word_v5(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

fn first_word_v6(s: &str) -> &str {
    if let Some(pos) = s.find(' ') {
        &s[0..pos]
    } else {
        s
    }
}

fn main() {
    let array_of_sentences: [&str; 3] = [
        "The quick brown fox",
        "jumps over the lazy dog",
        "and the dog barks back",
    ];

    for sentence in array_of_sentences.iter() {
        println!(
            "the first word in the sentence [{}] is : {}",
            sentence,
            first_word_in_string(sentence)
        );
    }
}
