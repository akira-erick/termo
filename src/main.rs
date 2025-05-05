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
        if resp.is_err() {
            println!("Invalid word. Please enter a valid 5-letter word.");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            attempt = input.trim().to_string();
            continue;
        }
        let resp = resp.unwrap();
        if resp == ['G', 'G', 'G', 'G', 'G'] {
            println!(
                "Congratulations! You guessed the word: {:?}",
                word.get_word()
            );
            println!("It took you {} attempts.", times);
            break;
        } else {
            let str: String = resp
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("Attempt: {:?}", str);
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
