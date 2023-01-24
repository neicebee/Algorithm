fn main() {
    let s_words = String::from("little bottle of water");
    let l_words = "Chips! tomato and lettuce";
    let s_f_word = find_first_word(&s_words[..]);
    let l_f_word = find_first_word(l_words);

    println!("{}", s_f_word);
    println!("{}", l_f_word);
}

fn find_first_word(s: &str) -> &str{
    let byte_s = s.as_bytes();

    for (i, &item) in byte_s.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}