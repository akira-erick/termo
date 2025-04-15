use std::io;

mod word;

fn main() {
    let word = word::Word::random();

    let mut input = String::new();

    println!("Enter a 5-letter word:");
    println!("or exit to exit...");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut attempt = input.trim().to_string();

    let mut times = 1;
    
    while attempt != "exit" {
        let resp = word.attempt(attempt.clone());
        if resp == ['G', 'G', 'G', 'G', 'G'] {
            println!("Congratulations! You guessed the word: {:?}", word.get_word());
            println!("It took you {} attempts.", times);
            break;
        } else {
            println!("Attempt: {:?}", resp);
        }
        input.clear();
        println!("Enter a 5-letter word:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        attempt = input.trim().to_string();
        times += 1;
    }
}
