use std::io;

fn main() {
    loop {
        println!("Write a word: ");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        let mut input = String::from(input.trim());

        let first_symbol = match input.chars().next() {
            Some(result) => result,
            None => {
                println!("Invalid input. Try again.");
                continue
            }
        };

        let is_letter_vowel= match is_vowel(first_symbol) {
            Some(result) => result,
            None => continue,
        };

        input.push_str("-");

        if is_letter_vowel {
            input.push_str("hay");
        } else {
            input.remove(0);
            input.push(first_symbol);
            input.push_str("ay")
        };

        println!("Output: {}", input);
    }
}

fn is_vowel(letter: char) -> Option<bool> {
    if let Some(c) = letter.to_lowercase().next() {
        Some(c == 'e' || c == 'y' || c == 'u' || c == 'i' || c == 'o' || c == 'a')
    } else {
        None
    }
}