fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word: &str = "";
    let mut max_count: usize = 0;

    // outer loop: pick a word
    for i in 0..words.len() {
        let mut count: usize = 0;

        // inner loop: count how many times it appears
        for j in 0..words.len() {
            if words[i] == words[j] {
                count += 1;
            }
        }

        // update max if needed
        if count > max_count {
            max_count = count;
            max_word = words[i];
        }
    }

    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";

    let (word, count) = most_frequent_word(text);

    println!("Most frequent word: \"{}\" ({} times)", word, count);
}