use std::io::{self, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom; // this brings choose_multiple method into scope for types that implement it

fn main() -> io::Result<()> {
    let words = load_words("words.txt")?;
    let game_words = select_random_words(&words, 20);

    let mut correct_words = 0;
    let mut total_chars = 0;

    for word in game_words {
        print!("Type this word: {}\nYour input: ", word);
        io::stdout().flush()?; // why flush here? - print is bufferd for flush to gaurantee its out immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == word {
            println!("Correct!");
            correct_words += 1;
        } else {
            println!("Incorrect. The correct word was: {}", word);
        }

        total_chars += word.len();

    }

    println!("\nGame Over!");
    println!("Correct words: {}/20", correct_words);
    println!("Accuracy: {}%", (correct_words as f32 / 20.0) * 100.0);
    println!("Total characters: {}", total_chars);

    Ok(())
}

// write main, then extract reusable bits, or common functionality that could be reused in the future

fn load_words(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file); // the documentation of new says inner R, where R: Read. What does that mean?
    Ok(reader.lines().filter_map(Result::ok).collect()) // why filter_map call here? 
}

fn select_random_words(words: &[String], count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words.choose_multiple(&mut rng, count).cloned().collect()
}