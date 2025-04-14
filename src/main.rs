mod word;

fn main() {
    let word = word::Word::new("barro".to_string());
    println!("Word: {}", word.get_word());
    println!("Try: {:?}", word.attempt("baror".to_string()));
}
